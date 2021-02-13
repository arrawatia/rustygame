#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("welcome {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("new member {}", self.name),
            VisitorAction::Refuse => println!("go away, {}", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    println!("Hello, what is your name!");

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 30),
        Visitor::new(
            "steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Please take off your shoes"),
            },
            12,
        ),
        Visitor::new("a", VisitorAction::Refuse, 2),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("Final list of visitors: {:#?}", visitor_list);
}
