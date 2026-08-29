use gritshield::GritComponent;

#[derive(Clone, GritComponent)]
pub struct WorkflowEngine {}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self{}
    }

    /// Validates whether an issue can move between workflow steps
    pub fn can_transition(&self, current_step_id: i32, target_step_id: i32) -> bool {
        // Enforce logical transitions (e.g. step jumps or rules)
        (target_step_id - current_step_id).abs() <= 2
    }
}