use std::ops::{AddAssign, Mul};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Face {
    Red,
    Green,
    Blue,
    White,
    Yellow,
    Orange,
}

pub struct Cube {
    data : [Face;54],
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            data: [
                Face::White, Face::White, Face::White, Face::White, Face::White, Face::White, Face::White, Face::White, Face::White,
                Face::Red, Face::Red, Face::Red, Face::Red, Face::Red, Face::Red, Face::Red, Face::Red, Face::Red,
                Face::Yellow, Face::Yellow, Face::Yellow, Face::Yellow, Face::Yellow, Face::Yellow, Face::Yellow, Face::Yellow, Face::Yellow,
                Face::Blue, Face::Blue, Face::Blue, Face::Blue, Face::Blue, Face::Blue, Face::Blue, Face::Blue, Face::Blue,
                Face::Orange, Face::Orange, Face::Orange, Face::Orange, Face::Orange, Face::Orange, Face::Orange, Face::Orange, Face::Orange,
                Face::Green, Face::Green, Face::Green, Face::Green, Face::Green, Face::Green, Face::Green, Face::Green, Face::Green
            ]
        }
    }
}

impl PartialEq<Cube> for Cube {
    fn eq(&self, other: &Cube) -> bool {
        return self.data[0..27] == other.data[0..27] && self.data[27..54] == other.data[27..54];
    }
}

impl<'a> Mul<&'a Move> for Cube {
    type Output = Cube;
    
    fn mul(self, m: &'a Move) -> Cube {
        let mut r = Cube::new();
        
        for (i,v) in self.data.iter().enumerate() {
            r.data[m.data[i] as usize] = *v;
        }
        
        r
    }
}

pub struct Move {
    data : [u8;54]
}

impl Move {
    pub fn new() -> Move {
        let mut t = Move {
            data: [0;54]
        };
        for i in 0..54 {
            t.data[i]=i as u8;
        }
        t
    }
    
    pub fn u() -> Move {Move::new()}
    pub fn f() -> Move {Move::new()}
    pub fn l() -> Move {Move::new()}
    pub fn b() -> Move {Move::new()}
    pub fn r() -> Move {Move::new()}
    pub fn d() -> Move {Move::new()}
}

impl Mul for Move {
    type Output = Move;
    
    fn mul(self, m: Move) -> Move {
        let mut r = Move::new();
        
        for (i,v) in self.data.iter().enumerate() {
            r.data[m.data[*v as usize] as usize] = i as u8;
        }
        
        r
    }
}

impl Clone for Move {
    fn clone(&self) -> Move {
        let t0 : Vec<_> = self.data.iter().cloned().collect();
        let t1 : [u8] = *t0.as_slice();
        let t2 : [u8; 54] = t1[0..54];
        Move {
            data: t1
        }
    }
}

pub struct Manuever {
    moves : Vec<Move>
}

impl Manuever {
    pub fn new() -> Manuever {
        Manuever {
            moves: vec![]
        }
    }
}

impl<'a> AddAssign<&'a Move> for Manuever {
    fn add_assign(&mut self, m : &'a Move) {
        self.moves.push(m);
    }
}
