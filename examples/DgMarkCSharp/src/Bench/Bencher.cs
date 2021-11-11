using System.Collections.Generic;
using System.IO;
using BenchmarkDotNet.Attributes;
using DgMarkFfiExample;
using DgMarkWasmExample;

namespace Bench
{
    public class Bencher
    {
        const string InputPath = "/home/martin/Projects/dgmark/dgmark/benches/inputs/text_with_everything.md";

        private readonly DgMarkWasm dgMarkWasm;
        private readonly string input;

        public Bencher()
        {
            dgMarkWasm = new DgMarkWasm();
            input = File.ReadAllText(InputPath);
        }

        [Benchmark]
        public IReadOnlyCollection<string> Ffi() => DgMarkFfi.Texts(input);

        [Benchmark]
        public IReadOnlyCollection<string> Wasm() => dgMarkWasm.Texts(input);
    }
}
