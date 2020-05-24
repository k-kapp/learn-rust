use std::fmt;
use std::fs::{self, DirEntry, Metadata};
use std::path::Path;
use std::env;
use std::io;

pub struct Entry
{
    pub name: String,
    pub size: u64,
    pub isdir: bool,
    pub nfiles: u64,
    pub ndirs: u64
}

impl fmt::Display for Entry
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sizestr = "bytes";
        let mut dispsize = self.size as f32;
        if self.size > 1024
        {
            sizestr = "kB";
            dispsize /= 1024f32;
        }
        if self.size > 1024 * 1024
        {
            sizestr = "MB";
            dispsize /= 1024f32;
        }
        if self.size > 1024 * 1024 * 1024
        {
            sizestr = "GB";
            dispsize /= 1024f32;
        }

        let mut writestr = format!("{0: <80} --- {1: <12} {2: <8}", self.name, dispsize, sizestr).to_string();
        if self.isdir
        {
            writestr += &format!(" ({} files)", self.nfiles).to_string();
        }

        write!(f, "{}", writestr)
    }
}

impl Entry
{
    pub fn new(name: String) -> Entry
    {
        let tempstr = name.clone();
        let temppath = std::path::Path::new(&tempstr);
        let mut ent = Entry {name: name, size: 0, isdir: temppath.is_dir(), nfiles: 0, ndirs: 0};
        if temppath.exists()
        {
            ent.calc_size();
        }
        ent
    }

    pub fn calc_size(&mut self)
    {
        if self.isdir
        {
            println!("Reading dir {}...", self.name);
            let tempstr = self.name.clone();
            self.calc_dirsize(tempstr.as_str());
        }
        else
        {
            let mdata = fs::metadata(&self.name).expect(format!("Error on file name {}", self.name).as_str());
            self.size = mdata.len();
        }
    }

    fn calc_dirsize(&mut self, pname : &str)
    {
        let entriesres = fs::read_dir(pname);
        let entries = match entriesres
        {
            Ok(v) => v,
            Err(_) => return
        };
        for entry in entries
        {
            let entry = entry.expect("Unable to get entry");
            //let firstchar = entry.path().file_name().unwrap().to_str().unwrap().chars().next().unwrap();
            //let nothidden = firstchar != '.';
            let path = entry.path();

            if !path.exists()
            {
                continue;
            }
            
            if path.is_dir()
            {
                //println!("Reading dir {}...", &pname);
                self.calc_dirsize(path.to_str().unwrap());
                self.ndirs += 1;
                if self.ndirs % 100 == 0
                {
                    println!("Number of directories: {}", self.ndirs);
                    println!("{}", path.to_str().unwrap());
                }
            }
            else
            {
                let fname = path.to_str().unwrap();
                let mdata = fs::metadata(fname).expect(format!("Error on file name {}", fname).as_str());
                self.size += mdata.len();
                self.nfiles += 1;
            }
        }
    }
}
