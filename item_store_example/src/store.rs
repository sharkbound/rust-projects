use crate::item::Item;

pub trait StoreItem {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn price(&self) -> f64;
    fn quantity(&self) -> u32;
    fn set_quantity(&mut self, quantity: u32);
    fn set_price(&mut self, price: f64);
    fn in_stock(&self) -> bool;
    fn clone(&self) -> Self;
}

pub struct Store<T: StoreItem> {
    items: Vec<T>,
}

impl<T: StoreItem> Store<T> {
    pub fn empty() -> Self {
        Self {
            items: vec![],
        }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn lookup<F: Fn(&&T) -> bool>(&self, predicate: F) -> Option<&T> {
        self.items.iter().find(predicate)
    }

    pub fn lookup_all<F: Fn(&&T) -> bool>(&self, predicate: F) -> Vec<&T> {
        self.items.iter().filter(predicate).collect()
    }

    pub fn iter(&self) -> StoreItemIter<T> {
        StoreItemIter::new(&self.items)
    }
}

pub struct StoreItemIter<'a, T: StoreItem> {
    items: &'a Vec<T>,
    index: usize,
}

impl<'a, T: StoreItem> StoreItemIter<'a, T> {
    pub fn new(items: &'a Vec<T>) -> Self {
        Self {
            items,
            index: 0,
        }
    }
}

impl<'a, T: StoreItem> Iterator for StoreItemIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = &self.items[self.index];
            self.index += 1;
            return Some(item);
        }
        None
    }
}

