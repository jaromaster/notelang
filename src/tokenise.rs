pub mod tokenise {

    /// split lines of ntlang-file into vector of tokens
    pub fn tokenise(lines: Vec<String>) -> Vec<String> {
        let mut result_vec = Vec::with_capacity(1024);

        let mut token = String::new();

        for l in lines.iter() {
            for c in l.chars() {

                if c.is_whitespace() {
                    result_vec.push(String::from(&token));
                    token.clear();
                    result_vec.push(String::from(c));
                    continue;
                }

                token.push(c);
            }
            result_vec.push(String::from(&token));
            token.clear();
            result_vec.push("\n".to_string());
        }

        if token.len() > 0 {
            result_vec.push(token);
        }

        return result_vec;
    }
}