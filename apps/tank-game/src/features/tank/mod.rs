pub mod managers {
    pub mod tank_health_manager;
    pub mod tank_movement_manager;
    pub mod tank_shooting_manager;
    pub mod tank_spawn_manager;
}
pub mod components {
    pub mod tank;
    pub mod tank_bullet;
    pub mod tank_gun;
    pub mod tank_health;
}
pub mod resources {
    pub mod tank_monitoring_timer;
}
pub mod tank_plugin;
pub mod tank_queries;
