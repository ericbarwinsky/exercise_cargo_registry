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
}

/// Searches the cargo registry for a specific ID.
/// Returns an `Option<&CargoItem>` via immutable borrowing without transferring ownership.
fn find_cargo_by_id(inventory: &[CargoItem], id: u32) -> Option<&CargoItem> {
    for item in inventory {
        if item.id == id {
            return Some(item);
        }
    }
    None
}

fn main() {
    println!("========================================");
    println!("  STARSHIP CARGO & ENERGY REGISTRY v1.0 ");
    println!("========================================");

    let inventory = vec![
        CargoItem::new(101, "Titanium Ore", CargoType::Mineral(95)),
        CargoItem::new(102, "Hydroponic Salads", CargoType::Food { kg: 150.5, fresh: true }),
        CargoItem::new(103, "Plasma Fuel Cell", CargoType::EnergyCell(500.0)),
    ];

    println!("Total items loaded in cargo bay: {}", inventory.len());
    print!("Enter cargo ID to scan (e.g., 101, 102, 103): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(search_id) = input.trim().parse::<u32>() {
            match find_cargo_by_id(&inventory, search_id) {
                Some(item) => {
                    println!("\n[FOUND] Cargo scanned successfully:");
                    println!("  ID:   {}", item.id);
                    println!("  Name: {}", item.name);
                    println!("  Type: {:?}", item.cargo_type);
                }
                None => {
                    println!("\n[NOT FOUND] No cargo container with ID {} exists.", search_id);
                }
            }
        } else {
            println!("Invalid numeric ID entered.");
        }
    }
}