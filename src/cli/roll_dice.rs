use crate::elements::dice::DiceRoll;

pub struct RollDiceArgs {
    pub num_dice: u32,
    pub num_faces: u32,
    pub modifier: i32,
}

pub fn execute(args: &RollDiceArgs) {
    let dice_roll = DiceRoll::new(args.num_dice, args.num_faces, args.modifier);
    let result = dice_roll.roll();

    println!(
        "Rolled {}d{} + {}: Total = {}",
        result.num_dice, result.num_faces, result.modifier, result.total
    );
}
