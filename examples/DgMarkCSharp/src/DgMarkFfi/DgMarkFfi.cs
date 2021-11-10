using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace DgMarkFfiExample
{
    public static class DgMarkFfi
    {
        [StructLayout(LayoutKind.Sequential)]
        struct TextsDescriptor
        {
            public unsafe IntPtr* Texts;

            public int Size;
        }

        [DllImport("../../target/release/libdgmark_ffi", CallingConvention = CallingConvention.Cdecl, EntryPoint = "texts")]
        static extern TextsDescriptor TextsFfi(string input);

        unsafe public static IReadOnlyCollection<string> Texts(string input)
        {
            var textsDescriptor = TextsFfi(input);

            return Enumerable
                .Range(0, textsDescriptor.Size)
                .Select(i => Marshal.PtrToStringUTF8(textsDescriptor.Texts[i]))
                .ToList();
        }
    }
}
