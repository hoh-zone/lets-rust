pub fn shortest_vec<'a>(x: &'a Vec<u32>, y: &'a Vec<u32>) -> &'a Vec<u32> {
    if x.len() <= y.len() {
        x
    } else {
        y
    }
}

pub fn shortest_vec_generic<'a, T>(x: &'a Vec<T>, y: &'a Vec<T>) -> &'a Vec<T> {
    if x.len() <= y.len() {
        x
    } else {
        y
    }
}

pub struct ServerConfig<'a> {
    pub host: &'a str,
    pub port: u32,
    pub db_url: &'a str,
}

impl<'a> ServerConfig<'a> {
    pub fn new(host: &'a str, port: u32, db_url: &'a str) -> Self {
        ServerConfig { host, port, db_url }
    }
}