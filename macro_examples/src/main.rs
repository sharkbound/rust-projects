use paste::paste;

mod declare_macros;


fn main() {
    let mut item = Item::new("milk".to_string(), (0, 0));

    println!("{:#?}", item);
}

#[derive(Debug)]
struct Item {
    name: String,
    location: (i32, i32),
}

impl Item {
    // generates the ::new() constructor
    declare_impl_new!(name:String; location:(i32, i32));
}

