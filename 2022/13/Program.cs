using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC22
{
    internal class Program
    {
        static void Main(string[] args)
        {
            String[] input = loadInput();
            Console.WriteLine("13");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            String b1;
            String b2;
            for (int i = 0; i < input.Length; i++)
            {
                if (i % 3 == 1)
                {
                    b1 = input[i];
                }
                else if (i % 3 == 2)
                {
                    b2 = input[i];
                }
                else
                {
                    
                }
            }
            return null;
        }

        static String part2(String[] input)
        {

            return null;
        }

    }
}