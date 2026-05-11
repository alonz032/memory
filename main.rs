#[derive(Debug, Clone)]
struct Item {
    name: String,
    qty: i32,
}

struct Warehouse {
    location: String,
    inventory: Vec<Item>,
}

impl Warehouse {
    pub fn new(location: String) -> Self {
        Self {
            location,
            inventory: Vec::new(),
        }
    }

    // Task #1:  Fix this method
    pub fn list_inventory(self) {
        println!("Warehouse: {}", self.location);
        for item in &self.inventory {
            println!(" - {}: {}", item.name, item.qty);
        }
    }
    

    // Task #2:  Fix this method
    pub fn restock(&self, item_name: &str, amount: i32) {
        for item in self.inventory.iter_mut() {
            if item.name == item_name {
                item.qty += amount;
            }
        }
    }

    // Task #3: Explain whether this method is correct or not.
    pub fn ship_item(&mut self, index: usize) -> Item {
        self.inventory.remove(index)
    }

    // Task #4: add an item method that 
    // accepts parameter of type Item and adds it to inventory

}

// Task 5:
// Implement order_popular_items function
// This should take a source Warehouse
// Loop through inventory and order(call add_item())
// On any items that have qty greater than 100


fn main() {
    let mut central_hub = Warehouse::new(String::from("Chicago"));
    let mut satellite_office = Warehouse::new(String::from("Denver"));

    // SETUP
    central_hub.add_item(Item { name: String::from("Laptops"), qty: 50 });
    central_hub.add_item(Item { name: String::from("Monitors"), qty: 250 });
    central_hub.add_item(Item { name: String::from("Keyboards"), qty: 500 });
    central_hub.add_item(Item { name: String::from("Mice"), qty: 80 });
    central_hub.add_item(Item { name: String::from("OLED Fans"), qty: 600 });

    println!("--- TASK 1: Listing Inventory ---");
    // TODO: Call list_inventory() on central_hub.
    // NOTE: If this "moves" the warehouse, the rest of the code will break!

    println!("\n--- TASK 2: Restocking Laptops ---");
    // Show before
    if let Some(i) = central_hub.inventory.iter().find(|i| i.name == "Laptops") {
        println!("Before: {:?}", i);
    }
    
    // TODO: Call restock() here to add 25 to "Laptops"
    
    if let Some(i) = central_hub.inventory.iter().find(|i| i.name == "Laptops") {
        println!("After:  {:?} (EXPECTED QTY: 75)", i);
    }

    println!("\n--- TASK 3: Shipping from Central Hub ---");
    // TODO: Call ship_item() on central_hub (index 0) and store in 'dispatched'
    // println!("Shipped: {:?}", dispatched);

    println!("\n--- TASK 4: Manual Entry ---");
    // TODO: Create a new Item "Webcam" with qty 150 and call add_item() for central_hub.

    println!("\n--- TASK 5: Popular Item Transfer (Qty > 100) ---");
    // TODO: Call order_popular_items() to copy heavy stock from central_hub to satellite_office.
    
    println!("Satellite Office Inventory (EXPECTED: Monitors, Keyboards, OLED Fans, Webcams):");
    for item in &satellite_office.inventory {
        println!(" -> {:?}", item);
    }

    println!("\n--- FINAL INTEGRITY CHECK ---");
    println!("Warehouse {} status: Operational.", central_hub.location);
}
