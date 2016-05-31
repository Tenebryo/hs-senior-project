use std::ops::{AddAssign, Mul};
use util;
use coordcube;
//use std::sync::Arc;

pub struct CubieCubeStatic {
    cube_sym                    : [CubieCube; 16],
    move_cube                   : [CubieCube; 18],
    move_cube_sym               : [i64; 18],
    first_move_sym              : [i32; 48],
            
    pre_move                    : [i8; 9],
            
    sym_inv                     : [i32; 16],
    sym_mult                    : [[i32; 16]; 16],
    sym_move                    : [[i32; 18]; 16],
    sym_mult_inv                : [[i32; 16]; 16],
    sym_8_mult                  : [i32; 64],
    sym_8_move                  : [i32; 144],
    sym_8_mult_inv              : [i32; 64],
    sym_move_ud                 : [[i32; 10]; 16],
            
    flip_s2r                    : [char; 336],
    twist_s2r                   : [char; 324],
    e_perm_s2r                  : [char; 2768],
    ud_slice_flip_s2r           : [i32; 64430],
            
    e2c                         : [u8; 16],
            
    m_to_e_perm                 : [char; 40320],
    
    flip_slice_2_ud_slice_flip  : [i32; coordcube::N_FLIP_SYM*coordcube::N_SLICE],
    
    flip_r2s                    : [char; 2048],
    twist_r2s                   : [char; 2187],
    e_perm_r2s                  : [char; 40320],
    flip_s2rf                   : [char; 336*8],
    twist_s2rf                  : [char; 324*8],
    
    sym_state_twist             : [char; 324],
    sym_state_flip              : [char; 336],
    sym_state_perm              : [char; 2768],
    sym_state_ud_slice_flip     : [char; 64430],
    
    urf1                        : CubieCube,
    urf2                        : CubieCube,
    urf_move                    : [[u8; 18]; 6],
}

impl CubieCubeStatic{
    fn new() -> CubieCubeStatic {
        CubieCubeStatic {
            cube_sym                    : [CubieCube::new(); 16],
            move_cube                   : [CubieCube::new(); 18],
            move_cube_sym               : [0; 18],
            first_move_sym              : [0; 48],
                    
            pre_move                    : [0; 9],
                    
            sym_inv                     : [0; 16],
            sym_mult                    : [[0; 16]; 16],
            sym_move                    : [[0; 18]; 16],
            sym_mult_inv                : [[0; 16]; 16],
            sym_8_mult                  : [0; 64],
            sym_8_move                  : [0; 144],
            sym_8_mult_inv              : [0; 64],
            sym_move_ud                 : [[0; 10]; 16],
                    
            flip_s2r                    : [' '; 336],
            twist_s2r                   : [' '; 324],
            e_perm_s2r                  : [' '; 2768],
            ud_slice_flip_s2r           : [0; 64430],
                    
            e2c                         : [0; 16],
                    
            m_to_e_perm                 : [' '; 40320],
            
            flip_slice_2_ud_slice_flip  : [0; coordcube::N_FLIP_SYM*coordcube::N_SLICE],
            
            flip_r2s                    : [' '; 2048],
            twist_r2s                   : [' '; 2187],
            e_perm_r2s                  : [' '; 40320],
            flip_s2rf                   : [' '; 336*8],
            twist_s2rf                  : [' '; 324*8],
            
            sym_state_twist             : [' '; 324],
            sym_state_flip              : [' '; 336],
            sym_state_perm              : [' '; 2768],
            sym_state_ud_slice_flip     : [' '; 64430],
            
            urf1                        : CubieCube::new(),
            urf2                        : CubieCube::new(),
            urf_move                    : [[0; 18]; 6],
        }
    }
    
    pub fn init() {
        unsafe {
            _cc_static = Some(CubieCubeStatic::new());
        }
    }
    
