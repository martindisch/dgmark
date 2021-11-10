using System.Collections.Generic;
using BenchmarkDotNet.Attributes;
using DgMarkFfiExample;
using DgMarkWasmExample;

namespace Bench
{
    public class Bencher
    {
        private readonly DgMarkWasm dgMarkWasm;
        private readonly string input = @"Hi there! Let's see a productlist:
[[productlist: 1|2|20]]

That went well. How about another one?

[[productlist:20]]

We can do quotes too.

[[quote:This is supposed to be the quote""And this is the source""]]

Pretty cool, right?";

        public Bencher()
        {
            dgMarkWasm = new DgMarkWasm();
        }

        [Benchmark]
        public IReadOnlyCollection<string> Ffi() => DgMarkFfi.Texts(input);

        [Benchmark]
        public IReadOnlyCollection<string> Wasm() => dgMarkWasm.Texts(input);
    }
}
