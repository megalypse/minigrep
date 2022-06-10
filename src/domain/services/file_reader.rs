use std::{env, fs};

use regex::Regex;

use crate::domain::structs::FileResult;

pub fn execute() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    let lines = get_file_lines(filename);
    let result_list = get_results(&lines, query);

    report_results(&result_list);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args.get(1).expect("Search pattern must be provided.");
    let filename = &args.get(2).expect("File to be searched must be provided.");

    (query, filename)
}

fn get_file_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("File not found.");

    contents.split("\n").map(|x| String::from(x)).collect()
}

fn find_word_in_text<'a, 'b>(search: &'a Regex, target: &'b String) -> Vec<String> {
    search
        .captures_iter(target)
        .map(|x| String::from(&x[0]))
        .collect()
}

fn get_results(lines: &[String], query: &str) -> Vec<FileResult> {
    let regex = Regex::new(query).unwrap();

    lines
        .iter()
        .enumerate()
        .map::<Vec<FileResult>, _>(|(index, line)| {
            let result_list = find_word_in_text(&regex, line);

            result_list
                .iter()
                .map(|result| FileResult {
                    line: index + 1,
                    word: String::from(result),
                })
                .collect()
        })
        .flatten()
        .collect()
}

fn report_results(result_list: &[FileResult]) -> () {
    let result: &str = &result_list
        .iter()
        .map(|dto| dto.line.to_string())
        .reduce(|acc, item| format!("{}, {}", acc, item))
        .unwrap();

    println!("Searched text present in lines: {}", result);
}
