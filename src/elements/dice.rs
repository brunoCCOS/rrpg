use rand::Rng;

#[derive(Debug)]
pub struct DiceRoll {
    pub num_dice: u32,
    pub num_faces: u32,
    pub modifier: i32,
}

impl DiceRoll {
    pub fn new(num_dice: u32, num_faces: u32, modifier: i32) -> Self {
        DiceRoll {
            num_dice,
            num_faces,
            modifier,
        }
    }

    pub fn roll(&self) -> RollResult {
        let mut rng = rand::thread_rng();
        let mut total = 0;

        for _ in 0..self.num_dice {
            total += rng.gen_range(1..=self.num_faces) as i32;
        }

        total += self.modifier;

        RollResult {
            total,
            num_dice: self.num_dice,
            num_faces: self.num_faces,
            modifier: self.modifier,
        }
    }
}

#[derive(Debug)]
pub struct RollResult {
    pub total: i32,
    pub num_dice: u32,
    pub num_faces: u32,
    pub modifier: i32,
}
