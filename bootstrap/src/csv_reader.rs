use std::fs;
use crate::dataframe;

fn parse_line(line: &Vec::<String>) -> Vec::<dataframe::DfEntry>
{
    let mut retval = Vec::<dataframe::DfEntry>::new();
    for el in line
    {
        let parseresult = el.parse::<f32>();
        if parseresult.is_ok() {
            retval.push(dataframe::DfEntry::Float(parseresult.unwrap()));
        }
        else {
            retval.push(dataframe::DfEntry::Text(el.to_string()));
        }
    }
    retval
}

pub fn read_csv(filename: &str) -> dataframe::DataFrame
{
    let contents = fs::read_to_string(filename).expect(format!("Could not open file {}", filename).as_str());

    let splititer = contents.split("\n");

    let mut lines  = Vec::< Vec::<String> >::new();

    for line in splititer
    {
        let els = line.split(",").map(|a| a.to_string()).collect();
        lines.push(els);
        println!("{}", line);
    }

    let mut parsed = Vec::< Vec::<dataframe::DfEntry> >::new();

    for line in &lines[1..]
    {
        parsed.push(parse_line(line));
    }

    for pvec in &parsed
    {
        for p in pvec
        {
            print!("{} ", p);
        }
        println!();
    }

    let colnames = lines[0].clone();
    let coltypes = parsed[0].clone();
    let mut df = dataframe::DataFrame::new(&colnames, &coltypes);

    for pvec in &parsed
    {
        let result = df.addrow(pvec);
        if !result.is_ok()
        {
            println!("WARNING: Could not add row to dataframe");
        }
    }
    df
}
