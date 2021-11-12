using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace DgMarkFfiExample
{
    public static class DgMarkFfi
    {
        private const string LibPath = "/home/martin/Projects/dgmark/target/release/libdgmark_ffi.so";

        [StructLayout(LayoutKind.Sequential)]
        struct TextsDescriptor
        {
            public unsafe IntPtr* Texts;

            public int Size;
        }

        [DllImport(LibPath, CallingConvention = CallingConvention.Winapi, EntryPoint = "texts")]
        static extern TextsDescriptor TextsFfi(string input);

        [DllImport(LibPath, CallingConvention = CallingConvention.Winapi, EntryPoint = "dealloc_texts")]
        static extern void DeallocTexts(TextsDescriptor textsDescriptor);

        unsafe public static IReadOnlyCollection<string> Texts(string input)
        {
            var textsDescriptor = TextsFfi(input);

            var texts = Enumerable
                .Range(0, textsDescriptor.Size)
                .Select(i => Marshal.PtrToStringUTF8(textsDescriptor.Texts[i]))
                .ToList();

            DeallocTexts(textsDescriptor);

            return texts;
        }
    }
}
