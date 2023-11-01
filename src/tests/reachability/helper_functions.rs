pub mod reachability_test_helper_functions {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::extract_system_rep::get_system_recipe;
    use crate::extract_system_rep::SystemRecipe;
    use crate::model_objects::expressions::StateExpression;
    use crate::model_objects::expressions::SystemExpression;
    use crate::parse_queries::parse_to_state_expr;
    use crate::transition_systems::TransitionSystem;
    use crate::ProjectLoader;

    /// Helper function which converts a string to an option<box<BoolExpression>> by replacing ',' with "&&" and using the invariant parser.
    pub fn string_to_state_expr(string: &str) -> StateExpression {
        parse_to_state_expr(string).unwrap()
    }

    /// Helper function to create a transition system and a machine (system recipe)
    pub fn create_system_recipe_and_machine(
        model: SystemExpression,
        folder_path: &str,
    ) -> (Box<SystemRecipe>, Box<dyn TransitionSystem>) {
        let comp_loader = Arc::new(Mutex::new(ProjectLoader::new(
            folder_path,
            crate::tests::TEST_SETTINGS,
        )));

        let dim = AtomicUsize::new(0);
        let quotient_index = Arc::new(Mutex::new(None));
        let machine = get_system_recipe(&model, comp_loader, &dim, quotient_index);
        let dim_before = dim.load(Ordering::SeqCst);
        // TODO: unwrap might not be the best way to handle this
        let system = machine.clone().compile(dim_before).unwrap();
        (machine, system)
    }
}
