/*
notes:
    TryInto:
        - returns Result<_, _> with ether the converted object, or the alternative `error` type.
          Intended to convert one struct to another, direction is as such: FROM -> TO
        - has a non-Result<_, _> alternative of type `Into`

    FromInto:
        - same as above, however the direction is different.
          direction is as such: TO <- FROM
        - returns Self instead of you needing to specify the type it converts to
        - has a non-Result<_, _> alternative of type `From`

    Both have the default error type of `()`, however it can be changed

    CANNOT have both Into(or From), and TryInto(or TryFrom) declared at the same time for the given struct
*/


/*
    MAIN
*/
extern crate core;

use std::fmt;
use std::fmt::{Formatter};

fn main() {
    let item = Item { name: "james".to_string(), price: 7.40 };
    let quantity_item: ItemWithQuantity = match item.try_into() {
        Ok(item) => {
            println!("{:?}", item);
            item
        }
        Err(e) => panic!("conversion error: {}", e)
    };
}
/*
    STRUCTS
*/
#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    price: f64,
}

#[derive(Debug, Clone)]
pub struct ItemWithQuantity {
    name: String,
    // alternatively, use `&'static str`, HOWEVER, comment from coding den discord:
    // QUOTE:
    // if you use &str you're going to have to thread lifetimes through the struct usage because something else has to actually own the string,
    // having structs own their contents generally makes them easier to deal with
    price: f64,
    quantity: u32,
}

/*
        TRY INTO
*/
impl TryInto<ItemWithQuantity> for Item {
    type Error = ItemConversionError;

    fn try_into(self) -> Result<ItemWithQuantity, Self::Error> {
        if self.name.len() == 0 {
            return Err(ItemConversionError { item: self });
        }
        Ok(ItemWithQuantity {
            name: self.name.to_string(),
            price: self.price,
            quantity: 1,
        })
    }
}

impl TryInto<Item> for ItemWithQuantity {
    type Error = ItemWithQuantityConversionError;

    fn try_into(self) -> Result<Item, Self::Error> {
        if self.name.len() == 0 {
            return Err(ItemWithQuantityConversionError { item: self });
        }
        Ok(Item {
            name: self.name.to_owned(),
            price: self.price,
        })
    }
}

#[derive(Debug)]
pub struct ItemWithQuantityConversionError {
    item: ItemWithQuantity,
}

impl fmt::Display for ItemWithQuantityConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "cannot convert item with empty name")
    }
}

#[derive(Debug)]
pub struct ItemConversionError {
    item: Item,
}

impl fmt::Display for ItemConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "cannot convert item with empty name")
    }
}

/*
        INTO WITHOUT TRY
*/

// impl Into<ItemWithQuantity> for Item {
//     fn into(self) -> ItemWithQuantity {
//         ItemWithQuantity {
//             name: self.name,
//             price: self.price,
//             quantity: 1,
//         }
//     }
// }
//
// impl Into<Item> for ItemWithQuantity {
//     fn into(self) -> Item {
//         Item {
//             name: self.name,
//             price: self.price,
//         }
//     }
// }

