use crate::product::Product;

pub struct ShoppingCart {
	items: Vec<Product>,
}

impl ShoppingCart {
	pub fn new() -> ShoppingCart {
		ShoppingCart { items: Vec::new() }
	}
	pub fn add_item(&mut self, name: &str, price: f64) {
		self.items.push(Product { name: name.to_string(), price });
	}
	pub fn remove_item(&mut self, name: &str) {
		self.items.retain(|item| item.name != name);
	}
	pub fn total_cost(&self) -> f64 {
		self.items.iter().map(|item| item.price).sum()
	}
}