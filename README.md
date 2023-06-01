# key-jobs
Use RAKE to extract keywords from job descriptions

## Usage:

Update the job-descriptions.txt file with important parts of your job description. Separate different job descriptions with `<end>`.

Run: `$ cargo r >output.md`

Enter: `job-descriptions.txt` in the STDIN.

## Output:

The calculated keywords will be placed in `output.md`.
