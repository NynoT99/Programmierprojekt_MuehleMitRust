use crate::game::Piece;

pub struct StringHandler {
    
}

impl StringHandler {
    pub fn string_is_a_number(string: String) ->  bool {
        let trimmedString = string.trim();
        let mut isNumeric = false;
        for c in trimmedString.chars() {
            if c.is_numeric() {
                isNumeric = true;
            }
            else {
                return false;      // character is not numeric => string is not a number
            }
        }
        //don't return true, when string is empty
        if isNumeric {
            return true;        
        }
        else {println!("String is empty.");
            return false;   //when string is empty
        }
    }

    pub fn place_piece_in_string(string: &mut String, index: usize, piece: Piece) {
        match piece {
            Piece::BLACK => {
                string.replace_range(index..index+1, "B");
            }
            Piece::WHITE => {
                string.replace_range(index..index+1, "W");
            }
            Piece::NONE => {
                string.replace_range(index..index+1, "O");
            }
        }
    }

    pub fn split_at_comma(string: &mut String) -> Vec<String> {
        let mut returnVector: Vec<String> = Vec::new();
        let mut indexOfComma = 0;
        let mut indexCounter = 0;
    
        for c in string.trim().chars() {
            if c == ',' {
                indexOfComma = indexCounter;
                break;
            }
            indexCounter += 1;
        }
        indexCounter -= 1;
        if indexOfComma > 0 {
            let mut firstCharArray: Vec<char> = Vec::new();
            let mut secondCharArray: Vec<char> = Vec::new();
            indexCounter = 0;
            for c in string.trim().chars() {
                if indexCounter < indexOfComma {
                    firstCharArray.push(c);
                }
                if indexCounter > indexOfComma {
                    secondCharArray.push(c);
                }
                indexCounter += 1;
            }
            indexCounter -= 1;
            returnVector.push(String::from_iter(firstCharArray));
            returnVector.push(String::from_iter(secondCharArray));
        }
        return returnVector;
    }
}