pub mod transpile {

    // notelang tags
    const HEADLINE: &str = ".h"; // -> <h1>
    const TEXT: &str = ".p"; // -> <p>
    const IMAGE: &str = ".img"; // -> <img>
    const LINK: &str = ".link"; // -> <a href="...">
    const TABLE: &str = ".tab"; // -> <table>
    const ROW: &str = ".row"; // -> <tr> containing <td>
    const HROW: &str = ".hrow"; // -> <tr> containing <th>

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

        // .link
        result_vec = transpile_hyperlink(&result_vec);

        // .table, .hrow, .row
        result_vec = transpile_table(&result_vec);

        // inline tags
        result_vec = transpile_bold(&result_vec); // * (bold)
        result_vec = transpile_italic(&result_vec); // # (italic)
        result_vec = transpile_mark(&result_vec); // _ (mark)

        // define html structure
        // TODO move to function
        result_vec.insert(0, "<!DOCTYPE html><html>".to_string());
        result_vec.insert(1, "<head>".to_string());
        // TODO maybe add title
        result_vec.insert(2, generate_table_style());
        result_vec.insert(3,"</head>".to_string());
        result_vec.insert(4, "<body>".to_string());
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

    /// convert link tags to a (href) html tag
    fn transpile_hyperlink(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();

        let mut opened_tag = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == LINK {
                result_tokens[i] = format!("<a href=\"{}\">", tokens[i+2]);
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == "\n" && opened_tag {
                result_tokens[i] = "</a>".to_string();
                opened_tag = false;
            }
        }

        return result_tokens;
    }

    /// construct html table from table tags
    fn transpile_table(tokens: &Vec<String>) -> Vec<String> {
        let mut result_tokens = tokens.to_owned();
        let delimiter = ",";

        // table tags
        let mut opened_tag = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == TABLE && !opened_tag {
                result_tokens[i] = "<table>".to_string();
                opened_tag = true;
            }
            else if tokens.get(i).unwrap() == TABLE && opened_tag {
                result_tokens[i] = "</table>".to_string();
                opened_tag = false;
            }
        }

        if opened_tag {
            panic!("invalid number of .tab tags (maybe not closing?)");
        }

        // header rows
        opened_tag = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == HROW && i < tokens.len()-2 {
                result_tokens[i] = "<tr>".to_string();
                opened_tag = true;

                let elements: Vec<String> = tokens[i+2].split(delimiter)
                    .map(|el| el.to_string()).collect();

                let mut row = String::new();
                for e in elements.iter() {
                    row.push_str(format!("<th>{}</th>", e).as_str());
                }
                result_tokens[i+2] = row;
            }
            else if tokens.get(i).unwrap() == "\n" && opened_tag {
                result_tokens[i] = "</tr>".to_string();
                opened_tag = false;
            }
        }

        // normal rows
        opened_tag = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == ROW && i < tokens.len()-2 {
                result_tokens[i] = "<tr>".to_string();
                opened_tag = true;

                let elements: Vec<String> = tokens[i+2].split(delimiter)
                    .map(|el| el.to_string()).collect();

                let mut row = String::new();
                for e in elements.iter() {
                    row.push_str(format!("<td>{}</td>", e).as_str());
                }
                result_tokens[i+2] = row;
            }
            else if tokens.get(i).unwrap() == "\n" && opened_tag {
                result_tokens[i] = "</tr>".to_string();
                opened_tag = false;
            }
        }

        return result_tokens;
    }

    /// generate css styling for html tables
    fn generate_table_style() -> String {
        let style_string = "
        <style>
            table, th, td {
                border: 1px solid black;
                border-collapse: collapse;
                padding: 4px;
            }
        </style>";

        return style_string.to_owned();
    }

}