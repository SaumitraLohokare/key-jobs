# key-jobs
Use RAKE to extract keywords from job descriptions

## Usage:

Update the job-descriptions.txt file with important parts of your job description. Separate different job descriptions with `<end>`.

Run: 

`$ cargo build`
`$ ./target/debug/key-jobs job-descriptions.txt [md|csv]`

## Output:

The calculated keywords will be placed in `output.md` or `output.csv`.
