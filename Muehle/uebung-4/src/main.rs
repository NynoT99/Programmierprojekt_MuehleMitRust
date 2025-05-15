mod playingField;
mod game;
mod StringHandler;
mod take;
mod turn;
mod FMIBeaver;
mod numerationConverter;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Clone)]
pub struct ConsoleGame {
    gameMode: game::Mode,
    placePosition: game::PlacePosition,
    take: take::Take,
    playingField: playingField::PlayingField,
}

impl ConsoleGame {
    pub fn new() -> ConsoleGame {
        return ConsoleGame{
            gameMode: game::Mode::PLACE_MODE,
            placePosition: game::PlacePosition::new(),
            take: take::Take::new(),
            playingField: playingField::PlayingField::new(),
        };
    }
    pub fn print_field_to_console(&self) {
        self.playingField.print_field_to_console();
    }
    pub fn set_field(&mut self, field: game::Field) {
        self.placePosition.set_field(field);
    }
    pub fn set_place_position(&mut self, newPlacePosition: game::PlacePosition) {
        self.placePosition = newPlacePosition;
    }
    pub fn get_turn(&mut self) -> bool {
        return self.placePosition.get_turn();
    }
    pub fn get_pieces_on_hand(&self) -> u8 {
        return self.placePosition.get_pieces_on_hand();
    }
    pub fn get_pieces_on_field(&self) -> u8 {
        return self.get_field().get_pieces_on_field();
    }
    pub fn get_game_mode(&self) -> game::Mode {
        return self.gameMode;
    }
    pub fn get_field(&self) -> game::Field {
        return self.placePosition.get_field();
    }
    pub fn get_playing_field(&mut self) -> &mut playingField::PlayingField {
        return &mut self.playingField;
    }
    pub fn get_place_position(&self) -> game::PlacePosition {
        return self.placePosition;
    }
    pub fn set_turn(&mut self, turn: bool) {
        self.placePosition.set_turn(turn);
    }
    pub fn set_piece(&mut self, position: usize) {
        self.placePosition.set_piece(position);
    }
    pub fn set_game_mode(&mut self, mode: game::Mode) {
        self.gameMode = mode;
    }
    pub fn remove_piece(&mut self, position: usize) {
        self.placePosition.remove_piece(position);
    }
    pub fn takePiece(&mut self) {
        let mut turn = String::new();
        let mut tile: usize = 0;

        match self.get_turn() {
            true => {turn = String::from("Black")},
            false => {turn = String::from("White")},
        }
        loop {
            let mut buffer = &mut String::new();
            let stdin = std::io::stdin();
            println!("{turn}'s turn: Enter position of the piece, to take.");
            stdin.read_line(&mut buffer);
            let trimmedString = buffer.trim().to_string();
            if StringHandler::StringHandler::string_is_a_number(trimmedString) {
                tile = buffer.trim().parse::<usize>().unwrap();
            }
            else {
                println!("Invalid input: Position must be a number.");
                continue;
            }
            if tile > 23 {
                println!{"Invalid input: Position can not be greater than 23."}
                continue;
            }
            match self.get_turn() {
                true => {
                    match self.get_field().get_piece_at_tile(tile) {
                        game::Piece::BLACK => {
                            println!("Invalid input: Can't remove your own piece.");
                            continue;
                        }
                        game::Piece::NONE => {
                            println!("Invalid input: No piece at this position.");
                            continue;
                        }
                        game::Piece::WHITE => {
                            let mut secureFromTake = self.take.secureFromTake(&mut self.get_field());
                            for piece in secureFromTake {
                                if tile != piece {
                                    println!("Invalid input: Can't take this piece. It is part of a chain.");
                                    continue;
                                }
                            }
                        }
                    }
                }
                false => {
                    match self.get_field().get_piece_at_tile(tile) {
                        game::Piece::WHITE => {
                            println!("Invalid input: Can't remove your own piece.");
                            continue;
                        }
                        game::Piece::NONE => {
                            println!("Invalid input: No piece at this position.");
                            continue;
                        }
                        game::Piece::BLACK => {}
                    }
                }
            }
            let pos = u32::try_from(tile);
            match pos {
                Ok(pos) => {
                    self.get_playing_field().overwrite_tile(pos, game::Piece::NONE);
                    self.remove_piece(tile);
                    break;
                }
                Err(_e) => {}
            }
        }
    }
    pub fn check_if_won(&mut self) -> bool {
        if self.get_turn() {
            let mut piecesLeft = self.get_pieces_on_field();
            piecesLeft = piecesLeft >> 4;
            println!("Pieces on field: {piecesLeft}");
            if piecesLeft <= 32 {
                return true;
            }
        }
        else {
            let mut piecesLeft = self.get_pieces_on_field();
            piecesLeft = piecesLeft << 4;
            println!("Pieces on field: {piecesLeft}");
            if piecesLeft <= 2 {
                return true;
            }
        }
        return false;
    }
}

