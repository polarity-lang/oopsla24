use std::rc::Rc;

use crate::de_bruijn::*;

use super::def::*;

pub fn occurs_in<P: Phase, L: Leveled>(base_lvl: &L, the_idx: Idx, in_exp: &Rc<Exp<P>>) -> bool {
    let target_lvl = base_lvl.idx_to_lvl(the_idx);
    occurs(base_lvl, target_lvl, in_exp)
}

fn occurs<P: Phase, L: Leveled>(curr_lvl: &L, target_lvl: Lvl, in_exp: &Rc<Exp<P>>) -> bool {
    match &**in_exp {
        Exp::Var { idx, .. } => curr_lvl.idx_to_lvl(*idx) == target_lvl,
        Exp::TypCtor { args, .. } => args.iter().any(|arg| occurs(curr_lvl, target_lvl, arg)),
        Exp::Ctor { args, .. } => args.iter().any(|arg| occurs(curr_lvl, target_lvl, arg)),
        Exp::Dtor { exp, args, .. } => {
            occurs(curr_lvl, target_lvl, exp)
                || args.iter().any(|arg| occurs(curr_lvl, target_lvl, arg))
        }
        Exp::Anno { exp, typ, .. } => {
            occurs(curr_lvl, target_lvl, exp) || occurs(curr_lvl, target_lvl, typ)
        }
        Exp::Type { .. } => false,
    }
}