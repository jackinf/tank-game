//! The "what can be built" rules: which buildings produce what, prerequisite
//! checks, and enqueuing (which charges credits up front).

use super::{OwnedBuildings, ProductionQueue};
use crate::defs::*;
use crate::economy::Economy;
use crate::faction::Faction;

/// Does this building host a build queue, and what can it make?
pub fn is_producer(kind: BuildingKind) -> bool {
    matches!(
        kind,
        BuildingKind::ConstructionYard | BuildingKind::Barracks | BuildingKind::WarFactory
    )
}

/// The list of things a given production building can build.
pub fn producible_menu(kind: BuildingKind) -> Vec<Producible> {
    match kind {
        BuildingKind::ConstructionYard => BuildingKind::ALL
            .into_iter()
            .filter(|k| *k != BuildingKind::ConstructionYard)
            .map(Producible::Building)
            .collect(),
        BuildingKind::Barracks => UnitKind::infantry().into_iter().map(Producible::Unit).collect(),
        BuildingKind::WarFactory => UnitKind::vehicles().into_iter().map(Producible::Unit).collect(),
        _ => Vec::new(),
    }
}

/// Whether `faction` meets the prerequisites to start building `item`.
pub fn prerequisites_met(owned: &OwnedBuildings, faction: Faction, item: Producible) -> bool {
    let set = owned.get(faction);
    match item {
        Producible::Building(b) => b.prerequisite().is_none_or(|p| set.contains(&p)),
        Producible::Unit(u) => set.contains(&u.prerequisite()),
    }
}

/// Try to add an item to a building's queue. Charges credits up front.
pub fn try_enqueue(
    queue: &mut ProductionQueue,
    economy: &mut Economy,
    owned: &OwnedBuildings,
    faction: Faction,
    item: Producible,
) -> bool {
    if !prerequisites_met(owned, faction, item) {
        return false;
    }
    let cost = item.cost();
    if !economy.get(faction).can_afford(cost) {
        return false;
    }
    economy.get_mut(faction).credits -= cost;
    queue.items.push_back(item);
    true
}
