use cubelib::cube::{Cube, Move, Manuever, Face};

enum IDAResult {
    Solution(Manuever),
    NewBound(u8)
}

fn ida_star_h(c : Cube) -> u8 {
    return 0;
}

fn ida_star_search<F: FnMut(Cube) -> bool>(c : Cube, g : u8, b : u8, ms : &Vec<Move>, mut is_goal : F) -> IDAResult {
    let f = g + ida_star_h(c);
    if f > b {
        return IDAResult::NewBound(f);
    }
    if is_goal(c) {
        return IDAResult::Solution(Manuever::new())
    }
    
    let mut min : u8 = 255;
    for m in ms.iter_mut() {
        match ida_star_search(c*m, g+1, b, ms, is_goal) {
            IDAResult::Solution(mut mn) => {
                mn += m;
                return IDAResult::Solution(mn);
            },
            IDAResult::NewBound(nb) => {
                if nb < min {
                    min = nb;
                }
            }
        }
    }
    return IDAResult::NewBound(min);
}

pub fn ida_star(c : Cube, ms : &Vec<Move>) -> Option<Manuever> {
    let mut bound = ida_star_h(c);
    let mut lbound = bound;
    loop {
        match ida_star_search(c, 0, bound, ms, |x| true) {
            IDAResult::Solution(mn) => {
                return Some(mn)
            },
            IDAResult::NewBound(nb) => {
                if nb > bound {
                    bound = nb;
                }
            }
        }
        if bound == lbound {
            return None;
        }
        lbound = bound;
    }
}