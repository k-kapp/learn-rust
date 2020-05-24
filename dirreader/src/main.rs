use std::io;
use std::fs::{self, DirEntry, Metadata};
use std::path::Path;
use std::env;
mod entry;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 
    {
        println!("Usage: ./sizes <dirname>");
        return ();
    }

    let mut entries = fs::read_dir(args.get(1).unwrap()).expect("Unable to list");

    let mut allentries: Vec::<entry::Entry> = Vec::<entry::Entry>::new();

    for entry in entries
    {
        let entry = entry.expect("Unable to get entry");
        let path = entry.path();
        let fname = path.to_str().unwrap();
        let ent = entry::Entry::new(fname.to_string());
        allentries.push(ent);
    }

    allentries.sort_by(|a, b| b.size.cmp(&a.size));

    for e in allentries
    {
        println!("{}", e);
    }
}
