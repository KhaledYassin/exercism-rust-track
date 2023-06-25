// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mana = if self.level >= 10 { Some(100) } else { None };

        let player = if self.health <= 0 {
            Some(Player {
                health: 100,
                mana: mana,
                level: self.level,
            })
        } else {
            None
        };
        player
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let damage = 2 * mana_cost;
        match self.mana {
            Some(x) => {
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    damage
                } else {
                    0
                }
            }
            None => {
                if self.health >= mana_cost {
                    self.health = self.health - mana_cost;
                } else {
                    self.health = 0
                }
                0
            }
        }
    }
}
