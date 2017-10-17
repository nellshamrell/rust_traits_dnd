pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

struct Dwarf {
   name: String
}

impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

struct HalfOrc {
    name: String
}

impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}

struct Elf {
    name: String
}

impl Constitution for Elf {
}

struct HalfElf {
    name: String
}

pub trait Elvish {
}

impl Elvish for Elf {
}

impl Elvish for HalfElf {
}

pub fn understand_elvish<T: Elvish>(character: T) -> String {
    String::from("yes")
}

pub trait Cast {
    fn cast(&self);
}

pub struct Cantrip {
}

impl Cast for Cantrip {
    fn cast(&self) {
        println!("I just cast a Cantrip");
    }
}

pub struct Transmutation {
}

impl Cast for Transmutation {
    fn cast(&self) {
        println!("I just cast a Transmutation");
    }
}

pub struct Enchantment {
}

impl Cast for Enchantment {
    fn cast(&self) {
        println!("I just cast an Enchantment");
    }
}

pub struct Necromancy {
}

impl Cast for Necromancy {
    fn cast(&self) {
        println!("I just cast a Necromancy");
    }
}

struct Spellbook {
    pub spells: Vec<Box<Cast>>,
}

impl Spellbook {
    pub fn run(&self) {
        for spell in self.spells.iter() {
            spell.cast();
        }
    }
}

fn main() {
    let my_dwarf = Dwarf { name: String::from("Nell Grimly") };
    println!("Here is the constitution bonus for Nell Grimly, the dwarf: {}", my_dwarf.constitution_bonus());

    let my_elf = Elf { name: String::from("Nell Lightfooted") };
    println!("Here is the constitution bonus for Nell Lightfooted, the elf: {}", my_elf.constitution_bonus());

    let my_half_orc = HalfOrc { name: String::from("Nell Heavyfooted") };
    println!("Here is the constitution bonus for Nell Heavyfooted, the half-orc: {}", my_half_orc.constitution_bonus());

    let my_half_elf = HalfElf { name: String::from("Nell Halffooted") };
    let half_elf_understands_elvish = understand_elvish(my_half_elf);
    println!("Does the half elf understand elvish? {}", half_elf_understands_elvish);

    let elf_understands_elvish = understand_elvish(my_elf);
    println!("Does the elf understand elvish? {}", elf_understands_elvish);

//    let half_orc_understands_elvish = understand_elvish(my_half_orc);
//    println!("Does the half orc understand elvish? {}", half_orc_understands_elvish);

    let spell_book = Spellbook {
                         spells: vec![
                             Box::new(Cantrip {}),
                             Box::new(Transmutation {}),
                             Box::new(Enchantment {}),
                             Box::new(Necromancy {}),
                         ],
                     };
     spell_book.run();

//    println!("Here is the Spellbook: {:?}", spell_book)
}
