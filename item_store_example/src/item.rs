use crate::store::StoreItem;

#[derive(Debug, Clone)]
pub struct Item {
    id: String,
    name: String,
    price: f64,
    quantity: u32,
}

impl Item {
    pub fn new(id: &str, name: &str, price: f64, quantity: u32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            price,
            quantity,
        }
    }

    pub fn null() -> Self { Self::new("", "", 0.0, 0) }

    pub fn is_null(&self) -> bool { self.id.is_empty() || self.name.is_empty() }
}

impl StoreItem for Item {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn quantity(&self) -> u32 {
        self.quantity
    }

    fn set_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    fn in_stock(&self) -> bool {
        self.quantity > 0
    }

    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            price: self.price,
            quantity: self.quantity,
        }
    }
}


impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r##"{{ Item -> id: "{}", name: "{}", price: {}, quantity: {} }}"##, self.id, self.name, self.price, self.quantity)
    }
}

