use rand::Rng;
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
    fn new(name: &str, age: i8, action: VisitorAction) -> Self {
        Self {
            name: name.to_string(),
            action,
            age,
        }
    }
}

fn get_name() -> String {
    let mut buf = String::new();
    let msg = "Failed to read line!";
    stdin().read_line(&mut buf).expect(msg);
    buf.trim().to_string()
}

fn check_member_status() {
    let mut rng = rand::thread_rng();
    let mut members = vec![
        Visitor::new("gabriel", 30, VisitorAction::Accept),
        Visitor::new(
            "abby",
            28,
            VisitorAction::AcceptWithNote {
                note: "You seems tired".to_string(),
            },
        ),
        Visitor::new("ariana", 2, VisitorAction::Probation),
        Visitor::new("jeff", 60, VisitorAction::Refuse),
    ];
    loop {
        println!("Hello, what's your name?");
        let name = get_name();
        let lowercase = name.to_lowercase();
        let member = members.iter().find(|member| member.name == lowercase);
        match member {
            Some(member) => {
                // println!("{:?}", member);
                match &member.action {
                    VisitorAction::Accept => {
                        println!("Hello, {}. Welcome to the tree house!", member.name);
                        if member.age < 21 {
                            println!("No alcohol for you though!")
                        }
                    }
                    VisitorAction::AcceptWithNote { note } => {
                        println!(
                            "Hello, {}. Welcome to the tree house! ({})",
                            member.name, note
                        );
                        if member.age < 21 {
                            println!("No alcohol for you though!")
                        }
                    }
                    VisitorAction::Probation => {
                        println!(
                            "Hello, {}. Welcome to the tree house! You better watch yourself...",
                            member.name
                        )
                    }
                    VisitorAction::Refuse => {
                        println!("Get out of here {}! ", member.name)
                    }
                }
            }
            None => {
                if name.is_empty() {
                    println!("Bye bye!\n");
                    println!("The final list:");
                    println!("{:#?}", members);
                    break;
                } else {
                    println!("You are not allowed in {}!", name);
                    println!("I will remember you next time.");
                    let age = rng.gen_range(1..=80);
                    members.push(Visitor::new(
                        &name.to_lowercase(),
                        age,
                        VisitorAction::Probation,
                    ));
                }
            }
        }
    }
}

fn main() {
    check_member_status();
}
