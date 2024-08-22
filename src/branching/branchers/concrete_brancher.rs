use crate::{
    branching::{Brancher, SelectionContext},
    predicates::Predicate,
};

struct ConcreteBrancher {
    // TODO
}

impl Brancher for ConcreteBrancher {
    fn next_decision(&mut self, context: &mut SelectionContext) -> Option<Predicate> {
        todo!()
    }
}
