#[cfg(test)]

mod test {
    use std::collections::HashSet;

    use crate::{
        tests::refinement::Helper::json_run_query,
        System::query_failures::{ActionFailure, QueryResult, SystemRecipeFailure},
    };

    const PATH: &str = "samples/json/SystemRecipe/Conjunction";

    #[test]
    fn conjunction1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction1 && RightConjunction1");
        assert!(matches!(
            actual,
            QueryResult::RecipeFailure(SystemRecipeFailure::Action(
                ActionFailure::NotDisjoint(_, _),
                _
            ))
        ));
    }

    #[test]
    fn conjunction1_fails_with_correct_actions() {
        let expected_actions = HashSet::from(["Input1".to_string()]); // Assuming inputs are checked first
        if let QueryResult::RecipeFailure(SystemRecipeFailure::Action(
            ActionFailure::NotDisjoint(left, right),
            _,
        )) = json_run_query(PATH, "consistency: LeftConjunction1 && RightConjunction1")
        {
            assert_eq!(
                left.actions
                    .intersection(&right.actions)
                    .cloned()
                    .collect::<HashSet<_>>(),
                expected_actions
            );
        } else {
            panic!("Models in samples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn conjunction2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction2 && RightConjunction2");
        assert!(matches!(
            actual,
            QueryResult::RecipeFailure(SystemRecipeFailure::Action(
                ActionFailure::NotDisjoint(_, _),
                _
            ))
        ));
    }

    #[test]
    fn conjunction2_fails_with_correct_actions() {
        let expected_actions = HashSet::from(["Input1".to_string()]);
        if let QueryResult::RecipeFailure(SystemRecipeFailure::Action(
            ActionFailure::NotDisjoint(left, right),
            _,
        )) = json_run_query(PATH, "consistency: LeftConjunction2 && RightConjunction2")
        {
            assert_eq!(
                left.actions
                    .intersection(&right.actions)
                    .cloned()
                    .collect::<HashSet<_>>(),
                expected_actions
            );
        } else {
            panic!("Models in samples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn conjunction3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction3 && RightConjunction3");
        assert!(matches!(
            actual,
            QueryResult::RecipeFailure(SystemRecipeFailure::Action(
                ActionFailure::NotDisjoint(_, _),
                _
            ))
        ));
    }

    #[test]
    fn conjunction3_fails_with_correct_actions() {
        let expected_actions = HashSet::from(["Output1".to_string()]);
        if let QueryResult::RecipeFailure(SystemRecipeFailure::Action(
            ActionFailure::NotDisjoint(left, right),
            _,
        )) = json_run_query(PATH, "consistency: LeftConjunction3 && RightConjunction3")
        {
            assert_eq!(
                left.actions
                    .intersection(&right.actions)
                    .cloned()
                    .collect::<HashSet<_>>(),
                expected_actions
            );
        } else {
            panic!("Models in samples/action have been changed, REVERT!");
        }
    }
}
