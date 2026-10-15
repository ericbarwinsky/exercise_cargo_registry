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

    /// Adds a new cargo item to the registry.
    pub fn add_item(&mut self, item: CargoItem) {
        self.items.push(item);
    }

    /// Removes a cargo item by its ID, taking ownership of the removed item (or None if not found).
    pub fn remove_item(&mut self, id: u32) -> Option<CargoItem> {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Finds a cargo item by its ID for mutable operations.
    pub fn find_by_id_mut(&mut self, id: u32) -> Option<&mut CargoItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Prints a summary of all loaded cargo items.
    pub fn list_cargo(&self) {
        println!("\n--- CURRENT CARGO REGISTRY ---");
        if self.items.is_empty() {
            println!("Registry is empty.");
        } else {
            for item in &self.items {
                println!("ID: {} | Name: {} | Type: {:?}", item.id, item.name, item.cargo_type);
            }
        }
        println!("------------------------------\n");
    }
}

/// Reads a line from standard input and parses it into a trimmed String.
fn read_input(prompt_text: &str) -> Result<String, String> {
    print!("{}", prompt_text);
    io::stdout().flush().map_err(|_| "Failed to flush stdout".to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read from standard input".to_string())?;

    Ok(input.trim().to_string())
}

/// Helper to read and parse a u32 value safely.
fn read_u32(prompt_text: &str) -> Result<u32, String> {
    let input = read_input(prompt_text)?;
    input.parse::<u32>().map_err(|_| format!("'{}' is not a valid number", input))
}

/// Helper to read and parse an f64 value safely.
fn read_f64(prompt_text: &str) -> Result<f64, String> {
    let input = read_input(prompt_text)?;
    input.parse::<f64>().map_err(|_| format!("'{}' is not a valid decimal number", input))
}

fn main() {
    println!("========================================");
    println!("  STARSHIP CARGO & ENERGY REGISTRY v1.3 ");
    println!("========================================");

    let initial_items = vec![
        CargoItem::new(101, "Titanium Ore", CargoType::Mineral(95)),
        CargoItem::new(102, "Hydroponic Salads", CargoType::Food { kg: 150.5, fresh: true }),
        CargoItem::new(103, "Plasma Fuel Cell", CargoType::EnergyCell(500.0)),
    ];

    let mut registry = CargoRegistry::new(initial_items);

    loop {
        registry.list_cargo();
        println!("Choose an action:");
        println!("1: Add new cargo");
        println!("2: Recharge energy cell");
        println!("3: Remove cargo by ID");
        println!("4: Exit");

        match read_u32("Enter choice (1-4): ") {
            Ok(1) => {
                let id = match read_u32("Enter cargo ID: ") {
                    Ok(val) => val,
                    Err(e) => { println!("[ERROR] {}", e); continue; }
                };

                let name = match read_input("Enter cargo name: ") {
                    Ok(val) => val,
                    Err(e) => { println!("[ERROR] {}", e); continue; }
                };

                println!("Select cargo type:");
                println!("1: Mineral");
                println!("2: Food");
                println!("3: EnergyCell");

                let type_choice = match read_u32("Type choice (1-3): ") {
                    Ok(val) => val,
                    Err(e) => { println!("[ERROR] {}", e); continue; }
                };

                let cargo_type = match type_choice {
                    1 => {
                        let purity = read_u32("Enter purity percentage (0-100): ").unwrap_or(50);
                        CargoType::Mineral(purity)
                    }
                    2 => {
                        let kg = read_f64("Enter weight in kg: ").unwrap_or(10.0);
                        CargoType::Food { kg, fresh: true }
                    }
                    3 => {
                        let energy = read_f64("Enter initial energy (MJ): ").unwrap_or(100.0);
                        CargoType::EnergyCell(energy)
                    }
                    _ => {
                        println!("[ERROR] Invalid type choice. Defaulting to empty mineral.");
                        CargoType::Mineral(0)
                    }
                };

                registry.add_item(CargoItem::new(id, &name, cargo_type));
                println!("[SUCCESS] Cargo added successfully!");
            }
            Ok(2) => {
                let id = match read_u32("Enter ID of Energy Cell to recharge: ") {
                    Ok(val) => val,
                    Err(e) => { println!("[ERROR] {}", e); continue; }
                };

                let amount = match read_f64("Enter energy amount to add (MJ): ") {
                    Ok(val) => val,
                    Err(e) => { println!("[ERROR] {}", e); continue; }
                };

                match registry.find_by_id_mut(id) {
                    Some(item) => {
                        match item.recharge(amount) {
                            Ok(()) => println!("[SUCCESS] Cell recharged! New state: {:?}", item.cargo_type),
                            Err(err) => println!("[ERROR] {}", err),
                        }
                    }
                    None => println!("[NOT FOUND] No item with ID {} found.", id),
                }
            }
            Ok(3) => {
                let id = match read_u32("Enter ID of cargo to remove: ") {
                    Ok(val) => val,
                    Err(e) => { println!("[ERROR] {}", e); continue; }
                };

                match registry.remove_item(id) {
                    Some(removed) => println!("[SUCCESS] Removed cargo: {} ({})", removed.name, removed.id),
                    None => println!("[NOT FOUND] No item with ID {} found.", id),
                }
            }
            Ok(4) => {
                println!("Shutting down cargo registry manager. Goodbye!");
                break;
            }
            Ok(_) => {
                println!("Invalid choice. Please select between 1 and 4.");
            }
            Err(e) => {
                println!("[INPUT ERROR] {}", e);
            }
        }
    }
}