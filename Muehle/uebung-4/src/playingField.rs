use crate::StringHandler::StringHandler;
use crate::game::Piece;

#[derive(Clone)]
pub struct PlayingField {
    fieldLayout: Vec<String>,
}


impl PlayingField {
    pub fn new() -> PlayingField {
        let mut playingField = PlayingField {
            fieldLayout: Vec::new(),
        };
        playingField.fieldLayout = vec!["".to_string(), "".to_string(), "".to_string(),
            "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(),
            "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
        playingField.fieldLayout[0] = String::from("O--------O--------O");
        playingField.fieldLayout[1] = String::from("|        |        |");
        playingField.fieldLayout[2] = String::from("|  O-----O-----O  |");
        playingField.fieldLayout[3] = String::from("|  |     |     |  |");
        playingField.fieldLayout[4] = String::from("|  |  O--O--O  |  |");
        playingField.fieldLayout[5] = String::from("|  |  |     |  |  |");
        playingField.fieldLayout[6] = String::from("O--O--O     O--O--O");
        playingField.fieldLayout[7] = String::from("|  |  |     |  |  |");
        playingField.fieldLayout[8] = String::from("|  |  O--O--O  |  |");
        playingField.fieldLayout[9] = String::from("|  |     |     |  |");
        playingField.fieldLayout[10]= String::from("|  O-----O-----O  |");
        playingField.fieldLayout[11]= String::from("|        |        |");
        playingField.fieldLayout[12]= String::from("O--------O--------O");
        return playingField;
    }

    pub fn print_field_to_console(&self) {
        for i in 0..13 {
            println!("{}", self.fieldLayout[i]);
        }
    }

    pub fn place_piece(&mut self, turn: bool, position: u32) {
        if turn {
            self.overwrite_tile(position, Piece::BLACK);
        }
        else {
            self.overwrite_tile(position, Piece::WHITE);
        }
    }

    pub fn overwrite_tile(&mut self, position: u32, piece: Piece) {
        match position {
            0 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[0], 0, piece);
            }
            1 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[0], 9, piece);
            }
            2 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[0], 18, piece);
            }
            3 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[2], 3, piece);
            }
            4 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[2], 9, piece);
            }
            5 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[2], 15, piece);
            }
            6 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[4], 6, piece);
            }
            7 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[4], 9, piece);
            }
            8 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[4], 12, piece);
            }
            9 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[6], 0, piece);
            }
            10 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[6], 3, piece);
            }
            11 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[6], 6, piece);
            }
            12 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[6], 12, piece);
            }
            13 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[6], 15, piece);
            }
            14 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[6], 18, piece);
            }
            15 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[8], 6, piece);
            }
            16 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[8], 9, piece);
            }
            17 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[8], 12, piece);
            }
            18 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[10], 3, piece);
            }
            19 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[10], 9, piece);
            }
            20 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[10], 15, piece);
            }
            21 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[12], 0, piece);
            }
            22 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[12], 9, piece);
            }
            23 => {
                StringHandler::place_piece_in_string(&mut self.fieldLayout[12], 18, piece);
            }
            _ => {
                println!("Position must be <23");
            }
        }
    }
}
