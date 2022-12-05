using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AOC22
{
    internal class Program
    {
        static void Main(string[] args)
        {
            String[] input = loadInput();
            Console.WriteLine("05");
            Console.WriteLine(part1(input));
            Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static String part1(String[] input)
        {
            var stackDict = getDictFromInput(input);
            bool skippedCrates = false;
            foreach (var line in input)
            {
                if (skippedCrates)
                {
                    var matches = Regex.Matches(line, "[0-9]+");
                    MoveXFromYToZ(stackDict,
                        Int32.Parse(matches[0].Value),
                        Int32.Parse(matches[1].Value),
                        Int32.Parse(matches[2].Value)
                    );

                }
                if (line == String.Empty)
                {
                    skippedCrates = true;
                }
            }
            String resultString = "";
            for (int i = 0; i < stackDict.Keys.Count; i++)
            {
                resultString += stackDict[i + 1].Peek();
            }
            return resultString;
        }

        static String part2(String[] input)
        {
            var stackDict = getDictFromInput(input);
            bool skippedCrates = false;
            foreach (var line in input)
            {
                if (skippedCrates)
                {
                    var matches = Regex.Matches(line, "[0-9]+");
                    MoveXFromYToZ2(stackDict,
                        Int32.Parse(matches[0].Value),
                        Int32.Parse(matches[1].Value),
                        Int32.Parse(matches[2].Value)
                    );

                }
                if (line == String.Empty)
                {
                    skippedCrates = true;
                }
            }
            String resultString = "";
            for (int i = 0; i < stackDict.Keys.Count; i++)
            {
                resultString += stackDict[i + 1].Peek();
            }
            return resultString;
        }

        static void MoveXFromYToZ(Dictionary<int, Stack<char>> stackDict, int x, int y, int z)
        {
            for (int i = 0; i < x; i++)
            {
                char crate = stackDict[y].Pop();
                stackDict[z].Push(crate);
            }
        }

        static void MoveXFromYToZ2(Dictionary<int, Stack<char>> stackDict, int x, int y, int z)
        {
            List<char> temp = new List<char>();
            for (int i = 0; i < x; i++)
            {
                char crate = stackDict[y].Pop();
                temp.Add(crate);
            }

            for (int i = temp.Count - 1; i >= 0; i--)
            {
                stackDict[z].Push(temp[i]);
            }
        }
        static Dictionary<int, Stack<char>> getDictFromInput(String[] input)
        {
            Dictionary<int, Stack<char>> stackDict = new Dictionary<int, Stack<char>>();
            Dictionary<int, List<char>> dict = new Dictionary<int, List<char>>();
            foreach (String line in input)
            {
                int result = 0;
                if (Int32.TryParse(line[1].ToString(), out result))
                {
                    break;
                }
                String nLine = line.Replace("[", " ").Replace("]", " ");
                int j = 1;
                for (int i = 0; i < nLine.Length; i++)
                {
                    if ((i - 1) % 4 == 0)
                    {
                        if (nLine[i] != ' ')
                        {
                            if (!dict.ContainsKey(j))
                            {
                                dict[j] = new List<char>();
                            }
                            dict[j].Add(nLine[i]);
                        }
                        j += 1;
                    }
                }
            }

            foreach (var key in dict.Keys)
            {
                stackDict[key] = new Stack<char>();
                for (int i = dict[key].Count - 1; i >= 0; i--)
                {
                    stackDict[key].Push(dict[key][i]);
                }
            }
            return stackDict;
        }
    }
}