use std::collections::HashMap;

use crate::StringHandler::StringHandler;

pub struct NumerationConverter {
    converterTo: HashMap<u8, u8>,
    converterFrom: HashMap<u8, u8>,
}

impl NumerationConverter {
    pub fn new() -> NumerationConverter {
        let mut converter = NumerationConverter {
            converterFrom: HashMap::new(),
            converterTo: HashMap::new(),
        };
        //TODO initialise HashMap
        converter.converterTo.insert(0,8);
        converter.converterTo.insert(1,1);
        converter.converterTo.insert(2,2);
        converter.converterTo.insert(3,16);
        converter.converterTo.insert(4,9);
        converter.converterTo.insert(5,10);
        converter.converterTo.insert(6,24);
        converter.converterTo.insert(7,17);
        converter.converterTo.insert(8,18);
        converter.converterTo.insert(9,7);
        converter.converterTo.insert(10,15);
        converter.converterTo.insert(11,23);
        converter.converterTo.insert(12,19);
        converter.converterTo.insert(13,11);
        converter.converterTo.insert(14,3);
        converter.converterTo.insert(15,22);
        converter.converterTo.insert(16,21);
        converter.converterTo.insert(17,20);
        converter.converterTo.insert(18,14);
        converter.converterTo.insert(19,13);
        converter.converterTo.insert(20,12);
        converter.converterTo.insert(21,6);
        converter.converterTo.insert(22,5);
        converter.converterTo.insert(23,4);

        converter.converterFrom.insert(1,1);
        converter.converterFrom.insert(2,2);
        converter.converterFrom.insert(3,14);
        converter.converterFrom.insert(4,23);
        converter.converterFrom.insert(5,22);
        converter.converterFrom.insert(6,21);
        converter.converterFrom.insert(7,9);
        converter.converterFrom.insert(8,0);
        converter.converterFrom.insert(9,4);
        converter.converterFrom.insert(10,5);
        converter.converterFrom.insert(11,13);
        converter.converterFrom.insert(12,20);
        converter.converterFrom.insert(13,19);
        converter.converterFrom.insert(14,18);
        converter.converterFrom.insert(15,10);
        converter.converterFrom.insert(16,3);
        converter.converterFrom.insert(17,7);
        converter.converterFrom.insert(18,8);
        converter.converterFrom.insert(19,12);
        converter.converterFrom.insert(20,17);
        converter.converterFrom.insert(21,16);
        converter.converterFrom.insert(22,15);
        converter.converterFrom.insert(23,11);
        converter.converterFrom.insert(24,6);
        return converter;
    }

    pub fn convert_to(&self, stringToConvert: &mut String) -> String {
        let mut stringIterator = stringToConvert.split_whitespace();
        let mut resultVector: Vec<String> = Vec::new(); 
        for string in stringIterator {
            //check, if string is a number 
            if StringHandler::string_is_a_number(String::from(string)) {
                let asInt = string.trim().parse::<u8>().unwrap();
                if asInt > 23 {
                    println!("Error in conversion:Invalid input. Number is greater than 23.");
                    return String::new();
                }
                resultVector.push(self.converterTo[&asInt].to_string());
            }
            else {
                resultVector.push(String::from(string));
            }
        }
        //build string vom vector
        let mut resultString = String::new();
        for string in resultVector {
            resultString.push_str(&string);
            resultString.push(' ');
        }
        return resultString;
    }
    pub fn convert_from(&self, stringToConvert: &mut String) -> String {
        let mut stringIterator = stringToConvert.split_whitespace();
        let mut resultVector: Vec<String> = Vec::new(); 
        for string in stringIterator {
            //check, if string is a number 
            if StringHandler::string_is_a_number(String::from(string)) {
                let asInt = string.trim().parse::<u8>().unwrap();
                if asInt > 23 {
                    println!("Error in conversion:Invalid input. Number is greater than 23.");
                    return String::new();
                }
                resultVector.push(self.converterTo[&asInt].to_string());
            }
            else {
                resultVector.push(String::from(string));
            }
        }
        //build string vom vector
        let mut resultString = String::new();
        for string in resultVector {
            resultString.push_str(&string);
            resultString.push(' ');
        }
        return resultString;
    }
}