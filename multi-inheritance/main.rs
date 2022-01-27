

trait List {
    fn length(&self) -> i32;
}

trait Rectangle {
    fn length(&self) -> i32;
}

struct MyListRectangle {
    list_length: i32,
}

impl List for MyListRectangle {
    fn length(&self) -> i32 {
        return self.list_length;
    }
}

impl Rectangle for MyListRectangle {
    fn length(&self) -> i32 {
        return <Self as List>::length(self) * 2;
    }
}

fn main() {
    let list = MyListRectangle { list_length: 10 };
    let rect = MyListRectangle { list_length: 10 };
    println!("List length: {}", List::length(&list));
    println!("Rectangle length: {}", Rectangle::length(&rect));
}
