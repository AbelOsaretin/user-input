use std::io;

#[derive(Debug)]
enum PowerMenu {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn power_messages(input: PowerMenu) {
    match input {
        PowerMenu::Off => println!("Power Off"),
        PowerMenu::Sleep => println!("Going to Sleep"),
        PowerMenu::Reboot => println!("Rebooting"),
        PowerMenu::Shutdown => println!("Shutting Down"),
        PowerMenu::Hibernate => println!("Hibernating"),
    }
}

fn main() {
    loop {
        let mut user_input = String::new();

        println!("Welcome");
        println!("Please enter one of the menu below");
        println!(
            "{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n",
            PowerMenu::Off,
            PowerMenu::Sleep,
            PowerMenu::Reboot,
            PowerMenu::Shutdown,
            PowerMenu::Hibernate
        );

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to get input");

        let user_input = user_input.trim().to_lowercase();

        let menu_choice = match user_input.as_str() {
            "off" => PowerMenu::Off,
            "sleep" => PowerMenu::Sleep,
            "reboot" => PowerMenu::Reboot,
            "shutdown" => PowerMenu::Shutdown,
            "hibernate" => PowerMenu::Hibernate,
            _ => {
                println!("Invalid menu input");
                continue;
            }
        };
        println!("-------------------------------");
        println!("-------------------------------");
        power_messages(menu_choice);
        println!("-------------------------------");
        println!("-------------------------------");
    }
}
