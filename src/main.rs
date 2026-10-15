use std::io::{self, Write};

/// Represents different categories of cargo with associated payload data.
#[derive(Debug)]
pub enum CargoType {
    Mineral(u32),
    Food { kg: f64, fresh: bool },
    EnergyCell(f64),
}

/// Represents an individual cargo container aboard the starship.
#[derive(Debug)]
pub struct CargoItem {
    pub id: u32,
    pub name: String,
    pub cargo_type: CargoType,
}

impl CargoItem {
    /// Creates a new cargo container instance.
    pub fn new(id: u32, name: &str, cargo_type: CargoType) -> Self {
        CargoItem {
            id,
            name: name.to_string(),
            cargo_type,
        }
    }

    /// Recharges the energy cell if the item is of type EnergyCell.
    pub fn recharge(&mut self, amount: f64) -> Result<(), String> {
        match &mut self.cargo_type {
            CargoType::EnergyCell(energy) => {
                *energy += amount;
                Ok(())
            }
            _ => Err(format!("Cargo item '{}' (ID: {}) is not an Energy Cell.", self.name, self.id)),
        }
    }
}

/// Represents the starships cargo hold managing multiple items.
pub struct CargoRegistry {
    items: Vec<CargoItem>,
}

impl CargoRegistry {
    /// Creates a new empty or pre-populated cargo registry.
    pub fn new(items: Vec<CargoItem>) -> Self {
        CargoRegistry { items }
    }

    /// Finds a cargo item by its ID using functional iterators (immutable borrow).
    pub fn find_by_id(&self, id: u32) -> Option<&CargoItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Finds a cargo item by its ID for mutable operations (mutable borrow).
    pub fn find_by_id_mut(&mut self, id: u32) -> Option<&mut CargoItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Returns the total count of items in the registry.
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

/// Parses a numeric ID from standard input with a custom Result type.
fn read_cargo_id_input() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read from standard input".to_string())?;

    let trimmed = input.trim();
    trimmed
        .parse::<u32>()
        .map_err(|_| format!("'{}' is not a valid numeric ID", trimmed))
}

fn main() {
    println!("========================================");
    println!("  STARSHIP CARGO & ENERGY REGISTRY v1.2 ");
    println!("========================================");

    let initial_items = vec![
        CargoItem::new(101, "Titanium Ore", CargoType::Mineral(95)),
        CargoItem::new(102, "Hydroponic Salads", CargoType::Food { kg: 150.5, fresh: true }),
        CargoItem::new(103, "Plasma Fuel Cell", CargoType::EnergyCell(500.0)),
    ];

    let mut registry = CargoRegistry::new(initial_items);

    println!("Total items loaded in cargo bay: {}", registry.count());
    print!("Enter cargo ID to recharge (e.g., 103): ");
    io::stdout().flush().unwrap();

    match read_cargo_id_input() {
        Ok(search_id) => {
            // Nutzung der neuen Methode mit mutable Borrowing über Iteratoren
            match registry.find_by_id_mut(search_id) {
                Some(item) => {
                    println!("\n[FOUND] Target acquired: {}", item.name);
                    match item.recharge(150.0) {
                        Ok(()) => println!("[SUCCESS] Cell recharged! New state: {:?}", item.cargo_type),
                        Err(e) => println!("[ERROR] Recharge failed: {}", e),
                    }
                }
                None => {
                    println!("\n[NOT FOUND] No cargo container with ID {} exists.", search_id);
                }
            }
        }
        Err(err) => {
            println!("[INPUT ERROR] {}", err);
        }
    }
}