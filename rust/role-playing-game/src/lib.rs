pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let is_dead = self.health == 0;
        let is_level_10_or_above = self.level >= 10;

        if !is_dead && !is_level_10_or_above {
            None
        } else {
            Some(Self{
                health: 100,
                mana: if is_level_10_or_above { Some(100) } else { self.mana },
                level: self.level
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health -= mana_cost.min(self.health);
                0
            }
            Some(mana) => {
                if mana < mana_cost { 0 }
                else {
                    self.mana = if mana > mana_cost { Some(mana - mana_cost) } else { None };
                    2 * mana_cost
                }
            }
        }
    }
}
