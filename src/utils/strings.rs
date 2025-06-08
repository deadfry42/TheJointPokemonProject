pub fn concatenate_strings(str1: &'static str, str2: &'static str) -> &'static str {
    let mut final_string = String::new().to_owned();

    final_string.push_str(str1);
    final_string.push_str(str2);

    Box::leak(Box::new(final_string))
}
