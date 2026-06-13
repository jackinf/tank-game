//! The bottom of the sidebar: the current queue line and a contextual
//! strengths/weaknesses blurb.

use super::*;
use crate::components::{Selected, Unit};
use crate::production::{PlacementMode, ProductionQueue};
use bevy::prelude::*;

/// Show a contextual strengths/weaknesses blurb: the building being placed, the
/// build option under the cursor, or the single selected unit.
#[allow(clippy::type_complexity)]
pub(super) fn update_description(
    placement: Res<PlacementMode>,
    hovered: Query<(&BuildButton, &Interaction)>,
    selected_units: Query<&Unit, With<Selected>>,
    mut text: Query<&mut Text, With<DescriptionText>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    let blurb = if let Some(kind) = placement.0 {
        format!("Placing {}: {}", kind.name(), kind.description())
    } else if let Some((b, _)) = hovered
        .iter()
        .find(|(_, i)| **i == Interaction::Hovered || **i == Interaction::Pressed)
    {
        format!("{}: {}", b.0.name(), b.0.description())
    } else {
        let mut it = selected_units.iter();
        match (it.next(), it.next()) {
            (Some(u), None) => format!("{}: {}", u.kind.name(), u.kind.description()),
            _ => "Hover a build option or select a unit to see its strengths and weaknesses."
                .to_string(),
        }
    };
    text.0 = blurb;
}

pub(super) fn update_queue_text(
    selected: Query<&ProductionQueue, With<Selected>>,
    mut text: Query<&mut Text, With<MenuQueueText>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    let status = match selected.iter().next() {
        Some(q) => {
            if let Some(front) = q.front() {
                let pct = (q.fraction() * 100.0) as i32;
                let extra = q.items.len().saturating_sub(1);
                if extra > 0 {
                    format!("{} {}%  (+{})", front.short(), pct, extra)
                } else {
                    format!("{} {}%", front.short(), pct)
                }
            } else {
                "idle".to_string()
            }
        }
        None => "—".to_string(),
    };
    text.0 = status;
}
