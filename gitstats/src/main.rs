mod stats;
use std::env;

fn main() {
    
    let args = env::args().collect::<Vec<String> >();

    if args.len() != 2 {
        println!("Usage: gitstats <dirname>");
        return;
    }

    let mut dirname: &str = args[1].as_str();
    let mut fromback = 0;
    let mut iter = dirname.chars().rev();
    while iter.next().unwrap() == '/' {
        fromback += 1;
    }
    dirname = &dirname[0..args[1].len() - fromback];

    let getter = stats::StringGetter::new(dirname);
    println!("{}", getter.run_git_command("git log --format=oneline"));

    /*
    let hashes = getter.get_hashes();

    for h in hashes {
        println!("{}", h);
    }
    */

    let (plussum, minussum) = getter.compare_hash_range_all();

    println!("{} lines added, {} lines removed", plussum, minussum);

    let mystr = "Hello yes".to_string();

    let strref: &str = mystr.as_str();

    let strvec = strref.split(" ").collect::<Vec<&str>>();

    for s in strvec {
        println!("{}", s);
    }

    println!("{}", getter.get("/home/konrad/PhDStuff/cohortsamplingPresentation"));

    println!("Hello, world!");
}
