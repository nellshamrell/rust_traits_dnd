/*
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
*/

pub trait Constitution {
    fn constitution_bonus(&self) -> u8;
}

struct Dwarf {
   name: String
}

impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

struct Elf {
    name: String
}

impl Constitution for Elf {
    fn constitution_bonus(&self) -> u8 {
        0
    }

}

fn main() {
    let my_dwarf = Dwarf { name: String::from("Nell Grimly") };
    println!("Here is the constitution bonus for Nell Grimly, the dwarf: {}", my_dwarf.constitution_bonus());

    let my_elf = Elf { name: String::from("Nell Lightfooted") };
    println!("Here is the constitution bonus for Nell Lightfooted, the elf: {}", my_elf.constitution_bonus());
/*
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
*/
}
