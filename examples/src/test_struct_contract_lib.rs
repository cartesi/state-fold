use ethers::types::U256;

#[derive(Debug, Clone, Default)]
struct Person {
    name: String,
    age: U256,
}

#[derive(Debug, Default, Clone)]
pub struct ContractCtx {
    person: Person,
}

impl ContractCtx {
    pub fn modify_name(self, name: String) -> Self {
        let mut person = self.person.clone();
        person.name = name.clone();
        println!("Name after modify: {:?}", person.name);
        Self { person: person }
    }
    pub fn modify_age(&self, age: U256) -> Self {
        let mut person = self.person.clone();
        person.age = age;
        println!("Age after modify: {:?}", person.age);
        Self { person: person }
    }
}
