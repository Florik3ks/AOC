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
            Console.WriteLine("10");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            List<int> values = new List<int>();
            int cycles = 0;
            int x = 1;
            foreach (var line in input)
            {
                string cmd = line.Split(" ")[0];
                values.Add(x);
                switch (cmd)
                {
                    case "noop":
                        cycles++;
                        break;
                    case "addx":
                        int val = Int32.Parse(line.Split(" ")[1]);
                        cycles++;
                        values.Add(x);
                        cycles++;
                        x += val;
                        break;
                    default:
                        break;
                }
            }
            int sum = 0;
            for (int i = 20; i < cycles; i += 40)
            {
                int temp = i * values[i - 1];
                sum += temp;
            }
            return sum.ToString();
        }

        static String part2(String[] input)
        {
            List<bool[]> crt = new List<bool[]>();
            for (int i = 0; i < 6; i++)
            {
                crt.Add(new bool[40]);
            }
            int cycles = 0;
            int x = 1;
            foreach (var line in input)
            {
                string cmd = line.Split(" ")[0];
                switch (cmd)
                {
                    case "noop":
                        cycles++;
                        if (Enumerable.Range((x % 40) - 1, 3).Contains((cycles - 1) % 40))
                        {
                            crt[(cycles - 1) / 40][(cycles - 1) % 40] = true;
                        }
                        break;
                    case "addx":
                        int val = Int32.Parse(line.Split(" ")[1]);
                        for (int j = 0; j < 2; j++)
                        {
                            cycles++;
                            if (Enumerable.Range((x % 40) - 1, 3).Contains((cycles - 1) % 40))
                            {
                                crt[(cycles - 1) / 40][(cycles - 1) % 40] = true;
                            }
                        }
                        x += val;
                        break;
                    default:
                        break;
                }
            }
            String result = "";
            for (int i = 0; i < 6; i++)
            {
                for (int j = 0; j < 40; j++)
                {
                    result += crt[i][j] ? "#" : " ";
                }
                result += "\n";
            }
            return result.ToString();
        }

    }
}