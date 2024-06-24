pub struct StartingPos {
    attackers: Vec<usize>,
    defenders: Vec<usize>,
    king: Vec<usize>,
}

pub enum Starts {
    COPENHAGEN(StartingPos),
}
