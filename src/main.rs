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
/// Returns an `Option<&CargoItem>` via immutable borrowing.
fn find_cargo_by_id(inventory: &[CargoItem], id: u32) -> Option<&CargoItem> {
    for item in inventory {
        if item.id == id {
            return Some(item);
        }
    }
    None
}

/// Recharges an energy cell by adding a given amount.
/// Uses mutable borrowing (`&mut CargoItem`) to modify the inner data safely.
fn recharge_energy_cell(item: &mut CargoItem, amount: f64) -> Result<(), String> {
    match &mut item.cargo_type {
        CargoType::EnergyCell(energy) => {
            *energy += amount;
            Ok(())
        }
        _ => Err(format!("Cargo item '{}' (ID: {}) is not an Energy Cell.", item.name, item.id)),
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
    println!("  STARSHIP CARGO & ENERGY REGISTRY v1.1 ");
    println!("========================================");

    // Initialisierung des Inventars mit einer mutablen Bindung (`mut`)
    let mut inventory = vec![
        CargoItem::new(101, "Titanium Ore", CargoType::Mineral(95)),
        CargoItem::new(102, "Hydroponic Salads", CargoType::Food { kg: 150.5, fresh: true }),
        CargoItem::new(103, "Plasma Fuel Cell", CargoType::EnergyCell(500.0)),
    ];

    println!("Total items loaded in cargo bay: {}", inventory.len());
    print!("Enter cargo ID to recharge (e.g., 103): ");
    io::stdout().flush().unwrap();

    // Verwendung unseres Result-basierten Parsers
    match read_cargo_id_input() {
        Ok(search_id) => {
            // Wir suchen den Container im Inventar – diesmal brauchen wir eine mutable Referenz (`&mut`)
            let mut target_item: Option<&mut CargoItem> = None;
            for item in &mut inventory {
                if item.id == search_id {
                    target_item = Some(item);
                    break;
                }
            }

            match target_item {
                Some(item) => {
                    println!("\n[FOUND] Target acquired: {}", item.name);
                    // Mutable Borrowing in Aktion: Wir laden die Zelle auf
                    match recharge_energy_cell(item, 150.0) {
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