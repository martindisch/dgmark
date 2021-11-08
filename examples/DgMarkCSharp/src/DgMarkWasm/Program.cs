using System;
using System.Text;
using Wasmtime;

namespace DgMarkWasm
{
    class Program
    {
        static void Main(string[] args)
        {
            var input = @"Hi there! Let's see a productlist:
[[productlist: 1|2|20]]

That went well. How about another one?

[[productlist:20]]

We can do quotes too.

[[quote:This is supposed to be the quote""And this is the source""]]

Pretty cool, right?";

            using var engine = new Engine();
            using var linker = new Linker(engine);
            using var store = new Store(engine);
            using var module = Module.FromFile(
                engine,
                "../../target/wasm32-unknown-unknown/release/dgmark_wasmtime.wasm");
            var instance = linker.Instantiate(store, module);
            var memory = instance.GetMemory(store, "memory");

            var alloc = instance.GetFunction(store, "__alloc");
            var dealloc = instance.GetFunction(store, "__dealloc");
            var texts = instance.GetFunction(store, "texts");

            // Put input into WASM memory
            var utf8Input = Encoding.UTF8.GetBytes(input);
            var offset = (int)alloc.Invoke(store, utf8Input.Length);
            var allocatedSlice = memory.GetSpan(store).Slice(offset);
            utf8Input.AsSpan<byte>().CopyTo(allocatedSlice);

            var resultPointer = (int)texts.Invoke(store, offset, utf8Input.Length);
            var arrayPtr = memory.ReadInt32(store, resultPointer);
            var arrayLength = memory.ReadInt32(store, resultPointer + 4);

            Console.WriteLine($"Pointer at {arrayPtr} of length {arrayLength}");

            var firstPointer = memory.ReadInt32(store, arrayPtr);
            var firstStringPointer = memory.ReadInt32(store, firstPointer);
            var firstLen = memory.ReadInt32(store, firstPointer + 4);
            Console.WriteLine($"{firstStringPointer}, {firstLen}");
            var firstText = memory.ReadString(store, firstStringPointer, firstLen);

            var lastPointer = memory.ReadInt32(store, arrayPtr + (arrayLength - 1) * 4);
            var lastText = memory.ReadString(store, memory.ReadInt32(store, lastPointer), memory.ReadInt32(store, lastPointer + 4));

            Console.WriteLine($"{firstText}, {lastText}");
        }
    }
}
