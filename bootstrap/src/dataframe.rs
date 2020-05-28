use std::mem;
use std::fmt;

#[derive(Clone)]
pub enum DfEntry
{
    Float(f32),
    Text(String),
    Int(i32)
}

impl DfEntry
{
    fn getstr(&self) -> String {
        match self {
            DfEntry::Float(v) => v.to_string(),
            DfEntry::Text(v)  => v.to_string(),
            DfEntry::Int(v)   => v.to_string()
        }
    }

    fn gettype(&self) -> String {
        match self {
            DfEntry::Float(_) => "Float".to_string(),
            DfEntry::Text(_) => "Text".to_string(),
            DfEntry::Int(_) => "Int".to_string()
        }
    }
}

impl fmt::Display for DfEntry
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typestr = self.gettype();

        let value = self.getstr();
        write!(f, "{}: {}", typestr, value)
    }
}


pub struct DataFrame
{
    colnames: Vec::<String>,
    coltypes: Vec::<DfEntry>,
    data: Vec::<Vec::<DfEntry> >
}

impl DataFrame
{
    pub fn new(colnames: &Vec::<String>, coltypes: &Vec::<DfEntry>) -> DataFrame
    {
        DataFrame { colnames:   colnames.to_vec().clone(), 
                    coltypes:   coltypes.to_vec().clone(), 
                    data:       Vec::< Vec::<DfEntry> >::new() }
    }

    /*
    pub fn addcol(&mut self, colname: &str, data: &Vec::<DfEntry>)
    {
        
    }
    */

    pub fn addrow(&mut self, rowdata: &Vec::<DfEntry>) -> Result<(), String>
    {
        if rowdata.len() == 0
        {
            return Err("Collection containing data for new row is empty".to_string());
        }

        // first validate the data provided, if column types are already specified
        if self.coltypes.len() > 0
        {
            if self.coltypes.len() != rowdata.len()
            {
                return Err("Row data not of the same length as number of current columns in dataframe".to_string());
            }

            let zipiter = self.coltypes.iter().zip(rowdata.iter());

            let mut coli: u32 = 0;
            for (coltp, rowval) in zipiter
            {
                if mem::discriminant(coltp) != mem::discriminant(rowval)
                {
                    return Err(format!("Row data in column {} not of the same type as that column in dataframe", coli));
                }
                coli += 1;
            }
        }
        else
        {
            self.coltypes = rowdata.clone();
        }
        

        // validation successful, now add data
        self.data.push(rowdata.to_vec());

        Ok(())
    }
}

impl fmt::Display for DataFrame
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for colname in &self.colnames {
            write!(formatter, "{} ", colname);
        }
        write!(formatter, "\n");
        for row in &self.data {
            for val in row {
                write!(formatter, "{} ", val.getstr());
            }
            write!(formatter, "\n");
        }
        write!(formatter, "Types: ");
        for tp in &self.coltypes {
            write!(formatter, "{} ", tp.gettype());
        }
        Ok(())
    }
}
