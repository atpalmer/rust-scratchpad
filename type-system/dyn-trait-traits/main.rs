use std::collections::HashMap;
use std::fmt;

trait MessageView {
    fn message(&self) -> &str;
    fn name(&self) -> &str;
}

impl fmt::Debug for dyn MessageView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}, {}", self.message(), self.name());
    }
}

#[derive(Debug)]
struct MessageStruct {
    message: String,
    name: String,
}

impl MessageView for MessageStruct {
    fn message(&self) -> &str {
        return &self.message;
    }

    fn name(&self) -> &str {
        return &self.name;
    }
}

impl MessageView for HashMap<String, String> {
    fn message(&self) -> &str {
        return self.get("message").map_or("", |x| x);
    }

    fn name(&self) -> &str {
        return self.get("name").map_or("", |x| x);
    }
}

fn main() {
    let message_map = {
        let mut map = HashMap::new();
        map.insert(String::from("message"), String::from("hello"));
        map.insert(String::from("name"), String::from("world"));
        map
    };

    let message_struct = MessageStruct {
        message: String::from("hello"),
        name: String::from("world"),
    };

    println!("{:>24}: {:?}", "map", message_map);
    println!("{:>24}: {:?}", "struct", message_struct);
    println!("{:>24}: {:?}", "map as MessageView", &message_map as &dyn MessageView);
    println!("{:>24}: {:?}", "struct as MessageView", &message_struct as &dyn MessageView);
}

