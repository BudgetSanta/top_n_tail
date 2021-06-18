use clap::Clap;
use std::{
    cmp, fs,
    io::{self, Read},
    process::exit,
};

fn main() {
    let opts: Opts = Opts::parse();

    let input = match opts.file {
        Some(f) => {
            let contents = fs::read_to_string(f);
            match contents {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading in file: {}", e);
                    exit(1);
                }
            }
        }
        None => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => buffer,
                Err(e) => {
                    eprintln!("Error readin from stdin: {}", e);
                    exit(1);
                }
            }
        }
    };

    let lines = input.lines().collect::<Vec<&str>>();

    let output = filter(opts.start, opts.end, lines);

    println!("{}", output)
}

#[derive(Clap, Debug)]
struct Opts {
    /// Start line index
    start: usize,

    /// End line index
    end: Option<usize>,

    /// File to top and tail
    #[clap(short, long)]
    file: Option<String>,
}

fn filter(start: usize, end: Option<usize>, lines: Vec<&str>) -> String {
    let count = lines.len();

    if start >= count {
        return "".to_string();
    }

    match end {
        None => lines[start..].join("\n"),
        Some(end) => {
            if end < start {
                panic!("End index must be greater than or equal to start index.")
            }

            let end = cmp::min(end, count - 1);
            lines[start..=end].join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    mod filter {
        static POEM: &str = r#"I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!"#;

        fn poem_vec() -> Vec<&'static str> {
            POEM.lines().collect()
        }

        #[test]
        fn start_and_end_all_lines() {
            // Arrange
            let lines = poem_vec();
            let start = 0;
            let end = lines.len() - 1;
            let expected = POEM;

            // Act
            let actual = crate::filter(start, Some(end), lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_and_end_subset_top() {
            // Arrange
            let start = 0;
            let end = 5;
            let lines = poem_vec();
            let expected = lines[start..=end].join("\n");

            // Act
            let actual = crate::filter(start, Some(end), lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_and_end_subset_bottom() {
            // Arrange
            let lines = poem_vec();
            let start = 3;
            let end = lines.len() - 1;
            let expected = lines[start..=end].join("\n");

            // Act
            let actual = crate::filter(start, Some(end), lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_valid_but_end_greater_than_end_of_file() {
            // Arrange
            let start = 2;
            let end = usize::MAX;
            let lines = poem_vec();
            let expected = lines[start..].join("\n");

            // Act
            let actual = crate::filter(start, Some(end), lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        #[should_panic(expected = "End index must be greater than or equal to start index.")]
        fn start_greater_than_end() {
            // Arrange
            let lines = poem_vec();
            let start = lines.len() - 2;
            let end = 2;

            // Act
            crate::filter(start, Some(end), lines);
        }

        #[test]
        fn start_and_end_greater_than_end_of_file() {
            // Arrange
            let start = usize::MAX - 1;
            let end = usize::MAX;
            let lines = poem_vec();
            let expected = String::from("");

            // Act
            let actual = crate::filter(start, Some(end), lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_and_end_equal() {
            // Arrange
            let start = 3;
            let end = 3;
            let lines = poem_vec();
            let expected = lines[3];

            // Act
            let actual = crate::filter(start, Some(end), lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_only_zero_whole_file() {
            // Arrange
            let start = 0;
            let lines = poem_vec();
            let expected = POEM;

            // Act
            let actual = crate::filter(start, None, lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_only_subset() {
            // Arrange
            let start = 5;
            let lines = poem_vec();
            let expected = lines[start..].join("\n");

            // Act
            let actual = crate::filter(start, None, lines);

            // Assert
            assert_eq!(expected, actual);
        }

        #[test]
        fn start_only_greater_than_end_of_file() {
            // Arrange
            let start = usize::MAX;
            let lines = poem_vec();
            let expected = String::from("");

            // Act
            let actual = crate::filter(start, None, lines);

            // Assert
            assert_eq!(expected, actual);
        }
    }
}
