/// Represents different categories of cargo with associated payload data.
#[derive(Debug, PartialEq)]
pub enum CargoType {
    Mineral(u32),
    Food { kg: f64, fresh: bool },
    EnergyCell(f64),
}

impl CargoType {
    /// Calculates and returns the exact weight in kilograms for any cargo type.
    pub fn calculate_weight(&self) -> f64 {
        match self {
            CargoType::Mineral(purity) => *purity as f64 * 2.5,
            CargoType::Food { kg, .. } => *kg,
            CargoType::EnergyCell(mj) => *mj * 0.1,
        }
    }
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

    /// Returns the weight of this specific item.
    pub fn weight(&self) -> f64 {
        self.cargo_type.calculate_weight()
    }
}

/// Represents the starships cargo hold managing multiple items with a strict weight limit.
pub struct CargoRegistry {
    items: Vec<CargoItem>,
    max_capacity_kg: f64,
}

impl CargoRegistry {
    /// Creates a new cargo registry with a defined maximum capacity.
    pub fn new(items: Vec<CargoItem>, max_capacity_kg: f64) -> Self {
        CargoRegistry { items, max_capacity_kg }
    }

    /// Calculates the current total weight of all cargo items using iterators.
    pub fn current_weight(&self) -> f64 {
        self.items.iter().map(|item| item.weight()).sum()
    }

    /// Adds a new cargo item if it doesn't exceed the maximum weight capacity.
    pub fn add_item(&mut self, item: CargoItem) -> Result<(), String> {
        let item_weight = item.weight();
        let current_load = self.current_weight();

        if current_load + item_weight > self.max_capacity_kg {
            return Err(format!(
                "Capacity exceeded! Cannot add '{}' ({} kg). Current load: {:.1} kg / Max: {:.1} kg",
                item.name, item_weight, current_load, self.max_capacity_kg
            ));
        }

        self.items.push(item);
        Ok(())
    }

    /// Removes a cargo item by its ID.
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

    /// Returns a reference to all items for read-only listing.
    pub fn items(&self) -> &[CargoItem] {
        &self.items
    }

    pub fn max_capacity(&self) -> f64 {
        self.max_capacity_kg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_weight_calculation() {
        let mineral = CargoType::Mineral(40); // 40 * 2.5 = 100.0 kg
        assert_eq!(mineral.calculate_weight(), 100.0);

        let food = CargoType::Food { kg: 75.5, fresh: true };
        assert_eq!(food.calculate_weight(), 75.5);
    }

    #[test]
    fn test_registry_capacity_limit() {
        let mut registry = CargoRegistry::new(vec![], 50.0); // Limit 50 kg

        let light_item = CargoItem::new(1, "Small Box", CargoType::Food { kg: 30.0, fresh: true });
        assert!(registry.add_item(light_item).is_ok());

        let heavy_item = CargoItem::new(2, "Heavy Armor", CargoType::Mineral(30)); // 30 * 2.5 = 75 kg -> zu schwer!
        let result = registry.add_item(heavy_item);
        assert!(result.is_err());
    }

    #[test]
    fn test_energy_cell_recharge() {
        let mut cell = CargoItem::new(3, "Battery", CargoType::EnergyCell(100.0));
        assert!(cell.recharge(50.0).is_ok());

        if let CargoType::EnergyCell(val) = cell.cargo_type {
            assert_eq!(val, 150.0);
        } else {
            panic!("Expected EnergyCell type");
        }
    }
}