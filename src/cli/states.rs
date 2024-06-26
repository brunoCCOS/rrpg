use combat::CombatState;
use telling::TellingState;

pub mod combat;
pub mod telling;

#[derive(Clone)]
pub enum State {
    Combat(CombatState),
    Telling(TellingState),
}