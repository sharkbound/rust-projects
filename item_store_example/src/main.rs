use item::Item;
use crate::store::{Store, StoreItem};

mod store;
mod item;


fn main() {
    let mut store = Store::<Item>::empty();

    store.add(Item::new("dairy:milk", "Milk", 18.5, 10));
    store.add(Item::new("dairy:choco-milk", "Chocolate Milk", 40.13, 17));
    store.add(Item::new("candy:tacs", "Tacs", 3.7, 167));

    println!("{:?}", store.lookup(|x| x.id() == "2").unwrap_or(&Item::null()));
    println!("{:?}", store.lookup_all(|x| x.quantity() > 20));
    println!("{:?}", store.iter().filter(|x| x.quantity() > 20).collect::<Vec<_>>());
}
