pub mod my;

pub mod a {
    pub fn f() -> String {
        "a".to_string()
    }
    
    pub mod b {
        pub fn g() -> String {
            "b".to_string()
        }
    }
    
    use super::my;
    
    pub fn my_a() -> String {
        my::a::name()
    }
    
    pub fn my_b() -> String {
        my::b::name()
    }
}