// todo: implement tests

use std::{fs::File, io, io::prelude::*, str::FromStr};

//--------------------------------------------------------------------
// Read File
//--------------------------------------------------------------------

//------------------------------
// Buffered Reader
//------------------------------

// returns a buffered reader
pub fn to_bufreader(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = get_file(filename)?;
    Ok(io::BufReader::new(file))
}

//------------------------------
// Read Into Memory
//------------------------------

/// reads the whole file into a String.
pub fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = get_file(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

/// splits the contents of the file at `split_at` and parses each section into T, returning a Vec<T>
/// T may be a String or any type that can be parsed from a string
pub fn file_to_vec<T>(filename: &str, split_bit: u8) -> Result<Vec<T>, io::Error>
where
    T: FromStr,
{
    let iter = file_to_string_iter(filename, split_bit)?;
    Ok(iter.flat_map(|s| s.parse()).collect())
}

/// reads the file parsing each line into type T and returning a Vec<T>
pub fn file_lines_to_vec<T>(filename: &str) -> Result<Vec<T>, io::Error>
where
    T: FromStr,
{
    let iter = file_to_iter::<T>(filename)?;
    Ok(iter.collect())
}

//------------------------------
// Read as Iterator
//------------------------------

// returns an Iterator over lines of a file
pub fn file_to_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = get_file(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// returns an Iterator over a buffered reader, lines are parsed into type T
pub fn file_to_iter<T>(filename: &str) -> io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    let file = get_file(filename)?;
    let iter = io::BufReader::new(file)
        .lines()
        .flatten()
        .flat_map(|s| (&s).parse::<T>());
    Ok(iter)
}

pub fn file_to_string_iter(
    filename: &str,
    split_bit: u8,
) -> Result<impl Iterator<Item = String>, io::Error> {
    let bf = to_bufreader(filename)?;
    Ok(bf
        .split(split_bit)
        .flatten()
        .filter(|v| !v.is_empty())
        .flat_map(String::from_utf8))
}

//--------------------------------------------------------------------
// Parsing String into Collection and Iterators
//--------------------------------------------------------------------

//------------------------------
// Collection
//------------------------------

/// parses an `input` where each line is an entry into a `Vec`.
pub fn lines_into_vec<T: FromStr>(input: &str) -> Vec<T> {
    input.lines().flat_map(str::parse::<T>).collect()
}

/// parses an `input` into a `Vec<T>`. Entries in the string slice are
/// separated by the `split_at` characters.
pub fn split_into_vec<T>(input: &str, split_at: &str) -> Vec<T>
where
    T: FromStr,
{
    input.split(split_at).flat_map(str::parse::<T>).collect()
}

//------------------------------
// Iterators
//------------------------------

/// returns an iterator over parsed values of an `input` string slice where the
/// entries are separated by a new line.
pub fn parsed_lines_iter<'a, T>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.lines().flat_map(str::parse::<T>)
}

/// returns an iterator over parsed values of an `input` string where the
/// entries are separated by the `split_at` characters.
pub fn parsed_split_iter<'a, T>(input: &'a str, split_at: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.split(split_at).map(str::parse::<T>).flatten()
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

fn get_file(filename: &str) -> Result<File, io::Error> {
    File::open("inputs/".to_string() + filename)
}

//--------------------------------------------------------------------
// Others
//--------------------------------------------------------------------

/// returns an iterator over parsed values of an `input` string slice where the
/// entries are separated by a new line.
pub fn parsed_lines_iter_cloneable<'a, T>(input: &'a str) -> impl Iterator<Item = T> + Clone + 'a
where
    T: FromStr + Clone + 'a,
{
    input.lines().map(str::parse::<T>).flatten()
}
