using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
namespace AOC22
{

    public class AOCDirectory
    {
        private string name;
        public string Name { get => name; }
        private List<AOCFile> files;
        private List<AOCDirectory> subDirs;
        public List<AOCFile> Files { get => files; }
        public List<AOCDirectory> SubDirs { get => subDirs; }
        public AOCDirectory(string name)
        {
            this.name = name;
            files = new List<AOCFile>();
            subDirs = new List<AOCDirectory>();
        }
        public void AddDir(AOCDirectory dir)
        {
            subDirs.Add(dir);
        }

        public void AddFile(AOCFile file)
        {
            files.Add(file);
        }

        public AOCDirectory findDir(string name){
            foreach (var item in subDirs)
            {
                if(item.name == name){
                    return item;
                }
            }
            return null;
        }

        public int findTotalSize(){
            int size = 0;
            foreach (var file in files)
            {
                size += file.Size;
            }
            foreach (var dir in subDirs)
            {
                size += dir.findTotalSize();
            }
            return size;
        }

        public List<AOCDirectory> findAllSubDirs(){
            List<AOCDirectory> l = new List<AOCDirectory>();
            l.AddRange(subDirs);
            foreach (var item in subDirs)
            {
                l.AddRange(item.findAllSubDirs());
            }
            return l;
        }
    }

}