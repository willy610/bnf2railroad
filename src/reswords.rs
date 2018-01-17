use std::collections::HashSet;
use read_file_content;
pub struct Reswords {
    pub words: HashSet<String>,
}
impl Reswords {
    pub fn new() -> Reswords {
        Reswords { words: HashSet::new() }
    }
    /*..................................................*/
    pub fn set_up_once(&mut self, file_name_keywords: String) -> Result<String, String> {
        if file_name_keywords.len() == 0 {
            let keys: Vec<&str> = vec![
                "abstract",
                "alignof",
                "as",
                "become",
                "box",
                "break",
                "const",
                "continue",
                "crate",
                "do",
                "else",
                "enum",
                "extern",
                "false",
                "final",
                "fn",
                "for",
                "if",
                "impl",
                "in",
                "let",
                "loop",
                "macro",
                "match",
                "mod",
                "move",
                "mut",
                "offsetof",
                "override",
                "priv",
                "proc",
                "pub",
                "pure",
                "ref",
                "return",
                "Self",
                "self",
                "sizeof",
                "static",
                "struct",
                "super",
                "trait",
                "true",
                "type",
                "typeof",
                "unsafe",
                "unsized",
                "use",
                "virtual",
                "where",
                "while",
                "yield",
            ];
            for a_key in keys {
                self.words.insert(a_key.to_string());
            }
            return Result::Ok("".to_string());
        } else {
            let tva_result: Result<String, String>;
            //          let mut split;
            tva_result = match read_file_content(&file_name_keywords) {
                Err(why) => Result::Err(why),
                Ok(content) => {
                    for a_key in content.lines() {
                        self.words.insert(a_key.to_string());
                    }
                    Result::Ok("".to_string())
                }
            };
            return tva_result;
            //            if tva_result.is_err() {
            //                println!("tva_result={:?}", tva_result.unwrap_err())
            //            }
        }
    }
    /*..................................................*/
    pub fn is_res_word(&self, _the_word: String) -> bool {
        self.words.contains(&_the_word)
    }
}
