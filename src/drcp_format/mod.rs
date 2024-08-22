//! This crate contains abstractions for dealing with the Deletion Reverse Constraint Propagation
//! (DRCP) proof format. The format can be used by Constraint Programming solvers to provide a
//! certifyable proof of unsatisfiability or optimality.
//!
//! To write DRCP proof files, look at the [`ProofWriter`] documentation. Literal definition files
//! can be written using [`LiteralDefinitions`].
mod atomic;
mod encountered_literals;
mod format;
mod literal_code_provider;
mod proof_literals;
mod steps;
mod writer;

use std::{
    fs::File,
    num::NonZeroU64,
    path::{Path, PathBuf},
};

pub use atomic::*;
pub use encountered_literals::*;
pub use format::*;
pub use literal_code_provider::*;
pub(crate) use proof_literals::*;
pub use steps::*;
pub use writer::*;

use crate::{engine::VariableLiteralMappings, variable_names::VariableNames, variables::Literal};

/// A proof log which logs the proof steps necessary to prove unsatisfiability or optimality. We
/// allow the following types of proofs:
/// - A CP proof log - This can be created using [`ProofLog::cp`].
///
/// When a proof log should not be generated, use the implementation of [`Default`].
#[derive(Debug, Default)]
pub struct ProofLog {
    internal_proof: Option<ProofImpl>,
}

impl ProofLog {
    /// Create a CP proof logger.
    pub fn cp(file_path: &Path, format: Format) -> std::io::Result<ProofLog> {
        let definitions_path = file_path.with_extension("lits");
        let file = File::create(file_path)?;

        let writer = ProofWriter::new(format, file, ProofLiterals::default());

        Ok(ProofLog {
            internal_proof: Some(ProofImpl::CpProof {
                writer,
                definitions_path,
            }),
        })
    }

    /// Log a learned clause to the proof.
    pub(crate) fn log_learned_clause(
        &mut self,
        literals: impl IntoIterator<Item = Literal>,
    ) -> std::io::Result<NonZeroU64> {
        // Used as a proof clause ID when no proof log is used. This should ideally be a `const`,
        // but `Option::<T>::unwrap()` is not yet stable in const context.
        let default_clause_id: NonZeroU64 = NonZeroU64::new(1).unwrap();

        match &mut self.internal_proof {
            Some(ProofImpl::CpProof { writer, .. }) => writer.log_nogood_clause(literals),
            None => Ok(default_clause_id),
        }
    }

    pub(crate) fn unsat(
        self,
        variable_names: &VariableNames,
        variable_literal_mapping: &VariableLiteralMappings,
    ) -> std::io::Result<()> {
        match self.internal_proof {
            Some(ProofImpl::CpProof {
                writer,
                definitions_path,
            }) => {
                let literals = writer.unsat()?;
                let file = File::create(definitions_path)?;
                literals.write(file, variable_names, variable_literal_mapping)
            }
            None => Ok(()),
        }
    }

    pub(crate) fn optimal(
        self,
        objective_bound: Literal,
        variable_names: &VariableNames,
        variable_literal_mapping: &VariableLiteralMappings,
    ) -> std::io::Result<()> {
        match self.internal_proof {
            Some(ProofImpl::CpProof {
                writer,
                definitions_path,
            }) => {
                let literals = writer.optimal(objective_bound)?;
                let file = File::create(definitions_path)?;
                literals.write(file, variable_names, variable_literal_mapping)
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug)]
enum ProofImpl {
    CpProof {
        writer: ProofWriter<File, ProofLiterals>,
        definitions_path: PathBuf,
    },
}
