using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC22
{
    public class AOCFile
    {
        private string name;
        private int size;
        public string Name{ get => name; }
        public int Size{ get => size; }
        public AOCFile(string name, int size){
            this.name = name;
            this.size = size;
        }
    }
}