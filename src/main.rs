#[derive(Debug)]
struct Character {
    name: String,
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8
}

fn main() {
    let my_character = Character {
        name: String::from("Rian Everstar"),
        strength: 8,
        dexterity: 8,
        constitution: 9,
        intelligence: 5,
        wisdom: 5,
        charisma: 7
    };

    println!("Here is my character {:?}", my_character)
}
