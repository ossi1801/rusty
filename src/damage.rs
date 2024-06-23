use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct DamageSystemPlugin;
impl Plugin for DamageSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_systems(Update, process_damage_events);
    }
}

#[derive(Event, Component)]
pub struct DamageEvent {
    pub damage: f32,
    pub target: Entity,
}
#[derive(Component)]
pub struct Health {
    pub hp: f32,
    pub is_dead: bool,
}

pub fn process_damage_events(
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for &DamageEvent { damage, target } in events.read() {
        // If this entity has a Health component, damage it.
        if let Ok(mut health) = health_query.get_mut(target) {
            //info!("{}", health.hp);
            if health.hp <= 0. {
                health.is_dead = true;
            } else {
                health.hp -= damage;
            }
        }
    }
}

// fn contact_damage(
//     mut events: EventWriter<DamageEvent>,
//     query: Query<(&ContactDamage, &CollidingEntities)>,
// ) {
//     for (contact_damage, collisions) in query.iter() {
//         for target in collisions {
//             events.send(DamageEvent {
//                 damage: contact_damage.damage,
//                 target,
//             });
//         }
//     }
// }
