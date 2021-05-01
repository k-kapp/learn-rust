use std::process::Command;
use std::process;

pub struct StringGetter
{
    gitdir: String,
    ncommits: u32,
    show_adds: bool,
    show_removes: bool
}

impl StringGetter
{
    pub fn new(dirname: &str) -> StringGetter {
        let mut gitdir = dirname.to_string();
        gitdir.push_str("/.git");
        StringGetter {gitdir: gitdir, ncommits: 0, show_adds: false, show_removes: false}
    }

    pub fn run_git_command(self: &StringGetter, comm: &str) -> String {
        let vec = comm.split(" ").collect::<Vec<_>>();
        let res = Command::new(vec[0])
            .arg("--git-dir")
            .arg(self.gitdir.as_str())
            .args(&vec[1..])
            .output();
        match res {
            Ok(v) => (if v.status.success() { v.stdout } else { v.stderr }).iter().map(|&x| x as char).collect::<String>(),
            Err(_) => "Error".to_string()
        }
    }

    pub fn compare_hashes(self: &StringGetter, hash1: &str, hash2: &str) -> (i32, i32) {
        let mut mystr = "git diff --numstat ".to_string();
        mystr.push_str(hash1);
        mystr.push_str(" ");
        mystr.push_str(hash2);
        let outstr = self.run_git_command(&mystr);
        let mut plussum = 0;
        let mut minussum = 0;
        for l in (&outstr).lines() {
            let strvec = l.split("\t").take(2).collect::<Vec<&str> >();
            let plusres = strvec[0].parse::<i32>();
            let minres = strvec[1].parse::<i32>();
            let (plus, minus) = match (plusres, minres) {
                (Err(_), _) | (_, Err(_)) => {println!("Warning: could not parse line {}. Ignoring it.", l); continue },
                (Ok(p), Ok(m))            => (p, m),
                _                         => panic!(format!("Unknown result for line {}", l))
            };
            if (plus < 0 || minus < 0) {
                panic!("Parsed values from git output cannot be negative!");
            }
            plussum += plus;
            minussum += minus;
        }
        (plussum, minussum)
    }

    pub fn compare_hash_range_all(self: &StringGetter) -> (i32, i32) {
        let hashes = self.get_hashes();
        let mut addsum = 0;
        let mut remsum = 0;
        for i in 0..(hashes.len() - 1) {
            let (add, rem) = self.compare_hashes_idxes(i, i + 1);
            addsum += add;
            remsum += rem;
        }
        (addsum, remsum)
    }

    pub fn compare_hashes_idxes(self: &StringGetter, hidx1: usize, hidx2: usize) -> (i32, i32) {
        let hashes = self.get_hashes();
        self.compare_hashes(&hashes[hidx1], &hashes[hidx2])
    }

    pub fn get_hashes(self: &StringGetter) -> Vec<String> {
        let hashstr = self.run_git_command("git log --format=oneline");
        let lines = hashstr.split("\n").collect::<Vec<&str> >();
        let linesf = lines.into_iter().filter(|s| s.len() > 0).collect::<Vec<&str> >();
        linesf.iter().map(|s| s.split(" ").collect::<Vec<&str> >()[0].to_string()).collect::<Vec<String> >()
    }

    pub fn get(self: &StringGetter, dirname: &str) -> String {
        let mut calldir: String = dirname.to_owned();
        calldir.push_str("/.git");
        let mut outvec: Vec<u8>;
        let res = Command::new("git")
            .args(&["--git-dir", calldir.as_str(), "log", "--numstat", "HEAD~1", "HEAD"])
            .output();
        match res {
            Ok(v) => (if v.status.success() { v.stdout } else { v.stderr }).iter().map(|&x| x as char).collect::<String>(),
            Err(_) => "Error".to_string()
        }
        /*
        Command::new("git")
            .args(&["--git-dir", calldir.as_str(),"--numstat", "HEAD~1", "HEAD"])
            .output()
            .expect("Couldn't execute git command")
            .stdout
            .iter()
            .map(|&x| x as char)
            .collect::<String>()
        */
    }
}

