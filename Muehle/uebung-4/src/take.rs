use std::collections::HashMap;
use crate::game::Piece;
use crate::game::Field;

#[derive(Clone)]
pub struct Take {
    takePositions: HashMap<usize, [usize; 3]>,
}

impl Take {
    pub fn new() -> Take {
        let mut take = Take { 
            takePositions: HashMap::new(),
        };
        take.takePositions.insert(0, [0,1,2]);
        take.takePositions.insert(1, [2,14,23]);
        take.takePositions.insert(2, [21,22,23]);
        take.takePositions.insert(3, [0,9,21]);
        take.takePositions.insert(4, [3,4,5]);
        take.takePositions.insert(5, [5,13,20]);
        take.takePositions.insert(6, [18,19,20]);
        take.takePositions.insert(7, [3,0,18]);
        take.takePositions.insert(8, [6,7,8]);
        take.takePositions.insert(9, [8,12,17]);
        take.takePositions.insert(10, [15,16,17]);
        take.takePositions.insert(11, [6,11,15]);
        take.takePositions.insert(12, [1,4,7]);
        take.takePositions.insert(13, [12,13,14]);
        take.takePositions.insert(14, [16,19,22]);
        take.takePositions.insert(15, [9,10,11]);
        take.takePositions.insert(16, [2,5,8]);
        take.takePositions.insert(17, [17,20,23]);
        take.takePositions.insert(18, [15,18,21]);
        take.takePositions.insert(19, [0,3,6]);

        return take;
    }

    pub fn secureFromTake(&mut self, field: &mut Field) -> Vec<usize> {
        let mut returnVector = Vec::new();
        //check which white pieces are secure from a take
        if field.get_turn() {
            for i in 0..20 {
                let mut muehle = false;
                for tile in self.takePositions[&i] {
                    match field.get_piece_at_tile(tile) {
                        Piece::WHITE => {muehle = true;
                        }
                        _ => {
                            muehle = false;
                            break;
                        }
                    }
                }
                if muehle == true {
                    for tile in self.takePositions[&i] {
                        returnVector.push(tile)
                    }
                }
            } 
        }
        //check which black pieces are secure from a take
        else {
            for i in 0..20 {
                let mut muehle = false;
                for tile in self.takePositions[&i] {
                    match field.get_piece_at_tile(tile) {
                        Piece::BLACK=> {
                            muehle = true;
                        }
                        _ => {
                            muehle = false;
                            break;
                        }
                    }
                }
                if muehle == true {
                    for tile in self.takePositions[&i] {
                        returnVector.push(tile)
                    }
                }
            } 
        }
        return returnVector;
    }
    
    pub fn checkTake(&mut self, field: &mut Field) -> bool {
        if field.get_turn() {
            for i in 0..20 {
                let mut take = false;
                for tile in self.takePositions[&i] {
                    match field.get_piece_at_tile(tile) {
                        Piece::BLACK => {take = true;
                        }
                        _ => {
                            take = false;
                            break;
                        }
                    }
                }
                if take == true {
                    return take;
                }
            } 
        }
        else {
            for i in 0..20 {
                let mut take = false;
                for tile in self.takePositions[&i] {
                    match field.get_piece_at_tile(tile) {
                        Piece::WHITE => {
                            take = true;
                        }
                        _ => {
                            take = false;
                            break;
                        }
                    }
                }
                if take == true {
                    return take;
                }
            } 
        }
        return false;
    }
}