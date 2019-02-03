use crate::player::Player;

#[derive(Debug)]
pub struct Game {
    pub board_boats: Vec<Vec<Option<u8>>>,
    pub shot_boats: Vec<Vec<Option<bool>>>,
    pub player: Player,
}

fn in_board(size_x: u8, size_y: u8, x: i8, y: i8) -> bool {
    x >= 0 && y >= 0 && x < size_x as i8 && y < size_y as i8
}

impl Game {
    pub fn shoot(&mut self, p: (u8, u8)) -> bool {
        match &self.board_boats[p.0 as usize][p.1 as usize] {
            None => false,
            Some(i) => self.player.boats[*i as usize].shoot(p),
        }
    }

    pub fn new(size_x: usize, size_y: usize, player: Player) -> Game {
        let mut board_boats: Vec<Vec<Option<u8>>> = vec![vec![None; size_y]; size_x];

        let mut i = 0;
        for boat in &player.boats {
            let d = boat.direction.delta();
            let mut x = boat.position.0 as i8;
            let mut y = boat.position.1 as i8;

            if !in_board(board_boats.len() as u8, board_boats[0].len() as u8, x, y)
                || !in_board(
                    board_boats.len() as u8,
                    board_boats[0].len() as u8,
                    x + d.0 * boat.max_life() as i8,
                    y + d.1 * boat.max_life() as i8,
                )
            {
                panic!("Wrong boat position {:?}", boat);
            }
            for _ in 0..boat.max_life() {
                match &board_boats[x as usize][y as usize] {
                    Some(b) => panic!("Boat overleap : {:?} and {:?}", b, boat),
                    None => board_boats[x as usize][y as usize] = Some(i),
                }
                x += d.0;
                y += d.1;
            }
            i += 1;
        }

        Game {
            board_boats,
            shot_boats: vec![vec![None; size_y]; size_x],
            player,
        }
    }
}

#[derive(Debug)]
pub enum GameType {
    Network(Game, std::net::TcpStream),
    _Ai(Game, Game),
}
