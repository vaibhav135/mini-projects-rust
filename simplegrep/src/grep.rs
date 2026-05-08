pub fn grep(searchstring: String, content: &String) -> String {
    let lines = content.lines();
    let mut grepped_lines = String::from("");
    for line in lines {
        if let Some(_) = line.find(&searchstring) {
            grepped_lines += &format!("{}\n", line.to_string());
        }
    }

    grepped_lines
}
