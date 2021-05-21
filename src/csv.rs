use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct CsvLine{
    pub values: Vec<String>,
    pub n: i32,
}

pub struct CsvData{
    pub header: Vec<String>,
    pub lines: Vec<CsvLine>,
    pub n_lines: i32,
}

fn make_csv_line(line: String, _n: i32) -> CsvLine{
    CsvLine{
        values: copy_str_vector(line.split(",").collect()),
        n: _n,
    }
}

fn copy_str_vector(vec: Vec<&str>) -> Vec<String>{
    let mut v: Vec<String> = Vec::new();
    for item in vec{
        v.push((*item).to_string());
    }
    v
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_csv(fname: String) -> CsvData{
    let mut header: Vec<String> = Vec::new();
    let mut lines = Vec::new();
    let mut n_lines = i32::from(0);
    if let Ok(reader) = read_lines(fname){
        for raw_line in reader{
            if let Ok(line)=raw_line {
                if n_lines==0 {
                    header = copy_str_vector(line.split(",").collect());
                }else{
                    lines.push(make_csv_line(line, n_lines-1));
                }
            }
            n_lines=n_lines+1;
            print!("Read {n_lines} lines\r", n_lines=n_lines); 
        }
        println!("");
    }

    CsvData{
        header: header,
        lines: lines,
        n_lines: n_lines,
    }
}
