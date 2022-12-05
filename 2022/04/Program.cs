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
            Console.WriteLine("04");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput(){
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input){
            int result = 0;
            foreach (var line in input)
            {
                String[] pair = line.Split(",");

                int first1 = Int32.Parse(pair[0].Split("-")[0]);
                int second1 = Int32.Parse(pair[0].Split("-")[1]);

                int first2 = Int32.Parse(pair[1].Split("-")[0]);
                int second2 = Int32.Parse(pair[1].Split("-")[1]);

                if(first1 >= first2 && second1 <= second2){
                    result++;
                }
                else if(first2 >= first1 && second2 <= second1){
                    result++;
                }
            }
            return result.ToString();
        }

        static String part2(String[] input){
            int result = 0;
            foreach (var line in input)
            {
                String[] pair = line.Split(",");

                int first1 = Int32.Parse(pair[0].Split("-")[0]);
                int second1 = Int32.Parse(pair[0].Split("-")[1]);

                int first2 = Int32.Parse(pair[1].Split("-")[0]);
                int second2 = Int32.Parse(pair[1].Split("-")[1]);

                List<int> one = new List<int>(); 
                List<int> two = new List<int>(); 

                one.AddRange(Enumerable.Range(first1, second1 - first1 + 1));
                two.AddRange(Enumerable.Range(first2, second2 - first2 + 1));

                if(one.Intersect(two).Any()){
                    result++;
                }
            }
            return result.ToString();
        }

    }
}