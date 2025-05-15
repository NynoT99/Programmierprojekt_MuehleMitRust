use crate::turn::Turn;
use crate::game::*;
use crate::take::Take;

pub struct FMIBeaver {
    take: Take,
}

impl FMIBeaver {
    pub fn new() -> FMIBeaver {
        return FMIBeaver{
            take: Take::new(),
        };
    }
    pub fn ask_the_beaver(&mut self, placePosition: &PlacePosition) -> Vec<Turn> {
        let mut returnVector = Vec::new();
        //no pieces on hand left
        if placePosition.get_pieces_on_hand() == 0 {
            if placePosition.get_turn() {
                for i in 0..24 {
                    match placePosition.get_field().get_piece_at_tile(i.try_into().unwrap()) {
                        Piece::BLACK => {}
                        _ => {continue;}
                    }
                    for j in 0..24 {
                        if is_move_legal(Move::new(i, j), &placePosition.get_field()) {
                            if self.take.checkTake(&mut calculate_field_after_move(Move::new(i.try_into().unwrap(), j.try_into().unwrap()), placePosition.get_field())) {
                                //calculate which pieces are secure
                                let mut secureFromTake = self.take.secureFromTake(&mut placePosition.get_field());
                                for a in 0..24 {
                                    match placePosition.get_field().get_piece_at_tile(a.try_into().unwrap()) {
                                        Piece::WHITE => {
                                            let mut isSecure = false;
                                            for piece in &secureFromTake {
                                                if &a == piece {
                                                    isSecure = true;
                                                    break;
                                                }
                                            }
                                            if !(isSecure) {
                                                returnVector.push(Turn::new2(i.try_into().unwrap(), j.try_into().unwrap(), a.try_into().unwrap()));
                                            }
                                        }
                                        _ => {continue;}
                                    }
                                }
                            }
                            else {
                                returnVector.push(Turn::new(i.try_into().unwrap(), j.try_into().unwrap()));
                            }
                        }
                    }
                }
            }
            else {
                for i in 0..24 {
                    match placePosition.get_field().get_piece_at_tile(i) {
                        Piece::WHITE => {}
                        _ => {continue;}
                    }
                    for j in 0..24 {
                        if is_move_legal(Move::new(i.try_into().unwrap(), j.try_into().unwrap()), &placePosition.get_field()) {
                            if self.take.checkTake(&mut calculate_field_after_move(Move::new(i.try_into().unwrap(), j.try_into().unwrap()), placePosition.get_field())) {
                                //calculate which pieces are secure
                                let mut secureFromTake = self.take.secureFromTake(&mut placePosition.get_field());
                                for a in 0..24 {
                                    match placePosition.get_field().get_piece_at_tile(a) {
                                        Piece::BLACK => {
                                            let mut isSecure = false;
                                            for piece in &secureFromTake {
                                                if &a == piece {
                                                    isSecure = true;
                                                    break;
                                                }
                                            }
                                            if !(isSecure) {
                                                returnVector.push(Turn::new2(i.try_into().unwrap(), j.try_into().unwrap(), a.try_into().unwrap()));
                                            }
                                        }
                                        _ => {continue;}
                                    }
                                }
                            }
                            else {

                                returnVector.push(Turn::new(i.try_into().unwrap(), j.try_into().unwrap()));
                            }
                        }
                    }
                }
            }
        }
        else {

        }
        return returnVector;
    }
}