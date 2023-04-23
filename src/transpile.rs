pub mod transpile {

    // notelang tags
    const HEADLINE: &str = ".h";


    /// transpile token-vector into valid html (as string-vec)
    pub fn transpile(tokens: Vec<String>) -> Vec<String> {
        let mut result_vec = tokens.to_vec();

        // define html structure
        result_vec.insert(0, "<!DOCTYPE html><html>".to_string());
        result_vec.insert(1, "<head>".to_string());
        // TODO maybe add title
        result_vec.insert(2,"</head>".to_string());
        result_vec.insert(3, "<body>".to_string());
        result_vec.push("</body>".to_string());
        result_vec.push("</html>".to_string());
        

        // .h
        result_vec = transpile_headlines(&result_vec);

        return result_vec;
    }

    /// convert headline tags to html tags
    fn transpile_headlines(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();

        let mut opened_tag = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == HEADLINE {
                result_tokens[i] = "<h1>".to_string();
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == "\n" && opened_tag {
                result_tokens[i] = "</h1>".to_string();
                opened_tag = false;
            }
        }

        return result_tokens.to_owned();
    }
}