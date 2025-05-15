pub struct Turn {
    pos1: u8,
    pos2: u8,
    take: bool,
    pos3: u8,
}

impl Turn {
    pub fn new(pos1: u8, pos2: u8) -> Turn {
        return Turn {
            pos1: pos1,
            pos2: pos2,
            take: false,
            pos3: 0,
        };
    }
    pub fn new2(pos1: u8, pos2: u8, pos3: u8) -> Turn {
        return Turn {
            pos1: pos1,
            pos2: pos2,
            take: true,
            pos3: pos3,
        };
    }
    
    pub fn to_string(&self) -> String {
        if self.take {
            return format!("M {} {} T {}", self.pos1.to_string(), self.pos2, self.pos3.to_string());
        }
        else {
            return format!("M {} {}", self.pos1.to_string(), self.pos2);
        }
    }
}