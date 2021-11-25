using System;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using Wasmtime;

namespace DgMarkUniversalWasmExample
{
    public class DgMarkUniversalWasm : IDisposable
    {
        private const string LibPath = "/home/martin/Projects/dgmark/target/wasm32-unknown-unknown/release/dgmark_universal.wasm";

        private readonly Engine engine;
        private readonly Module module;
        private readonly Linker linker;

        public DgMarkUniversalWasm()
        {
            engine = new Engine();
            module = Module.FromFile(
                engine,
                LibPath);
            linker = new Linker(engine);
        }

        public IReadOnlyCollection<string> Texts(string input)
        {
            using var store = new Store(engine);
            var instance = linker.Instantiate(store, module);
            var memory = instance.GetMemory(store, "memory");

            var alloc = instance.GetFunction(store, "__alloc");
            var dealloc = instance.GetFunction(store, "__dealloc");
            var texts = instance.GetFunction(store, "texts");

            var utf8Input = Encoding.UTF8.GetBytes(input);
            var offset = (int)alloc.Invoke(store, utf8Input.Length);
            var allocatedSlice = memory.GetSpan(store).Slice(offset);
            utf8Input.AsSpan<byte>().CopyTo(allocatedSlice);

            var descriptorOffset = (int)texts.Invoke(store, offset, utf8Input.Length);
            var (stringOffset, length) = ReadDescriptor(memory, store, descriptorOffset);
            var json = memory.ReadString(store, stringOffset, length);

            var extractedTexts = JsonSerializer.Deserialize<IReadOnlyList<string>>(json);

            dealloc.Invoke(store, offset, utf8Input.Length);
            dealloc.Invoke(store, descriptorOffset, 8);
            dealloc.Invoke(store, stringOffset, length);

            return extractedTexts;
        }

        private static (int Offset, int Length) ReadDescriptor(Memory memory, IStore store, int descriptorOffset)
        {
            var stringOffset = memory.ReadInt32(store, descriptorOffset);
            var length = memory.ReadInt32(store, descriptorOffset + 4);

            return (stringOffset, length);
        }

        public void Dispose()
        {
            engine.Dispose();
            module.Dispose();
            linker.Dispose();
        }
    }
}
