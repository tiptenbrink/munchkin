use crate::engine::variables::Literal;
use crate::pumpkin_assert_moderate;
use crate::pumpkin_assert_simple;

#[allow(clippy::len_without_is_empty)] // The clause will always have at least two literals.
#[derive(Debug)]
pub(crate) struct Clause {
    literals: Vec<Literal>,
    is_learned: bool,
    is_deleted: bool,
    is_protected_aganst_deletion: bool,
    lbd: u32,
    activity: f32,
}

impl Clause {
    pub(crate) fn new(literals: Vec<Literal>, is_learned: bool) -> Clause {
        pumpkin_assert_simple!(literals.len() >= 2);

        let num_literals = literals.len() as u32;
        Clause {
            literals,
            is_learned,
            is_deleted: false,
            is_protected_aganst_deletion: false,
            lbd: num_literals, // pessimistic lbd
            activity: 0.0,
        }
    }
}

impl Clause {
    pub(crate) fn len(&self) -> u32 {
        self.literals.len() as u32
    }

    pub(crate) fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    pub(crate) fn get_literal_slice(&self) -> &[Literal] {
        &self.literals
    }

    // note that this does _not_ delete the clause, it simply marks it as if it was deleted
    //  to delete a clause, use the ClauseManager
    //  could restrict access of this method in the future
    pub(crate) fn mark_deleted(&mut self) {
        pumpkin_assert_moderate!(!self.is_deleted);
        self.is_deleted = true;
    }
}

impl std::ops::Index<u32> for Clause {
    type Output = Literal;
    fn index(&self, index: u32) -> &Literal {
        self.literals.index(index as usize)
    }
}

impl std::ops::IndexMut<u32> for Clause {
    fn index_mut(&mut self, index: u32) -> &mut Literal {
        self.literals.index_mut(index as usize)
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let clause_string = &self
            .literals
            .iter()
            .fold(String::new(), |acc, lit| format!("{acc}{lit},"));

        write!(
            f,
            "({clause_string})[learned:{}, deleted:{}]",
            self.is_learned, self.is_deleted
        )
    }
}
