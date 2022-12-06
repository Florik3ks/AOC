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
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String solve(String input, int num)
        {
            Queue<char> q = new Queue<char>();

            for (int i = 0; i < input.Length; i++)
            {
                char c = input[i];
                if(i >= num){
                    if (q.Distinct().Count() == num)
                    {
                        return i.ToString();
                    }
                }
                if(q.Count >= num){
                    q.Dequeue();
                }
                q.Enqueue(c);
            }
            return "none?";
        }

        static String part1(String[] input)
        {
            return solve(input[0], 4);
        }

        static String part2(String[] input)
        {
            return solve(input[0], 14);
        }

    }
}