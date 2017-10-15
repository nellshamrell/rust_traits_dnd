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

fn main() {
    let my_dwarf = Dwarf { name: String::from("Nell Grimly") };
    println!("Here is the constitution bonus for Nell Grimly, the dwarf: {}", my_dwarf.constitution_bonus());

    let my_elf = Elf { name: String::from("Nell Lightfooted") };
    println!("Here is the constitution bonus for Nell Lightfooted, the elf: {}", my_elf.constitution_bonus());


    let my_half_orc = HalfOrc { name: String::from("Nell Heavyfooted") };
    println!("Here is the constitution bonus for Nell Heavyfooted, the half-orc: {}", my_half_orc.constitution_bonus());
}
