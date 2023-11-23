use bevy::ecs::event::Event;

#[derive(Event)]
pub struct GameOver {
    pub score: u32
}