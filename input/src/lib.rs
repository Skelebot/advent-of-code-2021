use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

fn read_file<P: AsRef<Path>>(path: P) -> BufReader<File> {
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
