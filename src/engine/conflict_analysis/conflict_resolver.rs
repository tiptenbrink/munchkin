use super::ConflictAnalysisContext;

pub(crate) trait ConflictResolver {
    fn resolve_conflict(&mut self, context: &mut ConflictAnalysisContext);

    fn process(&mut self, context: &mut ConflictAnalysisContext) -> Result<(), ()>;
}
