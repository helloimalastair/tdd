mod shopping_cart;
mod product;

#[cfg(test)]
mod tests {
    use crate::shopping_cart::ShoppingCart;
    #[test]
    fn add_to_shopping_cart() {
        let mut cart = ShoppingCart::new();
        cart.add_item("Mælk", 1.99);
    }
    #[test]
    fn remove_from_shopping_cart() {
        let mut cart = ShoppingCart::new();
        cart.add_item("Mælk", 1.99);
        cart.remove_item("Mælk");
    }
    #[test]
    fn total_cost() {
        let mut cart = ShoppingCart::new();
        cart.add_item("Mælk", 1.99);
        cart.add_item("Brød", 2.99);
        cart.add_item("Smør", 3.99);
        assert_eq!(cart.total_cost(), 8.97);
    }
}