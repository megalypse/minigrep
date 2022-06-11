use std::{env, fs};

use colored::Colorize;

use crate::domain::structs::{Config, Line};

pub fn execute() {
    let args: Vec<String> = env::args().collect();

    let config_result = Config::new(&args);

    match config_result {
        Ok(config) => {
            let filename = &config.filename;

            let lines = get_file_lines(&filename);
            let highlighted_lines: Vec<Line> = process_lines(lines, config);

            for line in highlighted_lines {
                println!("{}", line.generate_report())
            }
        }
        Err(msg) => panic!("{}", msg),
    }
}

fn process_lines(lines: Vec<String>, config: Config) -> Vec<Line> {
    let show_all = config.show_all;
    let query = &config.query;

    lines
        .iter()
        .enumerate()
        .map::<Line, _>(|(number, content)| Line {
            number,
            content: String::from(content),
        })
        .filter(|line| show_all || line.content.contains(query))
        .map(|line| highlight_matches(line, query))
        .collect()
}

fn get_file_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("File not found.");

    contents.split("\n").map(|x| String::from(x)).collect()
}

fn highlight_matches(line: Line, query: &str) -> Line {
    let content = line
        .content
        .replace(query, &query.green().bold().to_string());

    Line {
        content,
        number: line.number,
    }
}
