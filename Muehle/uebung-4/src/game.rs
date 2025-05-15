use std::{collections::HashMap, iter::FromIterator};

#[derive(Copy, Clone)]
pub enum Piece {
    NONE,
    WHITE,
    BLACK,
}

#[derive(Copy, Clone)]
pub struct Field {
    turn: bool,                         // 0 = white's turn, 1 = black's turn
    tiles: [Piece; 24],
    piecesOnField: u8,                  //first four bits for white, last four bits for black
}

impl Field {
    pub fn new() -> Field {
        return Field {
            turn: false,
            tiles: [Piece::NONE; 24],
            piecesOnField: 0,
        };
    }
    pub fn to_string(&self) -> String {
        let mut positionVector: Vec<char> = Vec::new();
        if self.turn {
            positionVector.push('B');
        }
        else {
            positionVector.push('W');
        }
        for tile in self.tiles {
            match tile {
                Piece::NONE => {positionVector.push('E');}
                Piece::BLACK => {positionVector.push('B');}
                Piece::WHITE => {positionVector.push('W');}
            }
        }
        return positionVector.iter().collect();
    }
    pub fn from_string(fieldAsString: String) -> Field {
        let mut converter: HashMap<usize, usize> = HashMap::new();
        converter.insert(1,1);
        converter.insert(2,2);
        converter.insert(3,14);
        converter.insert(4,23);
        converter.insert(5,22);
        converter.insert(6,21);
        converter.insert(7,9);
        converter.insert(8,0);
        converter.insert(9,4);
        converter.insert(10,5);
        converter.insert(11,13);
        converter.insert(12,20);
        converter.insert(13,19);
        converter.insert(14,18);
        converter.insert(15,10);
        converter.insert(16,3);
        converter.insert(17,7);
        converter.insert(18,8);
        converter.insert(19,12);
        converter.insert(20,17);
        converter.insert(21,16);
        converter.insert(22,15);
        converter.insert(23,11);
        converter.insert(24,6);

        let mut field = Field::new();
        let mut charIndex = 0;
        for c in fieldAsString.chars() {
            if charIndex == 0 {
                match c {
                    'B' => {field.set_turn(true);},
                    'W' => {field.set_turn(false);},
                    _ => {},
                }
            }
            else {
                //indexing starts with 1 but first 2 chars are not part of the field (first char used for turn, second is whitespace)
                let fieldIndex = charIndex-1;
                match c {
                    'B' => {field.set_piece_at(*converter.entry(fieldIndex.try_into().unwrap()).or_insert(100), Piece::BLACK)},
                    'W' => {field.set_piece_at(*converter.entry(fieldIndex.try_into().unwrap()).or_insert(100), Piece::WHITE)},  
                    _ => {},      
                }
            }
            charIndex += 1;
        }
        return field;
    }
    pub fn get_turn(&self) -> bool {
        return self.turn;
    }
    pub fn set_turn(&mut self, turn: bool) {
        self.turn = turn;
    }
    pub fn get_piece_at_tile(&self, tile: usize) -> Piece {
        return self.tiles[tile];
    }
    pub fn get_pieces_on_field(&self) -> u8 {
        return self.piecesOnField;
    }
    pub fn get_tiles(&self) -> [Piece; 24] {
        return self.tiles;
    }

    pub fn remove_piece(&mut self, position: usize) {
        self.tiles[position] = Piece::NONE;
        if self.turn {
            self.piecesOnField -= 1;
        }
        else {
            self.piecesOnField -= 16;
        }
    }
    pub fn set_piece_at(&mut self, position: usize, piece: Piece) {
        self.tiles[position] = piece;
    }
    pub fn set_piece(&mut self, position: usize) {
        if self.turn {
            self.tiles[position] = Piece::BLACK;
            self.piecesOnField += 16;
        }
        else {
            self.tiles[position] = Piece::WHITE;
            self.piecesOnField += 1;
        }
    }
    pub fn print_tiles(field: &Field) {
        let mut index = 0;
        for tile in field.tiles {
            match tile {
                Piece::NONE => println!("{},{}, ", index, 'O'),
                Piece::BLACK => println!("{},{}, ", index, 'B'),
                Piece::WHITE => println!("{},{}, ", index, 'W'),
            }
            index += 1;
        }
    }
    pub fn set_piece_on_field_counter(&mut self, counter: u8) {
        self.piecesOnField = counter;
    }
}

#[derive(Copy, Clone)]
pub struct PlacePosition {
    field: Field,
    //how many pieces are left to place
    piecesOnHand: u8,                   //first four bits for white, last four bits for black
}

