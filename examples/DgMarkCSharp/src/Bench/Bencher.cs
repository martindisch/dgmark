using System.Collections.Generic;
using System.IO;
using BenchmarkDotNet.Attributes;
using DgMarkFfiExample;
using DgMarkWasmExample;
using DgMarkUniversalWasmExample;

namespace Bench
{
    public class Bencher
    {
        const string InputPath = "/home/martin/Projects/dgmark/dgmark/benches/inputs/text_with_everything.md";

        private readonly DgMarkWasm dgMarkWasm;
        private readonly DgMarkUniversalWasm dgMarkUniversalWasm;
        private readonly string input;

        public Bencher()
        {
            dgMarkWasm = new DgMarkWasm();
            dgMarkUniversalWasm = new DgMarkUniversalWasm();
            input = File.ReadAllText(InputPath);
        }

        [Benchmark]
        public IReadOnlyCollection<string> Ffi() => DgMarkFfi.Texts(input);

        [Benchmark]
        public IReadOnlyCollection<string> Wasm() => dgMarkWasm.Texts(input);

        [Benchmark]
        public IReadOnlyCollection<string> WasmUniversal() => dgMarkUniversalWasm.Texts(input);
    }
}
