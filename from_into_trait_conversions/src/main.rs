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
fn main() {
    let item = Item { name: "cheese", price: 7.40 };
    let quantity_item: ItemWithQuantity = item.try_into().unwrap();
}
/*
    STRUCTS
*/
#[derive(Debug, Copy, Clone)]
pub struct Item {
    name: &'static str,
    price: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ItemWithQuantity {
    name: &'static str,
    price: f64,
    quantity: u32,
}

/*
        TRY INTO
*/
impl TryInto<ItemWithQuantity> for Item {
    type Error = &'static str;

    fn try_into(self) -> Result<ItemWithQuantity, Self::Error> {
        if self.name.len() == 0 {
            return Err("cannot convert item with empty name");
        }
        Ok(ItemWithQuantity {
            name: self.name,
            price: self.price,
            quantity: 1,
        })
    }
}

impl TryInto<Item> for ItemWithQuantity {
    type Error = &'static str; // alternatively, use (), and return Err(())

    fn try_into(self) -> Result<Item, Self::Error> {
        if self.name.len() == 0 {
            return Err("cannot convert item with empty name");
        }
        Ok(Item {
            name: self.name,
            price: self.price,
        })
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
