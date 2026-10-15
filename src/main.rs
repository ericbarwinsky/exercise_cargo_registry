mod cargo;

use cargo::{CargoItem, CargoRegistry, CargoType};
use std::io::{self, Write};

fn read_input(prompt_text: &str) -> Result<String, String> {
    print!("{}", prompt_text);
    io::stdout().flush().map_err(|_| "Failed to flush stdout".to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read from standard input".to_string())?;

    Ok(input.trim().to_string())
}

fn read_u32(prompt_text: &str) -> Result<u32, String> {
    let input = read_input(prompt_text)?;
    input.parse::<u32>().map_err(|_| format!("'{}' is not a valid number", input))
}

fn read_f64(prompt_text: &str) -> Result<f64, String> {
    let input = read_input(prompt_text)?;
    input.parse::<f64>().map_err(|_| format!("'{}' is not a valid decimal number", input))
}

fn list_registry_summary(registry: &CargoRegistry) {
    let load = registry.current_weight();
    println!("\n--- CURRENT CARGO REGISTRY ---");
    println!("Capacity: {:.1} / {:.1} kg", load, registry.max_capacity());
    if registry.items().is_empty() {
        println!("Registry is empty.");
    } else {
        for item in registry.items() {
            println!(
                "ID: {} | Name: {} | Weight: {:.1} kg | Type: {:?}",
                item.id, item.name, item.weight(), item.cargo_type
            );
        }
    }
    println!("------------------------------\n");
}

fn main() {
    println!("========================================");
    println!("  STARSHIP CARGO & ENERGY REGISTRY v2.0 ");
    println!("========================================");

    let initial_items = vec![
        CargoItem::new(101, "Titanium Ore", CargoType::Mineral(95)),
        CargoItem::new(102, "Hydroponic Salads", CargoType::Food { kg: 150.5, fresh: true }),
        CargoItem::new(103, "Plasma Fuel Cell", CargoType::EnergyCell(500.0)),
    ];

    let mut registry = CargoRegistry::new(initial_items, 1000.0);

    loop {
        println!("Choose an action:");
        println!("1: Add new cargo");
        println!("2: Recharge energy cell");
        println!("3: Remove cargo by ID");
        println!("4: Show current summary");
        println!("5: Exit");

        match read_u32("Enter choice (1-5): ") {
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

                let new_item = CargoItem::new(id, &name, cargo_type);
                match registry.add_item(new_item) {
                    Ok(()) => println!("[SUCCESS] Cargo added successfully!"),
                    Err(err) => println!("[CAPACITY ERROR] {}", err),
                }
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
                list_registry_summary(&registry);
            }
            Ok(5) => {
                println!("Shutting down cargo registry manager. Goodbye!");
                break;
            }
            Ok(_) => {
                println!("Invalid choice. Please select between 1 and 5.");
            }
            Err(e) => {
                println!("[INPUT ERROR] {}", e);
            }
        }
    }
}