    fn corn_mult(&self, a : &CubieCube, b : &CubieCube, prod : &mut CubieCube) {
        for corn in 0..8 {
            let oriA = a.ca[(b.ca[corn] & 7) as usize] >> 3;
            let oriB = b.ca[corn] >> 3;
            let mut ori = oriA;
            ori += if oriA < 3 { oriB } else { 6 - oriB};
            ori %= 3;
            if ((oriA >= 3) ^ (oriB >= 3)) {
                ori += 3;
            }
            prod.ca[corn] = (a.ca[(b.ca[corn] & 7) as usize] & 7 | ori << 3) as u8;
        }
    }

    fn edge_mult(&self, a : &CubieCube, b : &CubieCube, prod : &mut CubieCube) {
        for ed in 0..12 {
            prod.ea[ed] = (a.ea[(b.ea[ed] >> 1) as usize] ^ (b.ea[ed] & 1)) as u8;
        }
    }

    fn corn_conjugate(&self, a : &CubieCube, idx : i32, b : &mut CubieCube) {
        let sinv = self.cube_sym[cc_static().sym_inv[idx as usize] as usize];
        let s = self.cube_sym[idx as usize];
        for corn in 0..8 {
            let oriA = sinv.ca[(a.ca[(s.ca[corn] & 7) as usize] & 7) as usize] >> 3;
            let oriB = a.ca[(s.ca[corn] & 7) as usize] >> 3;
            let ori = if oriA < 3 { oriB } else { (3 - oriB) % 3};
            b.ca[corn] = (sinv.ca[(a.ca[(s.ca[corn] & 7) as usize] & 7) as usize] & 7 | ori << 3) as u8;
        }
    }
    fn edge_conjugate(&self, a : &CubieCube, idx : i32, b : &mut CubieCube) {
        let sinv = self.cube_sym[(cc_static().sym_inv[idx as usize]) as usize];
        let s = self.cube_sym[idx as usize];
        for ed in 0..12 {
            b.ea[ed] = (sinv.ea[(a.ea[(s.ea[ed] >> 1) as usize] >> 1) as usize] ^ (a.ea[(s.ea[ed] >> 1) as usize] & 1) ^ (s.ea[ed] & 1)) as u8;
        }
    }
}

static mut _cc_static : Option<CubieCubeStatic> = None; 

#[inline]
fn cc_static() -> CubieCubeStatic {
    _cc_static.unwrap()
}

#[derive(Clone)]
pub struct CubieCube {
    ca      : [u8; 8],
    ea      : [u8; 12],
    temps   : Option<Box<CubieCube>>,
}

impl CubieCube {
    
    pub fn new() -> CubieCube {
        CubieCube {
            ca      : [0,1,2,3,4,5,6,7],
            ea      : [0,2,4,6,8,10,12,14,16,18,20,22],
            temps   : None,
        }
    }

    pub fn from_coord(cperm : i32, twist : i32, eperm : i32, flip : i32) -> CubieCube {
        let mut c = CubieCube::new();
        c.set_c_perm(cperm);
        c.set_twist(twist);
        util::set_n_perm(&mut c.ea, eperm, 12, true);
        c.set_flip(flip);
        c
    }

    pub fn equals_corn(&self, c : &CubieCube) -> bool {
        for i in 0..8 {
            if self.ca[i] != c.ca[i] {
                return false;
            }
        }
        return true;
    }

    pub fn equals_edge(&self, c : &CubieCube) -> bool {
        for i in 0..12 {
            if self.ea[i] != c.ea[i] {
                return false;
            }
        }
        return true;
    }

    
    fn copy(&mut self, c : &CubieCube) {
        for i in 0..8 {
            self.ca[i] = c.ca[i];
        }
        for i in 0..12 {
            self.ea[i] = c.ea[i];
        }
    }
    
