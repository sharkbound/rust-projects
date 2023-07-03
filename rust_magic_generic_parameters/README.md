# What is this?
This is a neat trick for accepting variable parameter functions and such without having 
multiple functions for each parameter set.

It utilizes generic traits to achieve this

# Where I discovered this trick from
I originally found this [from this repo](https://github.com/alexpusch/rust-magic-function-params), 

[Axum](https://crates.io/crates/axum) is where I read saw this being used for the first time, and the repo above
has a simplified example showing how it works.


# Showing it in action
```rust
fn main() {
    trigger(Data::new("json".into(), 1), print_id);
    trigger(Data::new("json".into(), 1), print_data_type);
    trigger(Data::new("json".into(), 1), print_id_and_data_type);
}

fn print_id(Id { id }: Id) {
    println!("The id is {:?}", id);
}

fn print_data_type(DataType { data_type }: DataType) {
    println!("The data_type is {:?}", data_type);
}

fn print_id_and_data_type(Id { id }: Id, DataType { data_type }: DataType) {
    println!("The data_type is {:?}, and the id is {:?}", data_type, id);
}


fn trigger<T, H: Handler<T>>(data: Data, handler: H) {
    handler.handle(data);
}
```

As the example above shows, the `trigger` function can accept multiple types of functions, with varying Parameter 
types and counts