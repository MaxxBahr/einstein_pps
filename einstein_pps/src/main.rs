use std::vec;

#[derive(PartialEq, Clone, Copy)]
enum Color{
    Unknown,
    Red,
    Yellow,
    Green,
    White,
    Blue,    
}

#[derive(PartialEq, Clone, Copy)]
enum Nations {
    Unknown,
    German,
    Norwegian,
    Danish,
    Swedish,
    British
}

#[derive(PartialEq, Clone, Copy)]
enum Drinks{
    Unknown,
    Coffee,
    Water,
    Milk,
    Tea,
    Beer
}

#[derive(PartialEq, Clone, Copy)]
enum Animal {
    Unknown,
    Dog,
    Fish,
    Horse,
    Cat,
    Bird
}

#[derive(PartialEq, Clone, Copy)]
enum Cigarettes{
    Unknown,
    Rothmanns,
    Marlboro,
    Pallmall,
    Winfield,
    Dunhill
}

#[derive(PartialEq, Clone, Copy)]
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

fn check_constraints(persons: &mut Vec<Person>){
    // Rule 5 - 8, 11: Applied on instantiation
    loop{
        // Stop once all fields are filled
        if is_filled(persons){
            break;
        }
        // Rule 9: Der Besitzer des grünen Hauses trinkt Kaffee
        for person in persons {
            if person.housecolor == Some(Color::Green){
                person.setDrink(Drinks::Coffee);
            }
        }

        // Rule 10: Der Winfield-Raucher trinkt gerne Bier
        for person in persons {
            if person.cigarette == Some(Cigarettes::Winfield){
                person.setDrink(Drinks::Beer);
            }
        }

        // Rule 13: Der Besitzer des gelben Hauses raucht Dunhill
        for person in persons {
            if person.housecolor == Some(Color::Yellow){
                person.setCigarette(Cigarettes::Dunhill);
            }
        }
        
        // Rule 14: Die Person, die Pall Mall raucht, hält einen Vogel
        for person in persons {
            if person.cigarette == Some(Cigarettes::Pallmall){
                person.setAnimal(Animal::Bird);
            }
        }

        // Rule 15: Der Mann, der im mittleren Haus wohnt, trinkt Milch
        for person in persons {
            if person.position == Some(2){
                person.setDrink(Drinks::Milk);
            }
        }

        // Rule 16: Das grüne Haus steht unmittelbar links vom weißen Haus
        for person in persons {
            if person.housecolor == Some(Color::Green) && person.position.is_some(){
                let position = person.position.unwrap();
                for person in persons{
                    if person.housecolor == Some(Color::White){
                        person.position = Some(position + 1);
                    }
                }
            }
        }

        // Rule 17: Der Marlboro-Raucher wohnt neben dem, der eine Katze hält
        for person in persons {
            if person.cigarette == Some(Cigarettes::Marlboro){
                for person2 in persons{
                    if person2.animal == Some(Animal::Cat){
                        person.position = Some(person2.position.unwrap() -1);
                    }
                }
            }
        }

        // Rule 18: Der Marlboro-Raucher hat einen Nachbarn, der Wasser trinkt
        for person in persons {
            if person.cigarette == Some(Cigarettes::Marlboro){
                // Check if lesser or higher as Drink set
            }
        }

        // Rule 19: Der Mann, der ein Pferd hält, wohnt neben dem, der Dunhill raucht
        for person in persons {
            if person.cigarette == Some(Cigarettes::Dunhill){
                // Check if lesser or higher has Animal set
            }
        }
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
}