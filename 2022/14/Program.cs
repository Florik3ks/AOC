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
            Console.WriteLine("14");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            (int, int)[] relativeSteps = new (int, int)[] { (0, 1), (-1, 1), (1, 1) };

            HashSet<(int, int)> blocks = parseBlocks(input);
            (int, int) sand = (500, 0);
            int lowestY = 0;
            foreach (var b in blocks)
            {
                lowestY = Math.Max(b.Item2, lowestY);
            }
            HashSet<(int, int)> sandBlocks = new HashSet<(int, int)>();
            bool fallingSand = true;
            while (fallingSand)
            {
                (int, int) newSand = sand;
                bool canMove = true;
                while (newSand.Item2 < lowestY && canMove)
                {
                    (int, int) next;
                    bool moved = false;
                    foreach (var i in relativeSteps)
                    {
                        next = (newSand.Item1 + i.Item1, newSand.Item2 + i.Item2);
                        if (!blocks.Contains(next) && !sandBlocks.Contains(next))
                        {
                            newSand = next;
                            moved = true;
                            break;
                        }
                    }
                    if (!moved)
                    {
                        canMove = false;
                        sandBlocks.Add(newSand);
                    }
                }
                if (newSand.Item2 >= lowestY)
                {
                    return sandBlocks.Count().ToString();
                }
            }
            return "?";
        }
        static HashSet<(int, int)> parseBlocks(String[] input)
        {
            HashSet<(int, int)> blocks = new HashSet<(int, int)>();
            foreach (string line in input)
            {
                (int, int) lastPair = (-1, -1);
                (int, int) currentPair = (-1, -1);
                List<(int, int)> pairs = new List<(int, int)>();
                foreach (string pair in line.Split(" -> "))
                {
                    int x = int.Parse(pair.Split(",")[0]);
                    int y = int.Parse(pair.Split(",")[1]);
                    currentPair = (x, y);
                    if (lastPair == (-1, -1))
                    {
                        lastPair = currentPair;
                        continue;
                    }

                    if (lastPair.Item1 != currentPair.Item1)
                    {
                        int min = Math.Min(lastPair.Item1, currentPair.Item1);
                        int max = Math.Max(lastPair.Item1, currentPair.Item1);
                        for (int i = min; i <= max; i++)
                        {
                            blocks.Add((i, currentPair.Item2));
                        }
                    }
                    else if (lastPair.Item2 != currentPair.Item2)
                    {
                        int min = Math.Min(lastPair.Item2, currentPair.Item2);
                        int max = Math.Max(lastPair.Item2, currentPair.Item2);
                        for (int i = min; i <= max; i++)
                        {
                            blocks.Add((currentPair.Item1, i));
                        }
                    }
                    lastPair = currentPair;
                }
            }
            return blocks;
        }
        static String part2(String[] input)
        {
            (int, int)[] relativeSteps = new (int, int)[] { (0, 1), (-1, 1), (1, 1) };

            HashSet<(int, int)> blocks = parseBlocks(input);
            (int, int) sand = (500, 0);
            int lowestY = 0;
            foreach (var b in blocks)
            {
                lowestY = Math.Max(b.Item2, lowestY);
            }
            HashSet<(int, int)> sandBlocks = new HashSet<(int, int)>();
            bool fallingSand = true;
            while (fallingSand)
            {
                (int, int) newSand = sand;
                bool canMove = true;
                while (newSand.Item2 < lowestY + 1 && canMove)
                {
                    (int, int) next;
                    bool moved = false;
                    foreach (var i in relativeSteps)
                    {
                        next = (newSand.Item1 + i.Item1, newSand.Item2 + i.Item2);
                        if (!blocks.Contains(next) && !sandBlocks.Contains(next))
                        {
                            newSand = next;
                            moved = true;
                            break;
                        }
                    }
                    if (!moved)
                    {
                        canMove = false;
                        sandBlocks.Add(newSand);
                    }
                }
                if (newSand.Item2 >= lowestY + 1)
                {
                    sandBlocks.Add(newSand);
                }
                if(sandBlocks.Contains((500, 0))){
                    fallingSand = false;
                }
            }
            print(blocks, sandBlocks, lowestY);
            return sandBlocks.Count().ToString();
        }

        /// <summary>
        /// debug mehthod
        /// </summary>
        static void print(HashSet<(int, int)> blocks, HashSet<(int, int)> sand, int maxY){
            String res = "";
            for (int y = 0; y < maxY + 2; y++)
            {
                for (int x = 450; x < 550; x++)
                {
                    if(blocks.Contains((x,y))) {
                        res += "#";
                    }
                    else if(sand.Contains((x,y))){
                        res += "o";
                    }
                    else if(y == maxY + 2){
                        res += "#";
                    }
                    else{
                        res += ".";
                    }
                }
                res += "\n";
            }
            Console.WriteLine(res);
        }

    }
}