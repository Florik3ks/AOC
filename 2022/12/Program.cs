using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC22
{
    internal class Program
    {
        private static (int, int) start;
        private static (int, int) end;
        private static int[,] field = new int[0, 0]; // cant be null, fill with dummy array

        static void Main(string[] args)
        {
            String[] input = loadInput();
            Console.WriteLine("12");

            parseInput(input);
            Console.WriteLine(part1(start));

            parseInput(input);
            Console.WriteLine(part2());
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }
        static void parseInput(String[] input)
        {
            field = new int[input[0].Length, input.Length];
            for (int y = 0; y < input.Length; y++)
            {
                char[] array = input[y].ToCharArray();
                for (int x = 0; x < array.Length; x++)
                {
                    if (char.IsLower(array[x]))
                    {
                        field[x, y] = (int)array[x];
                    }
                    else
                    {
                        if (array[x] == 'S')
                        {
                            field[x, y] = (int)'a';
                            start = (x, y);
                        }
                        else if (array[x] == 'E')
                        {
                            field[x, y] = (int)'z';
                            end = (x, y);
                        }
                    }
                }
            }
        }

        static String part2()
        {
            List<(int, int)> a = new List<(int, int)>();
            for (int y = 0; y < field.GetLength(1); y++)
            {
                for (int x = 0; x < field.GetLength(0); x++)
                {
                    if (field[x, y] == (int)'a')
                    {
                        a.Add((x, y));
                    }
                }
            }

            int min = int.MaxValue;
            foreach (var start in a)
            {
                min = Math.Min(min, part1(start));
            }
            return min.ToString();
        }
        static int part1((int, int) start)
        {
            bool[,] visited = new bool[field.GetLength(0), field.GetLength(1)];
            (int, int) current = start;
            int s = 0;
            Queue<((int, int), int)> q = new Queue<((int, int), int)>();
            q.Enqueue((current, 0));
            while (!(current.Item1 == end.Item1 && current.Item2 == end.Item2))
            {
                if(q.Count == 0) return int.MaxValue;
                (current, s) = q.Dequeue();

                visited[current.Item1, current.Item2] = true;
                foreach (var n in getNeighbours(current, visited))
                {
                    if (field[current.Item1, current.Item2] + 1 >= field[n.Item1, n.Item2])
                    {
                        if (!q.Any(a => a.Item1.Item1 == n.Item1 && a.Item1.Item2 == n.Item2))
                        {
                            q.Enqueue((n, s + 1));
                        }
                    }
                }
            }

            return s;
        }
        static void printArray(bool[,] visited)
        {
            for (int y = 0; y < visited.GetLength(1); y++)
            {
                String a = "";
                for (int x = 0; x < visited.GetLength(0); x++)
                {
                    a += visited[x, y] ? "#" : ".";
                }
                Console.WriteLine(a + " " + y.ToString());
            }
            Console.WriteLine();
        }
        static List<(int, int)> getNeighbours((int, int) f, bool[,] visited)
        {
            List<(int, int)> result = new List<(int, int)>();
            result.Add((f.Item1 + 1, f.Item2 + 0));
            result.Add((f.Item1 - 1, f.Item2 + 0));
            result.Add((f.Item1 + 0, f.Item2 + 1));
            result.Add((f.Item1 + 0, f.Item2 - 1));

            for (int i = 3; i >= 0; i--)
            {
                if (result[i].Item1 < 0 || result[i].Item2 < 0)
                {
                    result.RemoveAt(i);
                }
                else if (result[i].Item1 > visited.GetLength(0) - 1 || result[i].Item2 > visited.GetLength(1) - 1)
                {
                    result.RemoveAt(i);
                }
                else if (visited[result[i].Item1, result[i].Item2])
                {
                    result.RemoveAt(i);
                }
            }
            return result;
        }

    }
}