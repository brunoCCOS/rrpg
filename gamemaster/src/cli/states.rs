pub mod combat;
pub mod telling;


#[derive(Clone)]
pub enum State {
    Combat(combat::CombatState),
    Telling(telling::TellingState),
}
