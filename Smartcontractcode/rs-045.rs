pub trait Animal {
    fn sound(&self) -> &str;
}

pub struct Cat;
pub struct Dog;

impl Animal for Dog {
    fn sound(&self) -> &str {
        "woof"
    }
}

impl Animal for Cat {
    fn sound(&self) -> &str {
        "meow"
    }
}

pub fn static_dispatch<A: Animal>(a: &A) -> &str {
    a.sound()
}

pub fn call_static() -> [String; 2] {
    [static_dispatch(&Cat).into(), static_dispatch(&Dog).into()]
}

pub fn dynamic_dispatch(a: &dyn Animal) -> &str {
    a.sound()
}

pub fn call_dynamic(animal_str: &str) -> String {
    let animal: &dyn Animal = match animal_str {
        "cat" => &Cat,
        _ => &Dog,
    };
    
    dynamic_dispatch(animal).into()
}