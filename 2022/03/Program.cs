using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AOC22
{
    internal class Program
    {
        static void Main(string[] args)
        {
            String[] input = loadInput();
            Console.WriteLine("03");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput(){
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input){
            int sum = 0;
            for (int i = 0; i < input.Length; i++)
            {
                List<char> l1 = new List<char>();
                List<char> l2 = new List<char>();

                int compartementLength = input[i].Length / 2;
                l1.AddRange(input[i].Substring(0, compartementLength));
                l2.AddRange(input[i].Substring(compartementLength));

                char common = l1.Intersect(l2).ElementAt(0);
                int num = (int)common;
                if(Char.IsUpper(common)){
                    num = num - 64 + 26;
                }else{
                    num -= 96;
                }
                sum += num;
            }
            return sum.ToString();
        }

        static String part2(String[] input){
            int sum = 0;
            for (int i = 0; i < input.Length - 2; i += 3)
            {
                List<char> l1 = new List<char>();
                List<char> l2 = new List<char>();
                List<char> l3 = new List<char>();

                int compartementLength = input[i].Length / 3;
                l1.AddRange(input[i]);
                l2.AddRange(input[i + 1]);
                l3.AddRange(input[i + 2]);

                char common = l1.Intersect(l2).Intersect(l3).ElementAt(0);
                int num = (int)common;
                if(Char.IsUpper(common)){
                    num = num - 64 + 26;
                }else{
                    num -= 96;
                }
                sum += num;
            }
            return sum.ToString();
        }

    }
}