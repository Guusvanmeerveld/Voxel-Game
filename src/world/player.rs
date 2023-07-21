use crate::{location::Location, orientation::Orientation};

pub struct Players {
    players: Vec<Player>,
}

impl Players {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    pub fn add(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn get(&mut self, id: String) -> Option<&Player> {
        self.players.iter().find(|player| player.id() == &id)
    }
}

impl From<Vec<Player>> for Players {
    fn from(players: Vec<Player>) -> Self {
        Self { players }
    }
}

pub struct Player {
    location: Location,
    orientation: Orientation,
    fov: usize,
    id: String,
}

impl Player {
    pub fn new(location: Location, orientation: Orientation, fov: usize, id: String) -> Self {
        Self {
            location,
            orientation,
            fov,
            id,
        }
    }

    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn fov(&self) -> usize {
        self.fov
    }
}
