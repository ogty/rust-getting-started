use rand::prelude::SliceRandom;

pub trait MessageTemplates {
    fn general_message(&mut self);
}

pub struct Greeting {
    pub name: &'static str,
}

impl Default for Greeting {
    fn default() -> Self {
        Greeting {
            name: "",
        }
    }
}

impl MessageTemplates for Greeting {
    fn general_message(&mut self) {
        let messages = vec![
            "Hello",
            "Hi",
            "Hey",
            "Howdy",
            "Greetings",
            "Salutations",
            "Good day",
            "Good evening",
            "Good morning",
        ];

        let message = messages.choose(&mut rand::thread_rng()).unwrap();
        if self.name == "" {
            println!("{}", message);
        } else {
            println!("{}: {}", self.name, message);
        }
    }
}
