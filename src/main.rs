use std::{env::Args, fs, io::stdin};

use rake::*;

fn main() {
    let mut job_descriptions_file = String::new();

    match stdin().read_line(&mut job_descriptions_file) {
        Ok(_) => {}
        Err(err) => eprintln!("{err}"),
    };

    let file_contents = fs::read_to_string(job_descriptions_file.trim())
        .expect("Failed to read contents of the provided file");
    let descriptions: Vec<&str> = file_contents.split("<end>").collect();

    let stop_words =
        StopWords::from_file("stop-words.txt").expect("Failed to open stop words file...");

    let r = Rake::new(stop_words);

    println!("# Keywords found: ");

    let mut i = 1;
    for description in descriptions {
        let keywords = r.run(description);
        println!("## Job Description ##{i}\n");
        i += 1;
        println!("| Keyword | Score |");
        println!("| ----- | --- |");

        for KeywordScore { keyword, score } in keywords {
            println!("| {keyword} | {score} |");
        }
        println!("</hr>");
    }
}
