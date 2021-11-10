using System;
using System.Collections.Generic;
using System.Linq;
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

            var arrayDescriptorOffset = (int)texts.Invoke(store, offset, utf8Input.Length);

            var extractedTexts = ExtractTexts(memory, store, arrayDescriptorOffset);
            foreach (var text in extractedTexts)
            {
                Console.WriteLine($"- {text}");
            }
        }

        static IEnumerable<string> ExtractTexts(Memory memory, Store store, int arrayDescriptorOffset)
        {
            var arrayOffset = memory.ReadInt32(store, arrayDescriptorOffset);
            var arrayLength = memory.ReadInt32(store, arrayDescriptorOffset + 4);

            return Enumerable
                .Range(0, arrayLength)
                .Select(i =>
                {
                    var currentArrayDescriptorOffset = memory.ReadInt32(store, arrayOffset + i * 4);

                    var currentStringOffset = memory.ReadInt32(store, currentArrayDescriptorOffset);
                    var currentStringLength = memory.ReadInt32(store, currentArrayDescriptorOffset + 4);

                    return memory.ReadString(store, currentStringOffset, currentStringLength);
                });
        }
    }
}