    fn inv_cubie_cube(&mut self) {
        match self.temps {
            None => {
                self.temps = Some(Box::new(CubieCube::new()));
            },
            _ => ()
        }
        for edge in 0u8..12 {
            self.temps.unwrap().ea[(self.ea[edge as usize] >> 1) as usize] =  (edge << 1 | self.ea[edge as usize] & 1) as u8;
        }
        for corn in 0u8..8 {
            let mut ori = self.ca[corn as usize] >> 3;
            ori = 4 >> ori & 3; //0->0, 1->2, 2->1
            self.temps.unwrap().ca[(self.ca[corn as usize] & 0x7) as usize] = (corn | ori << 3) as u8;
        }
        self.copy(&*self.temps.unwrap());
    }
    
    fn urf_conjugate(&mut self) {
        match self.temps {
            None => {
                self.temps = Some(Box::new(CubieCube::new()));
            },
            _ => ()
        }
        cc_static().corn_mult(&cc_static().urf2,    &self,              &mut *self.temps.unwrap());
        cc_static().corn_mult(&*self.temps.unwrap(),&cc_static().urf1,  self);
        cc_static().edge_mult(&cc_static().urf2,    &self,              &mut *self.temps.unwrap());
        cc_static().edge_mult(&*self.temps.unwrap(),&cc_static().urf1,  self);
    }

    fn get_flip(&self) -> i32 {
        let mut idx = 0;
        for i in 0..11 {
            idx = idx << 1 | self.ea[i] & 1;
        }
        return idx as i32;
    }

    fn set_flip(&self, mut idx : i32) {
        let mut parity = 0;
        for i in (0..11).rev() {
            let val = idx & 1;
            self.ea[i] = ((self.ea[i] & 0xfe) as i32 | val) as u8;
            parity ^= val;
            idx >>= 1;
        }
        self.ea[11] = ((self.ea[11] & 0xfe) as i32 | parity) as u8;
    }

    fn get_flip_sym(&self) -> i32 {
        return cc_static().flip_r2s[self.get_flip() as usize] as i32;
    }

    fn get_twist(&self) -> i32 {
        let mut idx = 0;
        for i in 0..7 {
            idx += (idx << 1) + (self.ca[i] >> 3);
        }
        return idx as i32;
    }

    fn set_twist(&self, mut idx : i32) {
        let mut twst = 0;
        for i in (0..7).rev() {
            let val = idx % 3;
            self.ca[i] = ((self.ca[i] & 0x7) as i32 | val << 3) as u8;
            twst += val;
            idx /= 3;
        }
        self.ca[7] = ((self.ca[7] & 0x7) as i32 | ((15 - twst) % 3) << 3) as u8;
    }

    fn get_twist_sym(&self) -> i32 {
        return cc_static().twist_r2s[self.get_twist() as usize] as i32;
        //removed things
    }

    fn get_ud_slice(&mut self) -> i32 {
        util::get_comb(&mut self.ea, 8, true)
    }

    fn set_ud_slice(&mut self, idx : i32) {
        util::set_comb(&mut self.ea, idx, 8, true);
    }

    fn get_u_4_comb(&mut self) -> i32{
        util::get_comb(&mut self.ea, 0, true)
    }

    fn get_d_4_comb(&mut self) -> i32 {
        util::get_comb(&mut self.ea, 4, true)
    }
    
    fn get_c_perm(&mut self) -> i32 {
        util::get_8_perm(&mut self.ca, false)
    }

    fn set_c_perm(&mut self, idx : i32) {
        util::set_8_perm(&mut self.ca, idx, false);
    }

    fn get_c_permSym(&self) -> i32{
        let idx = cc_static().e_perm_r2s[self.get_c_perm() as usize] as u8;
        return (idx ^ cc_static().e2c[(idx & 0xf) as usize]) as i32;
        //removed some stuff
    }

    fn get_e_perm(&mut self) -> i32 {
        util::get_8_perm(&mut self.ea, true)
    }

    fn set_e_perm(&mut self, idx : i32) {
        util::set_8_perm(&mut self.ea, idx, true);
    }

