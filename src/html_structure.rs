pub mod html_structure {

    /// build html structure around (html) content after transpiling
    pub fn build_html_structure(result_vec: &Vec<String>, file_name: &str) -> Vec<String> {
        let mut result_vec = result_vec.to_owned();

        // doctype
        result_vec.insert(0, "<!DOCTYPE html><html>".to_string());
        
        // head
        result_vec.insert(1, generate_head(file_name));

        // body
        result_vec.insert(2, "<body>".to_string());
        result_vec.push("</body>".to_string());
        result_vec.push("</html>".to_string());

        return result_vec.to_owned();
    }

    /// generate html head containing style and title
    fn generate_head(title: &str) -> String {
        let head = format!("<head><title>{}</title><style>{}</style></head>", 
            title, 
            generate_table_style());

        return head.to_owned();
    }

    /// generate css styling for html tables
    fn generate_table_style() -> String {
        let style_string = "
        table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
            padding: 4px;
        }
        th {
            background-color: lightgrey;
        }";

        return style_string.to_owned();
    }
}