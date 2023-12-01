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
            Console.WriteLine("15");
            Console.WriteLine(part1(input));
            // Console.WriteLine(part2(input));
        }

        static String[] loadInput()
        {
            return File.ReadAllLines("input.txt", System.Text.Encoding.UTF8);
        }

        static Dictionary<(int, int), (int, int)> parseInput(String[] input)
        {
            int numCount = 4;
            string pattern = @"\d+";
            int[] current = new int[numCount];
            Dictionary<(int, int), (int, int)> sensorBeacons = new Dictionary<(int, int), (int, int)>();
            foreach (var line in input)
            {
                Regex r = new Regex(pattern);
                var matches = r.Matches(line);
                for (int i = 0; i < numCount; i++)
                {
                    current[i] = int.Parse(matches.ElementAt(i).Value);
                }
                (int, int) sx = (current[0], current[1]);
                (int, int) bx = (current[2], current[3]);
                sensorBeacons.Add(sx, bx);
            }
            return sensorBeacons;
        }
        static int computeManhattanDist((int, int) a, (int, int) b)
        {
            return Math.Abs(a.Item1 - b.Item1) + Math.Abs(a.Item2 - b.Item2);
        }
        static int part1(String[] input)
        {
            int yval = 2000000;
            Dictionary<(int, int), (int, int)> sensorBeacons = parseInput(input);
            Dictionary<(int, int), int> distances = new Dictionary<(int, int), int>();
            foreach (var key in sensorBeacons.Keys)
            {
                distances.Add(key, computeManhattanDist(key, sensorBeacons[key]));
            }
            HashSet<(int, int)> detectedSpaces = new HashSet<(int, int)>();
            foreach (var key in sensorBeacons.Keys)
            {
                int dist = distances[key];
                for (int x = -dist; x <= dist; x++)
                {
                    var p = (x + key.Item1, yval);
                    if (computeManhattanDist(key, p) <= dist)
                    {
                        if (!sensorBeacons.ContainsKey(p))
                        {
                            detectedSpaces.Add(p);
                        }
                    }
                }
            }
            // print(detectedSpaces, sensorBeacons);
            int result = detectedSpaces.Count(a => a.Item2 == yval && !(sensorBeacons.ContainsKey(a) || sensorBeacons.ContainsValue(a)));
            return result;
        }
        static void print(HashSet<(int, int)> detectedSpaces, Dictionary<(int, int), (int, int)> sensorBeacons)
        {
            int minx = 0;
            int maxx = 20;
            int miny = 0;
            int maxy = 20;
            string res = "";
            for (int y = miny; y < maxy; y++)
            {
                for (int x = minx; x < maxx; x++)
                {
                    if (sensorBeacons.ContainsKey((x, y)))
                    {
                        res += "S";
                    }
                    else if (sensorBeacons.ContainsValue((x, y)))
                    {
                        res += "B";
                    }
                    else if (detectedSpaces.Contains((x, y)))
                    {
                        res += "#";
                    }
                    else
                    {
                        res += ".";
                    }
                }
                res += y.ToString() + "\n";
            }
            Console.WriteLine(res);
        }

        static int part2(String[] input)
        {
            Dictionary<(int, int), (int, int)> sensorBeacons = parseInput(input);
            Dictionary<(int, int), int> distances = new Dictionary<(int, int), int>();
            foreach (var key in sensorBeacons.Keys)
            {
                distances.Add(key, computeManhattanDist(key, sensorBeacons[key]));
            }
            HashSet<(int, int)> detectedSpaces = new HashSet<(int, int)>();
            foreach (var key in sensorBeacons.Keys)
            {
                int dist = distances[key];
                for (int y = -dist; y <= dist; y++)
                {

                    for (int x = -dist; x <= dist; x++)
                    {
                        var p = (x + key.Item1, y + key.Item2);
                        if (computeManhattanDist(key, p) <= dist)
                        {
                            detectedSpaces.Add(p);
                        }
                    }
                }
            }
            Console.WriteLine("a");
            // print(detectedSpaces, sensorBeacons);

            int m = 4000000;
            for (int y = 0; y <= m; y++)
            {
                for (int x = 0; x < m; x++)
                {
                    var p = (x, y);
                    if (!detectedSpaces.Contains(p) &&
                        detectedSpaces.Contains((p.Item1, p.Item2 + 1)) &&
                        detectedSpaces.Contains((p.Item1, p.Item2 - 1)) &&
                        detectedSpaces.Contains((p.Item1 + 1, p.Item2)) &&
                        detectedSpaces.Contains((p.Item1 - 1, p.Item2))
                        )
                    {
                        return p.Item1 * 4000000 + p.Item2;
                    }
                }
            }
            // int result = detectedSpaces.Count(a => a.Item2 == yval && !(sensorBeacons.ContainsKey(a) || sensorBeacons.ContainsValue(a)));
            return -1;
        }

    }
}