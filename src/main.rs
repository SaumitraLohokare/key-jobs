use std::fs;

use rake::*;

fn main() {
    let mut args = std::env::args();

    args.next().expect("Failed to get the program name..."); // Program name

    let job_descriptions_file = args
        .next()
        .expect("Please provide the job descriptions file name as the first argument.");
    let output_format = args
        .next()
        .expect("Please provide the output format as the second argument.");

    match output_format.as_str() {
        "md" => {}
        "csv" => {}
        _ => {
            eprintln!("Output format can only be one of `md` or `csv`");
            return;
        }
    }

    let file_contents = fs::read_to_string(job_descriptions_file.trim())
        .expect("Failed to read contents of the provided file");
    let descriptions: Vec<&str> = file_contents.split("<end>").collect();

    let stop_words =
        StopWords::from_file("stop-words.txt").expect("Failed to open stop words file...");

    let r = Rake::new(stop_words);

    if &output_format == "md" {
        println!("# Keywords found: ");
    } else {
        println!("Keyword,Score\n,");
    }

    let mut i = 1;
    for description in descriptions {
        let keywords = r.run(description);

        if &output_format == "md" {
            println!("## Job Description #{i}\n,");
            println!("| Keyword | Score |");
            println!("| ----- | --- |");
        } else {
            println!("Job Description #{i},\n");
        }
        i += 1;

        for KeywordScore { keyword, score } in keywords {
            if &output_format == "md" {
                println!("| {keyword} | {score} |");
            } else {
                println!("{keyword},{score}");
            }
        }

        if &output_format == "md" {
            println!("\n<hr>\n");
        } else {
            println!(",");
        }
    }
}
