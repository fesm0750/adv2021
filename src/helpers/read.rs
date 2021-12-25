// todo: implement tests

use std::{fs::File, io, io::prelude::*, str::FromStr};

//--------------------------------------------------------------------
// Read Input
//--------------------------------------------------------------------

/// reads the whole file into a String.
pub fn to_str(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open("inputs/".to_string() + filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

//------------------------------
// Buffered Outputs
//------------------------------

// returns a buffered reader
pub fn to_bufreader(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open("inputs/".to_string() + filename)?;
    Ok(io::BufReader::new(file))
}

// returns an Iterator over lines of a buffered reader
pub fn to_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("inputs/".to_string() + filename)?;
    Ok(io::BufReader::new(file).lines())
}

// returns an Iterator over a buffered reader, lines a parsed into type T
pub fn to_iter<T>(filename: &str) -> io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    let file = File::open("inputs/".to_string() + filename)?;
    let iter = io::BufReader::new(file)
        .lines()
        .flatten()
        .flat_map(|s| (&s).parse::<T>());
    Ok(iter)
}

//--------------------------------------------------------------------
// Commom Parsing
//--------------------------------------------------------------------

//------------------------------
// Entries are single lines
//------------------------------

/// parses an `input` where each line is an entry into a `Vec`.
pub fn lines_into_vec<T: FromStr>(input: &str) -> Vec<T> {
    input.lines().flat_map(str::parse::<T>).collect()
}

/// returns an iterator over parsed values of an `input` string slice where the
/// entries are separated by a new line.
pub fn parsed_lines_iter<'a, T>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.lines().flat_map(str::parse::<T>)
}

/// returns an iterator over parsed values of an `input` string slice where the
/// entries are separated by a new line.
pub fn parsed_lines_iter_cloneable<'a, T>(input: &'a str) -> impl Iterator<Item = T> + Clone + 'a
where
    T: FromStr + Clone + 'a,
{
    input.lines().map(str::parse::<T>).flatten()
}

//------------------------------
// Entries are separated by custom characters
//------------------------------

/// parses an `input` into a `Vec<T>`. Entries in the string slice are
/// separated by the `split_at` characters.
pub fn split_into_vec<T>(input: &str, split_at: &str) -> Vec<T>
where
    T: FromStr,
{
    input.split(split_at).flat_map(str::parse::<T>).collect()
}

/// returns an iterator over parsed values of an `input` string where the
/// entries are separated by the `split_at` characters.
pub fn parsed_split_iter<'a, T>(input: &'a str, split_at: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.split(split_at).map(str::parse::<T>).flatten()
}
