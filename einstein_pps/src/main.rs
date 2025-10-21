#[derive(PartialEq, Clone, Copy, Debug)]
enum Color{
    Unknown,
    Red,
    Yellow,
    Green,
    White,
    Blue,    
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Nations {
    Unknown,
    German,
    Norwegian,
    Danish,
    Swedish,
    British
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Drinks{
    Unknown,
    Coffee,
    Water,
    Milk,
    Tea,
    Beer
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Animal {
    Unknown,
    Dog,
    Fish,
    Horse,
    Cat,
    Bird
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cigarettes{
    Unknown,
    Rothmanns,
    Marlboro,
    Pallmall,
    Winfield,
    Dunhill
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Person{
    position: Option<i16>,
    housecolor: Option<Color>,
    nation: Option<Nations>,
    drink: Option<Drinks>,
    animal: Option<Animal>,
    cigarette: Option<Cigarettes>,
}

impl Person {
    // Constructor with options to meet constraints if information is unknown
    fn new(position: Option<i16>, housecolor: Option<Color>, nation: Option<Nations>, drink: Option<Drinks>, animal: Option<Animal>, cigarette: Option<Cigarettes>) -> Person{
        Person { 
            position,
            housecolor, 
            nation, 
            drink, 
            animal, 
            cigarette
        }
    }

    fn setPosition(&mut self, position: i16){
        self.position = Some(position);
    }

    fn setColor(&mut self, color: Color){
        self.housecolor = Some(color);
    }

    fn setNation(&mut self, nation: Nations){
        self.nation = Some(nation);
    }

    fn setDrink(&mut self, drink: Drinks){
        self.drink = Some(drink);
    }

    fn setAnimal(&mut self, animal: Animal){
        self.animal = Some(animal);
    }

    fn setCigarette(&mut self, cigarette: Cigarettes){
        self.cigarette = Some(cigarette);
    }
}

fn is_filled(persons: &mut Vec<Person>) -> bool{
    persons.iter().all(|person|
        person.position.is_some()
        && person.housecolor.is_some()
        && person.nation.is_some()
        && person.animal.is_some()
        && person.drink.is_some()
        && person.cigarette.is_some()
    )
}

fn is_neighbor(pos1: i16, pos2: i16) -> bool {
    (pos1 - pos2).abs() == 1
}

fn check_constraints(persons: &mut Vec<Person>){
    // keep track if changes are made and stop once done or after 100 iterations
    let mut progress: bool = true;
    let mut iterations = 0;

    // Rule 5 - 8, 11: Applied on instantiation
    while progress && iterations < 100 {
        progress = false;
        iterations += 1;

        // Rule 9: Der Besitzer des grünen Hauses trinkt Kaffee
        for person in persons.iter_mut() {
            match (person.housecolor, person.drink){
                (Some(Color::Green), None) =>{
                    person.setDrink(Drinks::Coffee);
                }
                (None, Some(Drinks::Coffee)) =>{
                    person.setColor(Color::Green);
                }
                _=>{}
            }
        }

        // Rule 10: Der Winfield-Raucher trinkt gerne Bier
        for person in persons.iter_mut() {
            match (person.cigarette, person.drink){
                (Some(Cigarettes::Winfield), None) => {
                    person.setDrink(Drinks::Beer);
                }
                (None, Some(Drinks::Beer)) =>{
                    person.setCigarette(Cigarettes::Winfield);
                }
                _=>{}
            }
        }

        // Rule 13: Der Besitzer des gelben Hauses raucht Dunhill
        for person in persons.iter_mut() {
            match(person.housecolor, person.cigarette) {
                (Some(Color::Yellow), None) =>{
                    person.setCigarette(Cigarettes::Dunhill);
                }
                (None, Some(Cigarettes::Dunhill)) =>{
                    person.setColor(Color::Yellow);
                }
                _=>{}
            }
        }
        
        // Rule 14: Die Person, die Pall Mall raucht, hält einen Vogel
        for person in persons.iter_mut() {
            match (person.cigarette, person.animal) {
                (Some(Cigarettes::Pallmall), None) =>{
                    person.setAnimal(Animal::Bird);
                }
                (None, Some(Animal::Bird)) => {
                    person.setCigarette(Cigarettes::Pallmall);
                }
                _=>{}
            }
        }

        // Rule 15: Der Mann, der im mittleren Haus wohnt, trinkt Milch
        for person in persons.iter_mut() {
            match (person.position, person.drink) {
                (Some(3), None) => {
                    person.setDrink(Drinks::Milk);
                }
                (None, Some(Drinks::Milk)) => {
                    person.setPosition(3);
                }
                _=>{}
            }
        }

        // Rule 16: Das grüne Haus steht unmittelbar links vom weißen Haus
        for i in 0..persons.len() {
            if let Some(pos_green) = persons[i].position {
                for j in 0..persons.len() {
                    if persons[j].housecolor == Some(Color::White) && persons[j].position.is_none() {
                        if pos_green < 5 {
                            persons[j].setPosition(pos_green + 1);
                            progress = true;
                        }
                    }
                }
            }
        }

        // Rule 17: Der Marlboro-Raucher wohnt neben dem, der eine Katze hält
        for i in 0..persons.len() {
            if persons[i].cigarette == Some(Cigarettes::Marlboro) {
                for j in 0..persons.len() {
                    if let (Some(pos_i), Some(pos_j)) = (persons[i].position, persons[j].position) {
                        if is_neighbor(pos_i, pos_j) && persons[j].animal == Some(Animal::Cat) {
                            progress = true;
                        }
                    }
                }
            }
        }


        // Rule 18: Der Marlboro-Raucher hat einen Nachbarn, der Wasser trinkt
        for person in persons.iter_mut() {
            if person.cigarette == Some(Cigarettes::Marlboro){
                // Check if lesser or higher as Drink set
                progress = true;
            }
        }

        // Rule 19: Der Mann, der ein Pferd hält, wohnt neben dem, der Dunhill raucht
        for person in persons.iter_mut() {
            if person.cigarette == Some(Cigarettes::Dunhill){
                // Check if lesser or higher has Animal set
                progress = true;
            }
        }
        print!("{}[2J", 27 as char);
        println!("----------{iterations}----------");
    }
    if !is_filled(persons){
        println!("Not all values set!");
    }
}

fn main() {
    // Check all constraints instead of assigning leftovers
    let person1 = Person::new(None, Some(Color::Red), Some(Nations::British), None, None, None);
    let person2 = Person::new(None, None, Some(Nations::Swedish), None, Some(Animal::Dog), None);
    let person3 = Person::new(None, None, Some(Nations::Danish), Some(Drinks::Tea), None, None);
    let person4 = Person::new(None, None, Some(Nations::German), None, None, Some(Cigarettes::Rothmanns));
    let person5 = Person::new(Some(1), None, Some(Nations::Norwegian), None, None, None);
    let mut persons: Vec<Person> = Vec::new();
    persons.push(person1);
    persons.push(person2);
    persons.push(person3);
    persons.push(person4);
    persons.push(person5);
    check_constraints(&mut persons);
    println!("{:?}", persons);

}