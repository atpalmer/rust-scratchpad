

struct SomeData {}

trait Collection {
    fn count(&self) -> i32 {
        return 10;
    }
}

trait List: Collection {
    fn count(&self) -> i32 {
        return 20;
    }
}

impl List for SomeData {}

// Since I impl-ed List,
// I _must_ impl its supertrait:
impl Collection for SomeData {}

fn collection_count(arg: &dyn Collection) {
    // arg.count calls Collection::count,
    // regardless of whether the underlying type is a List!

    println!("{:>26}: {}", "arg.count", arg.count());  // -> 10
}

fn list_count(arg: &dyn List) {
    // .count has multiple implementations, so arg.count is a compiler error.
    // I can still explicitly call each implementation...

    println!("{:>26}: {}", "Collection::count(arg)", Collection::count(arg));  // -> 10
    println!("{:>26}: {}", "List::count(arg)", List::count(arg));  // -> 20
}

fn main() {
    let data = SomeData {};
    collection_count(&data);
    list_count(&data);
}
