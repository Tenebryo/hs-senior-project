use std::ops::{AddAssign, Mul};

mod coordcube {

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Facelet {
        U,
        R,
        F,
        L,
        B,
        D,
    }


    pub struct CoordCube {
        data : [Face;54],
    }

    impl CoordCube {
        pub fn new() -> Cube {
            Cube {
                data: [
                    Face::U, Face::U, Face::U, Face::U, Face::U, Face::U, Face::U, Face::U, Face::U,
                    Face::R, Face::R, Face::R, Face::R, Face::R, Face::R, Face::R, Face::R, Face::R,
                    Face::F, Face::F, Face::F, Face::F, Face::F, Face::F, Face::F, Face::F, Face::F,
                    Face::D, Face::D, Face::D, Face::D, Face::D, Face::D, Face::D, Face::D, Face::D,
                    Face::L, Face::L, Face::L, Face::L, Face::L, Face::L, Face::L, Face::L, Face::L,
                    Face::B, Face::B, Face::B, Face::B, Face::B, Face::B, Face::B, Face::B, Face::B
                ]
            }
        }
    }

    impl PartialEq<CoordCube> for CoordCube {
        fn eq(&self, other: &CoordCube) -> bool {
            return self.data[0..27] == other.data[0..27] && self.data[27..54] == other.data[27..54];
        }
    }

    impl<'a> Mul<&'a Move> for CoordCube {
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
}