using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Wasmtime;

namespace DgMarkWasmExample
{
    public class DgMarkWasm : IDisposable
    {
        private Engine engine;
        private Module module;
        private Linker linker;

        public DgMarkWasm()
        {
            engine = new Engine();
            module = Module.FromFile(
                engine,
                "../../target/wasm32-unknown-unknown/release/dgmark_wasmtime.wasm");
            linker = new Linker(engine);
        }

        public IReadOnlyCollection<string> Texts(string input)
        {
            using var store = new Store(engine);
            var instance = linker.Instantiate(store, module);
            var memory = instance.GetMemory(store, "memory");

            var alloc = instance.GetFunction(store, "__alloc");
            var dealloc = instance.GetFunction(store, "__dealloc");
            var deallocTexts = instance.GetFunction(store, "dealloc_texts");
            var texts = instance.GetFunction(store, "texts");

            var utf8Input = Encoding.UTF8.GetBytes(input);
            var offset = (int)alloc.Invoke(store, utf8Input.Length);
            var allocatedSlice = memory.GetSpan(store).Slice(offset);
            utf8Input.AsSpan<byte>().CopyTo(allocatedSlice);

            var arrayDescriptorOffset = (int)texts.Invoke(store, offset, utf8Input.Length);
            var extractedTexts = ExtractTexts(memory, store, arrayDescriptorOffset);

            deallocTexts.Invoke(store, arrayDescriptorOffset);
            dealloc.Invoke(store, offset, utf8Input.Length);

            return extractedTexts;
        }

        static IReadOnlyCollection<string> ExtractTexts(Memory memory, Store store, int arrayDescriptorOffset)
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
                })
                .ToList();
        }

        public void Dispose()
        {
            engine.Dispose();
            module.Dispose();
            linker.Dispose();
        }
    }
}
