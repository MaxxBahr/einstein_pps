use std::vec;

#[derive(PartialEq)]
enum Color{
    Unknown,
    Red,
    Yellow,
    Green,
    White,
    Blue,    
}

#[derive(PartialEq)]
enum Nations {
    Unknown,
    German,
    Norwegian,
    Danish,
    Swedish,
    British
}

#[derive(PartialEq)]
enum Drinks{
    Unknown,
    Coffee,
    Water,
    Milk,
    Tea,
    Beer
}

#[derive(PartialEq)]
enum Animal {
    Unknown,
    Dog,
    Fish,
    Horse,
    Cat,
    Bird
}

#[derive(PartialEq)]
enum Cigarettes{
    Unknown,
    Rothmanns,
    Marlboro,
    Pallmall,
    Winfield,
    Dunhill
}

#[derive(PartialEq)]
struct Person{
    housecolor: Color,
    nation: Nations,
    drink: Drinks,
    animal: Animal,
    cigarette: Cigarettes,
}

impl Person {
    // Constructor with options to meet constraints if information is unknown
    fn new(housecolor: Option<Color>, nation: Option<Nations>, drink: Option<Drinks>, animal: Option<Animal>, cigarette: Option<Cigarettes>) -> Person{
        let housecolor = if let Some(color) = housecolor{
            color
        }else{Color::Unknown};

        let nation = if let Some(nation) = nation {
            nation
        }else{Nations::Unknown};

        let drink = if let Some(drink) = drink {
            drink
        }else{Drinks::Unknown};

        let animal = if let Some(animal) = animal{
            animal
        }else{Animal::Unknown};

        let cigarette = if let Some(cigarette) = cigarette{
            cigarette
        }else{Cigarettes::Unknown};

        Person { 
            housecolor, 
            nation, 
            drink, 
            animal, 
            cigarette
        }
    }

    fn setColor(&mut self, color: Color){
        self.housecolor = color;
    }

    fn setNation(&mut self, nation: Nations){
        self.nation = nation;
    }

    fn setDrink(&mut self, drink: Drinks){
        self.drink = drink;
    }

    fn setAnimal(&mut self, animal: Animal){
        self.animal = animal;
    }

    fn setCigarette(&mut self, cigarette: Cigarettes){
        self.cigarette = cigarette;
    }
}

fn color_check(persons: &mut Vec<Person>) {
    let mut colors: Vec<Color> = vec![Color::Unknown, Color::Yellow, Color::Green, Color::Red, Color::White, Color::Blue];
    // Check the already chosen colors and assign the leftovers
    for person in persons{
        if person.housecolor != Color::Unknown{
            colors.retain(|x| *x != person.housecolor);
        }else{
            person.setColor(colors.pop().unwrap());
        }
    }
}

fn nation_check(persons: &mut Vec<Person>){
    let mut nations: Vec<Nations> = vec![Nations::Unknown, Nations::German, Nations::Swedish, Nations::British, Nations::Norwegian, Nations::Danish];
    //Check the already known nations and assign leftovers
    for person in persons{
        if person.nation != Nations::Unknown{
            nations.retain(|x| *x != person.nation);
        }else{
            person.setNation(nations.pop().unwrap());
        }
    }
}

fn drink_check(persons: &mut Vec<Person>){
    let mut drinks: Vec<Drinks> = vec![Drinks::Unknown, Drinks::Coffee, Drinks::Water, Drinks::Milk, Drinks::Tea, Drinks::Beer];
    //Check the already known drinks and assign leftovers
    for person in persons{
        if person.drink != Drinks::Unknown{
            drinks.retain(|x| *x != person.drink);
        }else{
            person.setDrink(drinks.pop().unwrap());
        }
    }

}

fn animal_check(persons: &mut Vec<Person>){
    let mut animals: Vec<Animal> = vec![Animal::Unknown, Animal::Dog, Animal::Fish, Animal::Horse, Animal::Cat, Animal::Bird];
    //Check the already known animals and assign leftovers
    for person in persons {
        if person.animal != Animal::Unknown{
            animals.retain(|x| *x != person.animal);
        }else{
            person.setAnimal(animals.pop().unwrap());
        }
    }
}

fn cigarette_check(persons: &mut Vec<Person>){
    let mut cigarettes: Vec<Cigarettes> = vec![Cigarettes::Unknown, Cigarettes::Rothmanns, Cigarettes::Marlboro, Cigarettes::Pallmall, Cigarettes::Winfield, Cigarettes::Dunhill];
    //Check the already known cigarettes and assign leftovers
    for person in persons{
        if person.cigarette != Cigarettes::Unknown{
            cigarettes.retain(|x| *x != person.cigarette);
        }else{
            person.setCigarette(cigarettes.pop().unwrap());
        }
    }
}

fn checker(persons: &mut Vec<Person>){
    color_check(persons);
    nation_check(persons);
    drink_check(persons);
    animal_check(persons);
    cigarette_check(persons);
}

fn main() {
    let person1 = Person::new(None, None, None, None, None);
    let person2 = Person::new(None, None, None, None, None);
    let person3 = Person::new(None, None, None, None, None);
    let person4 = Person::new(None, None, None, None, None);
    let person5 = Person::new(None, None, None, None, None);
    let mut persons: Vec<Person> = Vec::new();
    persons.push(person1);
    persons.push(person2);
    persons.push(person3);
    persons.push(person4);
    persons.push(person5);
    checker(&mut persons);
}
