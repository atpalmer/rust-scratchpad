use std::collections::HashMap;
use std::fmt;

trait Message {
    fn message(&self) -> String;
    fn name(&self) -> String;
}

impl fmt::Display for dyn Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} {}", self.message(), self.name());
    }
}

struct StructMessage {
    message: String,
    name: String,
}

impl StructMessage {
    fn new(map: &HashMap<String, String>) -> Box<dyn Message> {
        return Box::new(StructMessage {
            message: map.get("message").unwrap_or(&String::new()).to_string(),
            name: map.get("name").unwrap_or(&String::new()).to_string(),
        });
    }
}

impl Message for StructMessage {
    fn message(&self) -> String {
        return self.message.to_string();
    }

    fn name(&self) -> String {
        return self.name.to_string();
    }
}

struct HashMessage {
    data: HashMap<String, String>,
}

impl HashMessage {
    fn new(map: HashMap<String, String>) -> Box<dyn Message> {
        return Box::new(HashMessage { data: map });
    }
}

impl Message for HashMessage {
    fn message(&self) -> String {
        return self.data.get("message").unwrap_or(&String::new()).to_string();
    }

    fn name(&self) -> String {
        return self.data.get("name").unwrap_or(&String::new()).to_string();
    }
}

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("message"), String::from("hello"));
    map.insert(String::from("name"), String::from("world"));

    println!("Map: {:?}", map);

    let struct_message: Box<dyn Message> = StructMessage::new(&map);
    let hash_message: Box<dyn Message> = HashMessage::new(map);

    println!("HashMessage: {}", &hash_message);
    println!("StructMessage: {}", &struct_message);
}

