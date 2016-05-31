//moves
pub const Ux1 : u8 = 0;
pub const Ux2 : u8 = 1;
pub const Ux3 : u8 = 2;
pub const Rx1 : u8 = 3;
pub const Rx2 : u8 = 4;
pub const Rx3 : u8 = 5;
pub const Fx1 : u8 = 6;
pub const Fx2 : u8 = 7;
pub const Fx3 : u8 = 8;
pub const Dx1 : u8 = 9;
pub const Dx2 : u8 = 10;
pub const Dx3 : u8 = 11;
pub const Lx1 : u8 = 12;
pub const Lx2 : u8 = 13;
pub const Lx3 : u8 = 14;
pub const Bx1 : u8 = 15;
pub const Bx2 : u8 = 16;
pub const Bx3 : u8 = 17;

//facelets
pub const U1 : u8 = 0;
pub const U2 : u8 = 1;
pub const U3 : u8 = 2;
pub const U4 : u8 = 3;
pub const U5 : u8 = 4;
pub const U6 : u8 = 5;
pub const U7 : u8 = 6;
pub const U8 : u8 = 7;
pub const U9 : u8 = 8;
pub const R1 : u8 = 9;
pub const R2 : u8 = 10;
pub const R3 : u8 = 11;
pub const R4 : u8 = 12;
pub const R5 : u8 = 13;
pub const R6 : u8 = 14;
pub const R7 : u8 = 15;
pub const R8 : u8 = 16;
pub const R9 : u8 = 17;
pub const F1 : u8 = 18;
pub const F2 : u8 = 19;
pub const F3 : u8 = 20;
pub const F4 : u8 = 21;
pub const F5 : u8 = 22;
pub const F6 : u8 = 23;
pub const F7 : u8 = 24;
pub const F8 : u8 = 25;
pub const F9 : u8 = 26;
pub const D1 : u8 = 27;
pub const D2 : u8 = 28;
pub const D3 : u8 = 29;
pub const D4 : u8 = 30;
pub const D5 : u8 = 31;
pub const D6 : u8 = 32;
pub const D7 : u8 = 33;
pub const D8 : u8 = 34;
pub const D9 : u8 = 35;
pub const L1 : u8 = 36;
pub const L2 : u8 = 37;
pub const L3 : u8 = 38;
pub const L4 : u8 = 39;
pub const L5 : u8 = 40;
pub const L6 : u8 = 41;
pub const L7 : u8 = 42;
pub const L8 : u8 = 43;
pub const L9 : u8 = 44;
pub const B1 : u8 = 45;
pub const B2 : u8 = 46;
pub const B3 : u8 = 47;
pub const B4 : u8 = 48;
pub const B5 : u8 = 49;
pub const B6 : u8 = 50;
pub const B7 : u8 = 51;
pub const B8 : u8 = 52;
pub const B9 : u8 = 53;

//sides
pub const U : u8 = 0;
pub const R : u8 = 0;
pub const F : u8 = 0;
pub const D : u8 = 0;
pub const L : u8 = 0;
pub const B : u8 = 0;

const fact : [i32; 14]  = [
    1,1,2,6,24,120,720,5040,40320,362880,3628800,39916800,479001600,6227020800
];