fn main() -> std::io::Result<()> {
    /* 
    let mut consoleGame = ConsoleGame::new();
    let mut won = false;

    consoleGame.print_field_to_console();

    loop {
        if won {
            break;
        }
        if consoleGame.get_pieces_on_hand() == 0 {
            consoleGame.set_game_mode(game::Mode::MOVE_MODE);
        }

        if consoleGame.placePosition.get_turn() {
            println!("Black's turn");
        }
        else {
            println!("White's turn");
        }

        match consoleGame.gameMode {
            game::Mode::PLACE_MODE =>  {
                loop {
                    let mut buffer = &mut String::new();
                    let stdin = std::io::stdin();
                    let mut startPosition: usize = 0;

                    println!("Place piece by entering position.");
                    stdin.read_line(&mut buffer);
                    let trimmedString = buffer.trim().to_string();
                    if StringHandler::StringHandler::string_is_a_number(trimmedString) {
                        startPosition = buffer.trim().parse::<usize>().unwrap();
                    }
                    else {
                        println!("Invalid input: Position must be a number.");
                        continue;
                    }
                    if startPosition > 23 {
                        println!{"Invalid input: Position can not be greater than 23."}
                        continue;
                    }
                    if game::is_place_legal(startPosition, &consoleGame.placePosition.get_field()) {
                        //consoleGame.set_place_position(game::calculate_field_after_place(startPosition, consoleGame.placePosition));
                    }
                    else {
                        println!("Can not place a piece on this position. There is a piece already on that position");
                        continue;
                    }
                    //save position of set piece
                    consoleGame.set_piece(usize::from(startPosition));

                    let pos = u32::try_from(startPosition);
                    match pos {
                        Ok(pos) => {
                            let turn = consoleGame.get_turn();
                            consoleGame.get_playing_field().place_piece(turn, pos);

                            //take piece, if conditions is met
                            if consoleGame.take.checkTake(&mut consoleGame.get_field()) {
                                consoleGame.takePiece();
                            }

                            //switch turns
                            if consoleGame.get_turn() {
                                consoleGame.set_turn(false);
                            }
                            else {
                                consoleGame.set_turn(true);
                            }
                            break;
                        }
                        Err(_e) => {}
                    }
                }
            }
            game::Mode::MOVE_MODE => {
                loop {
                    let mut buffer = &mut String::new();
                    let stdin = std::io::stdin();
                    let mut startPosition: usize = 0;
                    let mut endPosition: usize = 0;

                    println!("Move piece by entering position of piece and end position seperated by comma.");
                    stdin.read_line(&mut buffer);
                    
                    let stringVector = StringHandler::StringHandler::split_at_comma(&mut buffer);

                    if stringVector.len() != 2 {
                        println!("Invalid input: {}", buffer);
                        continue;
                    }
                    
                    if StringHandler::StringHandler::string_is_a_number(stringVector[0].clone()) 
                    && StringHandler::StringHandler::string_is_a_number(stringVector[1].clone()) {
                        startPosition = stringVector[0].parse::<usize>().unwrap();
                        endPosition = stringVector[1].parse::<usize>().unwrap();
                    }
                    else {
                        println!("Invalid input: Position must be two numbers seperated by comma. {}, {}", stringVector[0], stringVector[1]);
                        continue;
                    }
                    if startPosition > 23 || endPosition > 23 {
                        println!{"Invalid input: Position can not be greater than 23."}
                        continue;
                    }

                    if game::is_move_legal(game::Move::new(startPosition, endPosition), &consoleGame.placePosition.get_field()) {
                        //consoleGame.set_place_position(game::calculate_field_after_place(startPosition, consoleGame.placePosition));
                    }
                    else {
                        println!("Can not place a piece on this position.");
                        continue;
                    }

                    //save position of set piece
                    consoleGame.set_field(game::calculate_field_after_move(game::Move::new(startPosition, endPosition), consoleGame.get_field()));

                    let mut pos = u32::try_from(startPosition);
                    let mut start = 0;
                    let mut end = 0;
                    match pos {
                        Ok(pos) => {
                            start = pos;
                        }
                        Err(_e) => {}
                    }
                    pos = u32::try_from(endPosition);
                    match pos {
                        Ok(pos) => {
                            end = pos;

                            let turn = consoleGame.get_turn();
                            consoleGame.get_playing_field().place_piece(turn, pos);
                            consoleGame.get_playing_field().overwrite_tile(start, game::Piece::NONE);

                            //take piece, if conditions is met
                            if consoleGame.take.checkTake(&mut consoleGame.get_field()) {
                                consoleGame.takePiece();
                            }

                            //check, if won
                            if consoleGame.check_if_won() {
                                won = true;
                                break;
                            }

                            //switch turns
                            if consoleGame.get_turn() {
                                consoleGame.set_turn(false);
                            }
                            else {
                                consoleGame.set_turn(true);
                            }
                            break;
                        }
                        Err(_e) => {}
                    }
                }
            }
        }

        consoleGame.print_field_to_console();  
    }

    if consoleGame.get_turn() {
        println!("Game over! Black won!");
    }
    else {
        println!("Game over! White won!");
    }

    */

    let mut fmiBeaver = FMIBeaver::FMIBeaver::new();
    let mut placePosition = game::PlacePosition::new();
    placePosition.set_piece_on_hand(0);
    let converter = numerationConverter::NumerationConverter::new();

    let file = File::open("Testdaten_Blatt_4/input_felder.txt")?;
    //let file = File::open("input_felder.txt")?;
    
    let reader = BufReader::new(file);

    let path = "output.txt";
    let mut output = File::create(path)?;

    for line in reader.lines() {
        let mut result: Vec<turn::Turn> = Vec::new();
        let input = line;
        match input {
            Ok(input) => {
                    write!(output, "{input}\n");
                    placePosition.set_field(game::Field::from_string(input));
                    //count how many pieces are on the field for each player
                    for tile in 0..24 {
                        match placePosition.get_field().get_piece_at_tile(tile) {
                            game::Piece::BLACK => {
                                placePosition.set_piece_on_field_counter(placePosition.get_field().get_pieces_on_field() + 1);
                            }
                            game::Piece::WHITE => {
                                placePosition.set_piece_on_field_counter(placePosition.get_field().get_pieces_on_field() + 16);
                            }
                            _ => {}
                        }
                    }
                    //game::Field::print_tiles(&placePosition.get_field());
                    result.append(&mut fmiBeaver.ask_the_beaver(&placePosition));

                    for turn in result {
                        //println!("{}", turn.to_string());
                        write!(output, "{}\n", converter.convert_to(&mut turn.to_string()));
                    }
                }
            Err(_e) => {}
        }
    }

    // if result.is_empty() {
    //     println!("Vector is empty!");
    // }
    
    Ok(())
}


