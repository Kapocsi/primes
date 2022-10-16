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
    println!("Hello, world!");
}