const c_nk : [[i32; 13]; 13] = [
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

pub struct Util {
    corner_facelet : [[u8;3];8],
    edge_facelet : [[u8; 2]; 12],
    perm_mult : [[i32;24]; 24],
    pre_move : [i8; 9],
    ud2std : [i8; 10],
    std2ud : [i8; 18],
    ckmv2 : [[bool;10];11],
}

impl Util {
    pub fn new() -> Util {
        let mut perm_mult = [[0i32;24];24];
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
        let mut ckmv2 = [[false;10];11];
        
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
        
        let mut arr1 = [0u8; 4];
        let mut arr2 = [0u8; 4];
        let mut arr3 = [0u8; 4];
        for i in 0..24 {
            set_n_perm(&mut arr1, i, 4, false);
            for j in 0..24 {
                set_n_perm(&mut arr2, j, 4, false);
                for k in 0..4 {
                    arr3[k] = arr1[arr2[k] as usize];
                }
                perm_mult[i as usize][j as usize] = get_n_perm(&mut arr3, 4, false);
            }
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
}
    /////////////////////////////////////////////////////////////////////////////////////////////////
    
pub fn get_n_parity(mut idx : i32, n : i32) -> i32 {
    let mut p = 0;
    for i in (0..(n-1)).rev() {
        p ^= idx % (n - i);
        idx /= (n - i);
    }
    return p & 1;
}

pub fn set_val(val0 : i32, val : i32, isEdge : bool) -> u8 {
    (if isEdge { val << 1 | val0 & 1 } else { val | val0 & 0xf8 }) as u8
}

pub fn get_val(val0 : i32, isEdge : bool) -> i32 {
    if isEdge { val0 >> 1 } else { val0 & 7 }
}

pub fn set_8_perm(arr : &mut [u8], mut idx : i32, isEdge : bool) {
    let mut val = 0x76543210;
    for i in 0..7 {
        let p = fact[7 - i];
        let mut v = idx / p;
        idx -= v * p;
        v <<= 2;
        arr[i] = set_val(arr[i] as i32, (val >> v & 0x7), isEdge);
        let m = (1 << v) - 1;
        val = val & m | val >> 4 & (!m);
    }
    arr[7] = set_val(arr[7] as i32, val, isEdge);
}

pub fn get_8_perm(arr : &mut [u8], isEdge : bool) -> i32 {
    let mut idx = 0;
    let mut val = 0x76543210;
    for i in 0..7 {
        let v = get_val(arr[i] as i32, isEdge) << 2;
        idx = (8 - i) * idx + (val >> v & 0x7);
        val -= 0x11111110 << v;
    }
    return idx as i32;
}

pub fn set_n_perm(arr : &mut [u8], mut idx : i32, n : i32, isEdge : bool) {
    arr[(n - 1) as usize] = set_val(arr[(n - 1) as usize] as i32, 0, isEdge);
    for i in (0..(n-1)).rev() {
        let arri = idx % (n - i);
        arr[i as usize] = set_val(arr[i as usize] as i32, arri, isEdge);
        idx /= (n - i);
        for j in (i+1)..n {
            let mut arrj = get_val(arr[j as usize] as i32, isEdge);
            if (arrj >= arri) {
                arrj += 1;
                arr[j as usize] = set_val(arr[j as usize] as i32, arrj, isEdge);
            }
        }
    }
}

pub fn get_n_perm(arr : &mut [u8], n : i32, isEdge : bool) -> i32{
    let mut idx = 0;
    for i in 0..n {
        idx *= (n - i);
        let arri = get_val(arr[i as usize] as i32, isEdge);
        for j in (i+1)..n {
            let arrj = get_val(arr[j as usize] as i32, isEdge);
            if (arrj < arri) {
                idx += 1;
            }
        }
    }
    return idx;
}

pub fn get_comb(arr : &mut [u8], mask : i32, isEdge : bool) -> i32{
    let end = arr.len() - 1;
    let mut idxC = 0;
    let mut idxP = 0;
    let mut r = 4;
    let mut val = 0x0123;
    for i in (0..(end+1)).rev() {
        let perm = get_val(arr[i] as i32, isEdge);
        if ((perm & 0xc) == mask) {
            let v = (perm & 3) << 2;
            idxP = r * idxP + (val >> v & 0xf);
            val -= 0x0111 >> (12 - v);
            idxC += c_nk[i][r];
            r -= 1;
        }
    }
    return ((idxP as i32) << 9) | c_nk[arr.len()][4] - 1 - idxC;
}

pub fn set_comb(arr : &mut [u8], idx: i32, mask : i32, isEdge : bool) {
    let end = arr.len() as i32 - 1;
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
            arr[i as usize] = set_val(arr[i as usize] as i32, val >> v & 3 | mask, isEdge);
            let m = (1 << v) - 1;
            val = val & m | val >> 4 & (!m);
        } else {
            if ((fill & 0xc) == mask) {
                fill -= 4;
            }
            arr[i as usize] = set_val(arr[i as usize] as i32, fill, isEdge);
            fill -= 1;
        }
    }
}