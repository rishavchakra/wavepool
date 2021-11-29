/**
 * Entity for the ECS Entity Component System
 * Contains the ID of the Entity which points to its components
 */

#[derive(Hash, PartialEq, Eq)]
pub struct Entity {
    id: usize,
    name: String
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            id: 0,
            name: "".to_string()
        }
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

