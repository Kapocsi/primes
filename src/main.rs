use std::env;

/// Returns a string of a progress bar to be printed to the command line
/// get_progress_bar(5,10,10) -> |█████-----|
fn get_progress_bar(current: i32, max: i32, progress_bar_len: i32) -> String {
    let bar_size = progress_bar_len as f32 / max as f32;
    format!(
        "|{}{}| {}/{} ~ {:.2}%",
        { "█".repeat((current as f32 * bar_size) as usize) },
        { "-".repeat(((max - current) as f32 * bar_size) as usize) },
        current,
        max,
        (current as f32 / max as f32) * 100.00
    )
}

/// Returns a Vec of primes up to the limit set by 'n'
fn get_prime_to_limit(n: i32, do_progress_bar: bool) -> Vec<i32> {
    let mut primes: Vec<i32> = vec![2, 3];
    for i in (3..n).step_by(2) {
        let mut is_prime: bool = true;
        for x in &primes {
            if i % x == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i)
        }
        if do_progress_bar {
            print!("\r{}", get_progress_bar(i, n, 19));
        }
    }
    primes
}

fn main() {
    const HELP_TEXT: &str = "This is command line interface to primes\n Usage: \r primes <i32 int> prints the prime numbers up the limit set \n Options:\n -npb or --no-progress-bar does the same thing as before but removes the progress bar this combined with the '>' can be used to send the numbers to a text file";

    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("{}", HELP_TEXT);
        return;
    }

    match args.len() {
        0 => {
            panic!("Function Cannot have 0 arguments")
        }
        1 => {
            println!("Cannot Run Without Args, use -h to see usage");
        }
        2 => match args[1].parse::<i32>() {
            Ok(t) => {
                // Starts on next line as the progress bar does not create a new line
                let x = get_prime_to_limit(t, true);
                println!("\nResults");
                for i in x {
                    println!("{}", i)
                }
            }
            Err(_) => {
                println!("Limit must be a integer (up to 2^32)")
            }
        },
        _ => {
            if args.contains(&"-npb".to_string()) | args.contains(&"--no-progress-bar".to_string())
            {
                match args[1].parse::<i32>() {
                    Ok(t) => {
                        for i in get_prime_to_limit(t, false) {
                            println!("{}", i)
                        }
                    }
                    Err(_) => {
                        println!("Limit must be a integer (up to 2^32)")
                    }
                }
            } else {
                println!("Invalid Usage \n{}", HELP_TEXT)
            }
        }
    }
}
