use log::info;
use rand::rngs::StdRng;

use crate::{orientation::Orientation, world::player::Player};

use super::GeneratorConfig;

pub struct PlayerGenerator;

impl PlayerGenerator {
    pub fn generate_player<I: Into<String>>(
        rng: &mut StdRng,
        id: I,
        config: &GeneratorConfig,
    ) -> Player {
        let id = id.into();

        let location = config.random_location(rng);

        info!(
            "Adding a player with id '{}' at ({}, {})",
            id,
            location.x(),
            location.z()
        );

        let orientation = Orientation::new(0, 90);

        let fov = 60;

        Player::new(location, orientation, fov, id)
    }

    pub fn generate_players(rng: &mut StdRng, config: &GeneratorConfig) -> Vec<Player> {
        let mut players = Vec::new();

        let main_player = Self::generate_player(rng, "main", config);

        players.push(main_player);

        players
    }
}