    fn get_e_perm_sym(&self) -> i32 {
        return cc_static().e_perm_r2s[self.get_e_perm() as usize] as i32;
        //removed some stuff
    }

    fn get_m_perm(&mut self) -> i32 {
        util::get_comb(&mut self.ea, 8, true) >> 9
    }

    fn set_m_perm(&mut self, idx : i32) {
        util::set_comb(&mut self.ea, idx << 9, 8, true);
    }

    fn get_c_comb(&mut self) -> i32 {
        return 69 - (util::get_comb(&mut self.ca, 0, false) & 0x1ff);
    }

    fn set_c_comb(&mut self, idx : i32) {
        util::set_comb(&mut self.ca, 69 - idx, 0, false);
    }

    /**
     * Check a cubiecube for solvability. Return the error code.
     * 0: Cube is solvable
     * -2: Not all 12 edges exist exactly once
     * -3: Flip error: One edge has to be flipped
     * -4: Not all corners exist exactly once
     * -5: Twist error: One corner has to be twisted
     * -6: Parity error: Two corners or two edges have to be exchanged
     */
    fn verify(&self) -> i32{
        let mut sum = 0;
        let mut edgeMask = 0;
        for e in 0..12 {
            edgeMask |= 1 << (self.ea[e] >> 1);
            sum ^= self.ea[e] & 1;
        }
        if edgeMask != 0xfff {
            return -2;// missing edges
        }
        if sum != 0 {
            return -3;
        }
        let mut cornMask = 0;
        sum = 0;
        for c in 0..8 {
            cornMask |= 1 << (self.ca[c] & 7);
            sum += self.ca[c] >> 3;
        }
        if cornMask != 0xff {
            return -4;// missing corners
        }
        if sum % 3 != 0 {
            return -5;// twisted corner
        }
        if (util::get_n_parity(util::get_n_perm(&mut self.ea, 12, true), 12) ^ util::get_n_parity(self.get_c_perm(), 8)) != 0 {
            return -6;// parity error
        }
        return 0;// cube ok
    }

    fn self_symmetry(&self) -> i64 {
        let mut c = self.clone();
        let mut d = CubieCube::new();
        let mut sym = 0i64;
        for i in 0..48 {
            cc_static().corn_conjugate(&c, cc_static().sym_inv[i % 16], &mut d);
            if d.equals_corn(&self) {
                cc_static().edge_conjugate(&c, cc_static().sym_inv[i % 16], &mut d);
                if d.equals_edge(&self) {
                    sym |= 1i64 << i;
                }
            }
            if i % 16 == 15 {
                c.urf_conjugate();
            }
        }
        c.inv_cubie_cube();
        for i in 0..48 {
            cc_static().corn_conjugate(&c, cc_static().sym_inv[i % 16], &mut d);
            if d.equals_corn(&self) {
                cc_static().edge_conjugate(&c, cc_static().sym_inv[i % 16], &mut d);
                if d.equals_edge(&self) {
                    sym |= 1i64 << 48;
                    break;
                }
            }
            if (i % 16 == 15) {
                c.urf_conjugate();
            }
        }
        return sym;
    }

    fn set_ud_slice_flip(&self, idx : i32) {
        self.set_flip(idx & 0x7ff);
        self.set_ud_slice(idx >> 11);
    }

    fn get_ud_slice_flip(&self) -> i32{
        return (self.get_ud_slice() & 0x1ff) << 11 | self.get_flip();
    }
/*
    fn get_ud_slice_flip_sym(&self) -> i32{
        let mut flip = self.get_flip_sym();
        let fsym = flip & 0x7;
        flip >>= 3;
        let udslice = self.get_ud_slice() & 0x1ff;
        let udsliceflip = cc_static().flip_slice_2_ud_slice_flip[flip * 495 + CoordCube.UDSliceConj[udslice][fsym]];
        return udsliceflip & 0xfffffff0 | cc_static().sym_mult[udsliceflip & 0xf][fsym << 1];
    }*/
}

/*
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
}*/