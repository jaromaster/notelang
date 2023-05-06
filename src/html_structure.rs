pub mod html_structure {

    /// build html structure around (html) content after transpiling
    pub fn build_html_structure(result_vec: &Vec<String>) -> Vec<String> {
        let mut result_vec = result_vec.to_owned();

        result_vec.insert(0, "<!DOCTYPE html><html>".to_string());
        result_vec.insert(1, "<head>".to_string());
        // TODO maybe add title
        result_vec.insert(2, generate_table_style());
        result_vec.insert(3,"</head>".to_string());
        result_vec.insert(4, "<body>".to_string());
        result_vec.push("</body>".to_string());
        result_vec.push("</html>".to_string());

        return result_vec.to_owned();
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
            th {
                background-color: lightgrey;
            }
        </style>";

        return style_string.to_owned();
    }
}