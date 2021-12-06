use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn read_file<P: AsRef<Path>>(path: P) -> BufReader<File> {
    let file = File::open(path).expect("failed to read file");
    BufReader::new(file)
}

pub fn read_lines<T, P: AsRef<Path>>(path: P) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_file(path)
        .lines()
        .map(|line| {
            line.expect("invalid line")
                .parse()
                .expect("failed to parse")
        })
        .collect()
}

pub fn read_line_split<T, P: AsRef<Path>>(path: P, delim: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_file(path)
        .lines()
        .next()
        .expect("file contained zero lines")
        .unwrap()
        .split(delim)
        .map(|s| s.trim().parse().expect("failed to parse"))
        .collect()
}
