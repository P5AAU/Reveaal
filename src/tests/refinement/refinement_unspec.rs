#[cfg(test)]
mod test {
    use crate::tests::refinement::helper::refinement_check;

    const PATH: &str = "samples/json/Unspec";

    #[test]
    fn test_arefines_self() {
        assert!(refinement_check(PATH, "refinement: A <= A"));
    }

    #[test]
    fn test_aa_refines_self() {
        assert!(refinement_check(PATH, "refinement: AA <= AA"));
    }

    #[test]
    fn test_brefines_self() {
        assert!(refinement_check(PATH, "refinement: B <= B"));
    }
}
