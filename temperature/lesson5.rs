fn main() {
    let log = create_log();

    print_log(&log);

    let avg_temp = average_temperature(&log);
    println!("Average temperature: {:.2}°C", avg_temp);

    let dates = extract_dates(&log);
    println!("Dates in the log: {:?}", dates);

    let (max_date, max_temp) = find_max_temperature(&log);
    println!("The highest temperature {}°C was in {}", max_temp, max_date);
}

fn create_log() -> [(&'static str, f32); 5] {
    [
        ("2023-08-01", 23.5),
        ("2023-08-02", 25.0),
        ("2023-08-03", 22.1),
        ("2023-08-04", 26.3),
        ("2023-08-05", 24.7),
    ]
}

fn print_log(log: &[(&str, f32)]) {
    for l in log {
        println!("Data: {}, Temperature: {}", l.0, l.1);
    }
}

fn average_temperature(log: &[(&str, f32)]) -> f32 {
    let sum = log.iter().map(|l| l.1).sum::<f32>();
    sum / log.len() as f32
}

fn extract_dates<'a>(log: &'a [(&'a str, f32)]) -> Vec<&'a str> {
    let date = log.iter().map(|l| l.0).collect::<Vec<&str>>();
    date
}

fn find_max_temperature<'a>(log: &'a [(&'a str, f32)]) -> (&'a str, f32) {
    let max_entry = log
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    *max_entry
}
