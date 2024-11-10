pub mod funcs {
    pub fn fix_new_lines(input: &str) -> String {
        input.replace("\r\n", "\n")
    }
}
