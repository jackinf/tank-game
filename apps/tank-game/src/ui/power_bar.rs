//! The vertical power gauge on the left edge.

use super::*;
use crate::economy::Economy;
use crate::faction::Faction;
use bevy::prelude::*;

/// Drive the vertical power bar. The fill height shows how much of produced
/// power is being consumed; green = plenty of headroom, yellow = getting tight,
/// red = over budget (production is throttled).
pub(super) fn update_power_bar(
    economy: Res<Economy>,
    mut fill: Query<(&mut Node, &mut BackgroundColor), With<PowerBarFill>>,
) {
    let Ok((mut node, mut bg)) = fill.single_mut() else {
        return;
    };
    let eco = economy.get(Faction::Player);
    let produced = eco.power_produced.max(0) as f32;
    let consumed = eco.power_consumed as f32;
    let load = consumed / produced.max(1.0);
    // Fill = remaining headroom: a tall bar means plenty of spare power, an
    // empty bar means we're at (or over) capacity.
    let headroom = (1.0 - load).clamp(0.0, 1.0);
    node.height = Val::Percent((headroom * 100.0).max(4.0));
    *bg = BackgroundColor(if load < 0.75 {
        Color::srgb(0.2, 0.9, 0.2)
    } else if load < 1.0 {
        Color::srgb(0.95, 0.8, 0.2)
    } else {
        Color::srgb(0.95, 0.25, 0.2)
    });
}