impl PlacePosition {
    pub fn new() -> PlacePosition {
        return PlacePosition {
            field: Field::new(),
            piecesOnHand: 153,              //153_10 == 1001 1001_2 => both start with 9 pieces to place
        };
    }
    pub fn to_string(&self) -> String {
        return self.field.to_string();
    }
    pub fn get_field(&self) -> Field {
        return self.field;
    }
    pub fn get_turn(&self) -> bool {
        return self.field.get_turn();
    }
    pub fn get_pieces_on_hand(&self) -> u8 {
        return self.piecesOnHand;
    }
    pub fn set_field(&mut self, field: Field) {
        self.field = field;
    }
    pub fn set_turn(&mut self, turn: bool) {
        self.field.set_turn(turn);
    }
    pub fn set_piece_on_field_counter(&mut self, counter: u8) {
        self.field.set_piece_on_field_counter(counter);
    }
    pub fn set_piece(&mut self, position: usize) {
        self.field.set_piece(position);
        if self.get_turn() {
            self.piecesOnHand -= 16;
        }
        else {
            self.piecesOnHand -= 1;
        }
    }
    pub fn set_piece_on_hand(&mut self, piecesOnHand: u8) {
        self.piecesOnHand = piecesOnHand;
    }
    pub fn remove_piece(&mut self, position: usize) {
        self.field.remove_piece(position);
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    startTile: usize,
    endTile: usize,
}

impl Move {
    pub fn new(startTile: usize, endTile: usize) -> Move {
        return Move{
            startTile: startTile,
            endTile: endTile,
        }
    }
}

pub fn piece_on_position(tile: usize, field: &Field) -> Piece {
    return field.tiles[tile];
}

pub fn is_place_legal(placePosition: usize, field: &Field) -> bool {
    match field.tiles[placePosition] {
        Piece::NONE => return true,
        _ => return false,
    }
}

pub fn is_move_legal(movePiece: Move, field: &Field) -> bool {
    let array: [u8;130] = [24,32,0,1,0,9,1,0,1,2,1,4,2,1,2,14,3,4,3,10,4,1,4,3,4,5,4,7,
    5,4,5,13,6,7,6,11,7,4,7,6,7,8,8,7,8,12,9,0,9,10,9,21,10,3,10,9,10,11,10,18,
    11,6,11,10,11,15,12,8,12,13,12,17,13,5,13,12,13,14,13,20,14,2,14,13,14,23,15,11,
    15,16,16,15,16,17,16,19,17,12,17,16,18,10,18,19,19,16,19,18,19,20,19,22,20,13,
    20,19,21,9,21,22,22,19,22,21,22,23,23,14,23,22];       //Kantenliste

    let mut isNeighbour: bool = false;

    //Field::print_tiles(&field);

    //check, if piece belongs to the player, who wants to move
    if field.turn == true {
        match field.tiles[movePiece.startTile] {
            Piece::WHITE => {
                println!("Piece doesnt belong to player BLACK");
                return false;
            },
            _ => {}
        }
    }
    else {
        match field.tiles[movePiece.startTile] {
            Piece::BLACK => {
                println!("Piece doesnt belong to player WHITE");
                return false;
            }
            _ => {}
        }
    }
    //check, if end position is free
    match field.tiles[movePiece.endTile] {
        Piece::NONE => {
            
        },
        _ => {
            println!("Tile is not free.");
            return false;
        },
    }
    //check, if only three pieces left for player
    if field.get_turn() {
        let mut piecesLeft = field.get_pieces_on_field();
        piecesLeft = piecesLeft << 4;
        if piecesLeft == 48 {
            return true;
        }
    }
    else {
        let mut piecesLeft = field.get_pieces_on_field();
        piecesLeft = piecesLeft >> 4;
        if piecesLeft == 3 {
            return true;
        }
    }

    //check, if start and end position are neighbouring
    for n in 1..64 {
        if (array[n*2] as usize == movePiece.startTile) && (array[n*2+1] as usize == movePiece.endTile) {
            isNeighbour = true;
            break;
        }
    }    
    if isNeighbour == true {
        return true;
    }
    else {
        println!("Not a neighbour.");
        return false;
    }
    
}

pub fn calculate_field_after_place(tile: usize, placePosition: PlacePosition) -> PlacePosition {
    let mut newPosition: PlacePosition = placePosition;
    if is_place_legal(tile, &placePosition.field) {
        if placePosition.field.turn {
            //black's turn
            newPosition.field.tiles[tile] = Piece::BLACK;
            newPosition.field.piecesOnField += 1;
            newPosition.piecesOnHand -= 1;
        }
        else {
            //white's turn
            newPosition.field.tiles[tile] = Piece::WHITE;
            newPosition.field.piecesOnField += 16;
            newPosition.piecesOnHand -= 16;
        }
    }
    return newPosition;
}

pub fn calculate_field_after_move(movePiece: Move, field: Field) -> Field {
    let mut newPosition = field;
    if is_move_legal(movePiece, &field) {
        let movingPiece = field.tiles[movePiece.startTile];
        newPosition.tiles[movePiece.startTile] = Piece::NONE;
        newPosition.tiles[movePiece.endTile] = movingPiece;
    }
    return newPosition;
}

#[derive(Copy, Clone)]
pub enum Mode {
    PLACE_MODE, MOVE_MODE,
}