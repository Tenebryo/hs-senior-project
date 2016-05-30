mod util {
    
    //moves
    const Ux1 : u8 = 0;
    const Ux2 : u8 = 1;
    const Ux3 : u8 = 2;
    const Rx1 : u8 = 3;
    const Rx2 : u8 = 4;
    const Rx3 : u8 = 5;
    const Fx1 : u8 = 6;
    const Fx2 : u8 = 7;
    const Fx3 : u8 = 8;
    const Dx1 : u8 = 9;
    const Dx2 : u8 = 10;
    const Dx3 : u8 = 11;
    const Lx1 : u8 = 12;
    const Lx2 : u8 = 13;
    const Lx3 : u8 = 14;
    const Bx1 : u8 = 15;
    const Bx2 : u8 = 16;
    const Bx3 : u8 = 17;
    
    //facelets
    const U1 : u8 = 0;
    const U2 : u8 = 1;
    const U3 : u8 = 2;
    const U4 : u8 = 3;
    const U5 : u8 = 4;
    const U6 : u8 = 5;
    const U7 : u8 = 6;
    const U8 : u8 = 7;
    const U9 : u8 = 8;
    const R1 : u8 = 9;
    const R2 : u8 = 10;
    const R3 : u8 = 11;
    const R4 : u8 = 12;
    const R5 : u8 = 13;
    const R6 : u8 = 14;
    const R7 : u8 = 15;
    const R8 : u8 = 16;
    const R9 : u8 = 17;
    const F1 : u8 = 18;
    const F2 : u8 = 19;
    const F3 : u8 = 20;
    const F4 : u8 = 21;
    const F5 : u8 = 22;
    const F6 : u8 = 23;
    const F7 : u8 = 24;
    const F8 : u8 = 25;
    const F9 : u8 = 26;
    const D1 : u8 = 27;
    const D2 : u8 = 28;
    const D3 : u8 = 29;
    const D4 : u8 = 30;
    const D5 : u8 = 31;
    const D6 : u8 = 32;
    const D7 : u8 = 33;
    const D8 : u8 = 34;
    const D9 : u8 = 35;
    const L1 : u8 = 36;
    const L2 : u8 = 37;
    const L3 : u8 = 38;
    const L4 : u8 = 39;
    const L5 : u8 = 40;
    const L6 : u8 = 41;
    const L7 : u8 = 42;
    const L8 : u8 = 43;
    const L9 : u8 = 44;
    const B1 : u8 = 45;
    const B2 : u8 = 46;
    const B3 : u8 = 47;
    const B4 : u8 = 48;
    const B5 : u8 = 49;
    const B6 : u8 = 50;
    const B7 : u8 = 51;
    const B8 : u8 = 52;
    const B9 : u8 = 53;
    
    const U : u8 = 0;
    const R : u8 = 0;
    const F : u8 = 0;
    const D : u8 = 0;
    const L : u8 = 0;
    const B : u8 = 0;

    const fact : [u32; 14]  = [
        1,1,2,6,24,120,720,5040,40320,362880,3628800,39916800,479001600,6227020800
    ];
    
    const c_nk : [[u32; 13]; 13] = [
        [1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [1,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [1,   2,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [1,   3,   3,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0],
        [1,   4,   6,   4,   1,   0,   0,   0,   0,   0,   0,   0,   0],
        [1,   5,  10,  10,   5,   1,   0,   0,   0,   0,   0,   0,   0],
        [1,   6,  15,  20,  15,   6,   1,   0,   0,   0,   0,   0,   0],
        [1,   7,  21,  35,  35,  21,   7,   1,   0,   0,   0,   0,   0],
        [1,   8,  28,  56,  70,  56,  28,   8,   1,   0,   0,   0,   0],
        [1,   9,  36,  84, 126, 126,  84,  36,   9,   1,   0,   0,   0],
        [1,  10,  45, 120, 210, 252, 210, 120,  45,  10,   1,   0,   0],
        [1,  11,  55, 165, 330, 462, 462, 330, 165,  55,  11,   1,   0],
        [1,  12,  66, 220, 495, 792, 924, 792, 495, 220,  66,  12,   1]
    ];

    struct Util {
        corner_facelet : [[u8;3];8],
        edge_facelet : [[u8; 2]; 12],
        perm_mult : [[u32;24]; 24],
        pre_move : [i8; 9],
        ud2std : [i8; 10],
        std2ud : [i8; 18],
        ckmv2 : [[bool;11];10],
    }
    
    impl Util {
        fn new() -> Util {
            let mut perm_mult = [[0u32;24];24];
            let ud2std = [
                Ux1 as i8,
                Ux2 as i8,
                Ux3 as i8,
                Rx2 as i8,
                Fx2 as i8,
                Dx1 as i8,
                Dx2 as i8,
                Dx3 as i8,
                Lx2 as i8,
                Bx2 as i8
            ];
            let mut std2ud = [-1i8; 18];
            let mut ckmv2 = [[false;11];10];
            
            for i in 0..10 {
                std2ud[ud2std[i] as usize] = i as i8;
            }
            
            for i in 0..10 {
                let ix = ud2std[i];
                for j in 0..10 {
                    let jx = ud2std[j];
                    ckmv2[i][j] = (ix / 3 == jx / 3) || ((ix / 3 % 3 == jx / 3 % 3) && (ix >= jx));
                }
                ckmv2[10][i] = false;
            }
            
            Util {
                corner_facelet : [
                    [U9, R1, F3],
                    [U7, F1, L3],
                    [U1, L1, B3],
                    [U3, B1, R3],
                    [D3, F9, R7],
                    [D1, L9, F7],
                    [D7, B9, L7],
                    [D9, R9, B7]
                ],
                edge_facelet : [
                    [U6, R2],
                    [U8, F2],
                    [U4, L2],
                    [U2, B2],
                    [D6, R8],
                    [D2, F8],
                    [D4, L8],
                    [D8, B8],
                    [F6, R4],
                    [F4, L6],
                    [B6, L4],
                    [B4, R6]
                ],
                perm_mult : perm_mult,
                pre_move : [
                    -1,
                    Rx1 as i8,
                    Rx3 as i8,
                    Fx1 as i8,
                    Fx3 as i8,
                    Lx1 as i8,
                    Lx3 as i8,
                    Bx1 as i8,
                    Bx3 as i8
                ],
                ud2std : ud2std,
                std2ud : std2ud,
                ckmv2 : ckmv2
            }
        }
        
        /////////////////////////////////////////////////////////////////////////////////////////////////
        
        fn get_n_parity(mut idx : u32, n : u32) -> u32 {
            let mut p = 0;
            for i in (0..(n-1)).rev() {
                p ^= idx % (n - i);
                idx /= (n - i);
            }
            return p & 1;
        }

        fn set_val(val0 : u32, val : u32, isEdge : bool) -> u8 {
            (if isEdge { val << 1 | val0 & 1 } else { val | val0 & 0xf8 }) as u8
        }

        fn get_val(val0 : u32, isEdge : bool) -> u32 {
            if isEdge { val0 >> 1 } else { val0 & 7 }
        }

        fn set_8_perm(arr : &mut [u8], mut idx : u32, isEdge : bool) {
            let mut val = 0x76543210;
            for i in 0..7 {
                let p = fact[7 - i];
                let mut v = idx / p;
                idx -= v * p;
                v <<= 2;
                arr[i] = Util::set_val(arr[i] as u32, (val >> v & 0x7), isEdge);
                let m = (1 << v) - 1;
                val = val & m | val >> 4 & (!m);
            }
            arr[7] = Util::set_val(arr[7] as u32, val, isEdge);
        }

        fn get_8_perm(arr : &mut [u8], isEdge : bool) -> u32 {
            let mut idx = 0;
            let mut val = 0x76543210;
            for i in 0..7 {
                let v = Util::get_val(arr[i] as u32, isEdge) << 2;
                idx = (8 - i) * idx + (val >> v & 0x7);
                val -= 0x11111110 << v;
            }
            return idx as u32;
        }

        fn set_n_perm(arr : &mut [u8], mut idx : u32, n : u32, isEdge : bool) {
            arr[(n - 1) as usize] = Util::set_val(arr[(n - 1) as usize] as u32, 0, isEdge);
            for i in (0..(n-1)).rev() {
                let arri = idx % (n - i);
                arr[i as usize] = Util::set_val(arr[i as usize] as u32, arri, isEdge);
                idx /= (n - i);
                for j in (i+1)..n {
                    let mut arrj = Util::get_val(arr[j as usize] as u32, isEdge);
                    if (arrj >= arri) {
                        arrj += 1;
                        arr[j as usize] = Util::set_val(arr[j as usize] as u32, arrj, isEdge);
                    }
                }
            }
        }

        fn get_n_perm(arr : &mut [u8], n : u32, isEdge : bool) -> u32{
            let mut idx = 0;
            for i in 0..n {
                idx *= (n - i);
                let arri = Util::get_val(arr[i as usize] as u32, isEdge);
                for j in (i+1)..n {
                    let arrj = Util::get_val(arr[j as usize] as u32, isEdge);
                    if (arrj < arri) {
                        idx += 1;
                    }
                }
            }
            return idx;
        }

        fn get_comb(arr : &mut [u8], mask : u32, isEdge : bool) -> u32{
            let end = arr.len() - 1;
            let mut idxC = 0;
            let mut idxP = 0;
            let mut r = 4;
            let mut val = 0x0123;
            for i in (0..(end+1)).rev() {
                let perm = Util::get_val(arr[i] as u32, isEdge);
                if ((perm & 0xc) == mask) {
                    let v = (perm & 3) << 2;
                    idxP = r * idxP + (val >> v & 0xf);
                    val -= 0x0111 >> (12 - v);
                    idxC += c_nk[i][r];
                    r -= 1;
                }
            }
            return ((idxP as u32) << 9) | c_nk[arr.len()][4] - 1 - idxC;
        }

        fn set_comb(arr : &mut [u8], idx: u32, mask : u32, isEdge : bool) {
            let end = arr.len() as u32 - 1;
            let mut r = 4;
            let mut fill = end;
            let mut val = 0x0123;
            let mut idxC = c_nk[arr.len()][4] - 1 - (idx & 0x1ff);
            let mut idxP = idx >> 9;
            for i in (0..(end+1)).rev() {
                if (idxC >= c_nk[i as usize][r as usize]) {
                    idxC -= c_nk[i as usize][r as usize];
                    r -= 1;
                    let p = fact[r];
                    let v = idxP / p << 2;
                    idxP %= p;
                    arr[i as usize] = Util::set_val(arr[i as usize] as u32, val >> v & 3 | mask, isEdge);
                    let m = (1 << v) - 1;
                    val = val & m | val >> 4 & (!m);
                } else {
                    if ((fill & 0xc) == mask) {
                        fill -= 4;
                    }
                    arr[i as usize] = Util::set_val(arr[i as usize] as u32, fill, isEdge);
                    fill -= 1;
                }
            }
        }
    }
}