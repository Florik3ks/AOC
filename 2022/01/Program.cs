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

        static String[] loadInput(){
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input){
            int sum = 0;
            int maxSum = 0;
            for (int i = 0; i < input.Length; i++)
            {
                if (input[i] == String.Empty){
                    if(sum > maxSum){
                        maxSum = sum;
                    }
                    sum = 0;
                }
                else{
                    sum += Int32.Parse(input[i]);
                }
            }
            return maxSum.ToString();
        }

        static String part2(String[] input){
            int sum = 0;
            int[] maxSums = new int[3];
            for (int i = 0; i < input.Length; i++)
            {
                if (input[i] == String.Empty){

                    if(sum > maxSums.Min()){
                        maxSums[Array.IndexOf(maxSums, maxSums.Min())] = sum;
                    }
                    sum = 0;
                }
                else{
                    sum += Int32.Parse(input[i]);
                }
            }
            return maxSums.Sum().ToString();
        }

    }
}