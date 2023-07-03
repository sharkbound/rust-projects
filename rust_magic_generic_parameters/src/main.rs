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

#[derive(Debug, Copy, Clone)]
enum Command {
    Print,
    Log,
    Delete,
}

trait FromData {
    fn from_data(data: &Data) -> Self;
}

#[derive(Debug, Clone)]
struct Data {
    data_type: String,
    id: u32,
}

impl Data {
    pub fn new(data_type: String, id: u32) -> Self {
        Self { data_type, id }
    }
}

impl FromData for Data {
    fn from_data(data: &Data) -> Self {
        data.clone()
    }
}

struct Id {
    id: u32,
}


impl FromData for Id {
    fn from_data(data: &Data) -> Self {
        Id { id: data.id }
    }
}

struct DataType {
    data_type: String,
}

impl FromData for DataType {
    fn from_data(data: &Data) -> Self {
        DataType { data_type: data.data_type.clone() }
    }
}

trait Handler<T> {
    fn handle(&self, data: Data);
}

impl<F, T> Handler<T> for F
    where
        F: Fn(T),
        T: FromData, {
    fn handle(&self, data: Data) {
        (self)(T::from_data(&data));
    }
}

impl<F, T1, T2> Handler<(T1, T2)> for F
    where
        F: Fn(T1, T2),
        T1: FromData,
        T2: FromData,
{
    fn handle(&self, data: Data) {
        (self)(T1::from_data(&data), T2::from_data(&data));
    }
}