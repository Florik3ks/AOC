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
            Console.WriteLine("11");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllText("input.txt", System.Text.Encoding.UTF8).Split("\r\n\r\n");
        }

        static String part1(String[] input)
        {
            return compute(GetMonkeys(input), 20);
        }

        static String part2(String[] input)
        {
            List<Monkey> monkeys = GetMonkeys(input);
            int modVal = 1;
            foreach (var m in monkeys)
            {
                modVal *= m.DivisibleByX;
            }
            return compute(monkeys, 10000, true, modVal);
        }

        static List<Monkey> GetMonkeys(String[] input)
        {
            List<Monkey> monkeys = new List<Monkey>();
            foreach (var item in input)
            {
                monkeys.Add(Monkey.CreateMonkeyFromInput(item.Split("\r\n")));
            }
            return monkeys;
        }
        static string compute(List<Monkey> monkeys, int rounds, bool part2 = false, int modVal = 0)
        {
            for (int i = 0; i < rounds; i++)
            {
                foreach (var m in monkeys)
                {
                    if (!part2)
                    {
                        m.TakeTurn((m, n) => monkeys.Find(j => j.Name == m).AddToList(n), n => n / 3);
                    }
                    else
                    {
                        m.TakeTurn((m, n) => monkeys.Find(j => j.Name == m).AddToList(n), n => n % modVal);
                    }
                }
            }
            long max1 = 0;
            long max2 = 0;
            foreach (var m in monkeys)
            {
                if (m.InspectCount > max1) { max2 = max1; max1 = m.InspectCount; }
                else if (m.InspectCount > max2) { max2 = m.InspectCount; }
            }
            return (max1 * max2).ToString();
        }

    }
}