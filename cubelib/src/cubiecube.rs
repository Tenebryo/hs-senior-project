use std::ops::{AddAssign, Mul};

mod cubiecube {
    
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Corners {
        URF,
        UFL,
        ULB,
        UBR,
        DFR,
        DLF,
        DBL,
        DRB
    }
    
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Edges {
        UR,
        UF,
        UL,
        UB,
        DR,
        DF,
        DL,
        DB,
        FR,
        FL,
        LB,
        BR
    }
    
    #[derive(PartialEq, Eq, Copy, Clone)]
    struct Corner {
        d : u8
    }
    
    impl Corner {
        fn new(c : u8, o : u8) -> Corner {
            Corner {
                d: ((c%16)<<4) + (o%16)
            }
        }
        
        fn c(&self) -> u8 {
            self.d >> 4
        }
        
        fn o(&self) -> u8 {
            self.d % 16
        }
    }
    
    #[derive(PartialEq, Eq, Copy, Clone)]
    struct Edge {
        d : u8
    }
    
    impl Edge {
        fn new(c : u8, o : u8) -> Edge {
            Edge {
                d: ((c%16)<<4) + (o%16)
            }
        }
        
        fn e(&self) -> u8 {
            self.d >> 4
        }
        
        fn o(&self) -> u8 {
            self.d % 16
        }
    }
    
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub struct CubieCube {
        co : [Corner; 8],
        ed : [Edge; 12]
    }
    
    impl CubieCube {
        fn new() -> CubieCube {
            CubieCube {
                co : [
                    Corner::new(0, 0), //URF
                    Corner::new(1, 0), //UFL
                    Corner::new(2, 0), //ULB
                    Corner::new(3, 0), //UBR
                    Corner::new(4, 0), //DFR
                    Corner::new(5, 0), //DLF
                    Corner::new(6, 0), //DBL
                    Corner::new(7, 0), //DRB
                ],
                ed : [
                    Edge::new( 0, 0) //UR
                    Edge::new( 1, 0) //UF
                    Edge::new( 2, 0) //UL
                    Edge::new( 3, 0) //UB
                    Edge::new( 4, 0) //DR
                    Edge::new( 5, 0) //DF
                    Edge::new( 6, 0) //DL
                    Edge::new( 7, 0) //DB
                    Edge::new( 8, 0) //FR
                    Edge::new( 9, 0) //FL
                    Edge::new(10, 0) //LB
                    Edge::new(11, 0) //BR
                ]
            }
        }
        
        fn corner_orientation_coord(&self) -> u32 {
            let mut s = 0;
            for c in self.co.iter().take(7) {
                s = 3*s + c.o as u32;
            }
            s
        }
        
        fn edge_orientation_coord(&self) -> u32 {
            let mut s = 0;
            for e in self.ed.iter.take(11) {
                s = 2*s + e.o as u32;
            }
            s
        }
        
        fn corner_permutation_coord(&self) -> u32 {
            let mut s = 0;
            for (i,co) in self.co.iter().cloned().enumerate().skip(1) {
                s += util::factorial(i as u32)*self.co.iter().cloned().take(i).filter(|c| c.c() > co).count() as u32;
            }
            s
        }
        
        fn edge_permutation_coord(&self) -> u32 {
            let mut s = 0;
            for (i,ed) in self.ed.iter().cloned().enumerate().skip(1) {
                s += util::factorial(i as u32)*self.ed.iter().cloned().take(i).filter(|e| e.e() > ed).count() as u32;
            }
            s
        }
        
        fn ud_slice_coord(&self) -> u32 {
            self.ed.iter().enumerate().fold((0,-1), |(s, c), &(i, e)| {
                if 8 <= e && e < 12 {
                    (s, c+1)
                } else {
                    (s+util::binomial(i,c), c)
                }
            }).0
        }
        
        fn p2_edge_permutation_coord(&self) -> u32 {
            
        }
        
        fn p2_ud_slice_coord(&self) -> u32 {
            
        }
    }
}