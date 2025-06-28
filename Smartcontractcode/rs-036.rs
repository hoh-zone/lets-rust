pub trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
}

pub trait Fly {
    fn fly(&self) -> String;
}

pub struct Crow;

pub trait Bird: Animal + Fly {
    fn fly_and_make_sound(&self) -> String {
        format!("{}: {} {}", self.name(), self.sound(), self.fly())
    }
}

impl Animal for Crow {
    fn name(&self) -> &str {
        "crow"
    }
    
    fn sound(&self) -> &str {
        "caw!"
    }
    
}

impl Fly for Crow {
    fn fly(&self) -> String {
        "flap wings".to_string()
    }
}

impl Bird for Crow {}