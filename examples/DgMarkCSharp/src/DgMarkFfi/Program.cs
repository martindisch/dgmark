using System;

namespace DgMarkCSharp
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

            var texts = DgMark.Texts(input);

            foreach (var text in texts)
            {
                Console.WriteLine($"- {text}");
            }
        }
    }
}
