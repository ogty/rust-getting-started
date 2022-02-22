use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, Display};
use std::env;

extern crate uuid;
use uuid::Uuid;

trait Initializer {
    fn write(&mut self, path: &str);
}

struct ArticleInitializer {
    title: String,
    topics: Vec<String>,
}

impl Default for ArticleInitializer {
    fn default() -> ArticleInitializer {
        ArticleInitializer {
            title: String::from(""),
            topics: Vec::new(),
        }
    }
}

impl Initializer for ArticleInitializer {
    fn write(&mut self, path: &str) {
        let path: &Path = Path::new(&path);
        let display: Display = path.display();
        let mut template: String = String::from("");

        if self.title != String::from("") {
            let topics_length: usize = self.topics.len();
            if topics_length > 0 {
                if topics_length > 2 {
                    let before_last: usize = topics_length - 1;
                    let topics: String = self.topics[..before_last].join(", ");
                    template = format!(
                        "---\ntitle: {}\nemoji: 🐒\ntype: tech\ntopics: [{}{}]\npublished: false\n---", 
                        self.title, topics, self.topics[before_last]
                    );
                } else {
                    template = format!(
                        "---\ntitle: {}\nemoji: 🐒\ntype: tech\ntopics: [{}]\npublished: false\n---", 
                        self.title, self.topics[0]
                    );
                }
            } else {
                template = format!("---\ntitle: {}\nemoji: 🐒\ntype: tech\ntopics: []\npublished: false\n---", self.title);
            }
        } else {
            template = String::from("---\ntitle: \nemoji: 🐒\ntype: tech\ntopics: []\npublished: false\n---");
        }
    
        let mut file: File = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    
        match file.write_all(template.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }
    }
}

fn main() {
    let id: Uuid = Uuid::new_v4();
    let path: String = format!("./articles/{}.md", id);
    let mut args: Vec<String> = env::args().collect();
    args.push(String::from(""));

    let mut initializer: ArticleInitializer = ArticleInitializer{ ..Default::default() };
    initializer.title = args[1].to_string();
    initializer.topics = (&args[2..]).to_vec();
    initializer.write(&path);
}
