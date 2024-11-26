use crate::branching::Brancher;
use crate::branching::SelectionContext;
use crate::predicates::Predicate;

struct ConcreteBrancher {
    // TODO
}

impl Brancher for ConcreteBrancher {
    fn next_decision(&mut self, context: &mut SelectionContext) -> Option<Predicate> {
        todo!()
    }
}
