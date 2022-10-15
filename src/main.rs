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

fn main() {
    println!("Hello, world!");
}
