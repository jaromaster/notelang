pub mod transpile {

    // notelang tags
    const HEADLINE: &str = ".h"; // -> <h1>
    const TEXT: &str = ".p"; // -> <p>
    const IMAGE: &str = ".img"; // -> <img>

    // notelang inline tags
    const BOLD: &str = "*"; // -> <strong>
    const ITALIC: &str = "#"; // -> <i>
    const MARK: &str = "_"; // -> <mark>


    /// transpile token-vector into valid html (as string-vec)
    pub fn transpile(tokens: Vec<String>) -> Vec<String> {
        let mut result_vec = tokens.to_vec();
 
        // .h
        result_vec = transpile_headlines(&result_vec);

        // .p
        result_vec = transpile_text(&result_vec);

        // .img
        result_vec = transpile_image(&result_vec);

        // inline tags
        result_vec = transpile_bold(&result_vec); // * (bold)
        result_vec = transpile_italic(&result_vec); // # (italic)
        result_vec = transpile_mark(&result_vec); // _ (mark)

        // define html structure
        result_vec.insert(0, "<!DOCTYPE html><html>".to_string());
        result_vec.insert(1, "<head>".to_string());
        // TODO maybe add title
        result_vec.insert(2,"</head>".to_string());
        result_vec.insert(3, "<body>".to_string());
        result_vec.push("</body>".to_string());
        result_vec.push("</html>".to_string());

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

        return result_tokens;
    }

    /// convert text tags to html tags
    fn transpile_text(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();

        let mut opened_tag = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == TEXT {
                result_tokens[i] = "<p>".to_string();
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == "\n" && opened_tag {
                result_tokens[i] = "</p>".to_string();
                opened_tag = false;
            }
        }

        return result_tokens;
    }

    /// convert image tags to html tags
    fn transpile_image(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = Vec::with_capacity(tokens.capacity());

        let mut i = 0;
        while i < tokens.len()-2 {
            if tokens[i] == IMAGE {
                result_tokens.push(format!("<img src=\"{}\" alt=\"{}\">",tokens[i+2], tokens[i+2]));
                i += 2;
            }
            else {
                result_tokens.push(tokens[i].to_owned());
            }
            i+=1;
        }

        for j in i..tokens.len() {
            result_tokens.push(tokens[j].to_owned());
        }

        return result_tokens;
    }

    /// convert bold tags to html tags
    fn transpile_bold(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();

        let mut opened_tag = false;
        let mut count = 0;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == BOLD && !opened_tag {
                result_tokens[i] = "<strong>".to_string();
                count += 1;
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == BOLD && opened_tag {
                result_tokens[i] = "</strong>".to_string();
                count -= 1;
                opened_tag = false;
            }
        }

        if count != 0 {
            panic!("invalid number of bold tags!");
        }

        return result_tokens;
    }

    /// convert italic tags to html tags
    fn transpile_italic(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();

        let mut opened_tag = false;
        let mut count = 0;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == ITALIC && !opened_tag {
                result_tokens[i] = "<i>".to_string();
                count += 1;
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == ITALIC && opened_tag {
                result_tokens[i] = "</i>".to_string();
                count -= 1;
                opened_tag = false;
            }
        }

        if count != 0 {
            panic!("invalid number of italic tags!");
        }

        return result_tokens;
    }

    /// convert mark tags to html tags
    fn transpile_mark(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();

        let mut opened_tag = false;
        let mut count = 0;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == MARK && !opened_tag {
                result_tokens[i] = "<mark>".to_string();
                count += 1;
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == MARK && opened_tag {
                result_tokens[i] = "</mark>".to_string();
                count -= 1;
                opened_tag = false;
            }
        }

        if count != 0 {
            panic!("invalid number of mark tags!");
        }

        return result_tokens;
    }
}