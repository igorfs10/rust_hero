/// UI actions
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
}
#[derive(Debug, Copy, Clone)]
pub enum Action {
    Move(Direction),
}
