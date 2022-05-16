pub fn error(line_num: u32, message: String) {
    report(line_num, String::from(""), message);
}

fn report(line_num: u32, location: String, message: String) {
    println!("[line: {}] Error{}: {}", line_num, location, message)
}
