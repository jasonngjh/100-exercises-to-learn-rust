// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name : String,
    quantity : u32,
    unit_price : u32
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_param(&quantity);
        validate_param(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String){
        validate_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32){
        // validate_product_name(&product_name);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32){
        // validate_product_name(&product_name);
        self.unit_price = unit_price;
    }

    pub fn total(&mut self) -> u32 {
        return self.quantity * self.unit_price
    }

}

fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!("Product Name cannot be empty")
    }
    if product_name.len() > 300{
        panic!("Product Name cannot be longer than 300 bytes")
    }
}

fn validate_param(param: &u32) {
    if param == &0 {
        panic!("Param cannot be 0")
    }
}