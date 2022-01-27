

struct Data {}

trait Collection {
    fn count(&self) -> i32 {
        return 10;
    }

    fn my_type(&self) -> String {
        return String::from("I'm a Collection");
    }
}

trait List: Collection {
    fn count(&self) -> i32 {
        return 20;
    }
}

// Can't leave this out, since I also have "impl List for Data":
impl Collection for Data {}

impl List for Data {}

fn show_collection_count(coll: &dyn Collection) {
    println!("Collection count: {}", coll.count()); // 10

    // Can't write this, b/c Collections are not guaranteed to be Lists:
    // println!("List count: {}", List::count(coll));

    println!("Collection my_type: {}", coll.my_type()); // "I'm a Collection"
}

fn show_list_count(list: &dyn List) {

    // Can't write this, b/c there are multiple implementations for .count():
    // println!("List count: {}", list.count());

    println!("Collection::count(list): {}", Collection::count(list)); // 10
    println!("List count: {}", List::count(list)); // 20
    println!("List my_type: {}", list.my_type()); // "I'm a Collection"
}

fn main() {
    let data = Data {};
    show_collection_count(&data);
    show_list_count(&data);
}
