#[cfg(test)]
mod test {
    use crate::tests::refinement::helper::refinement_check;

    const PATH: &str = "samples/json/DelayAdd";

    #[test]
    fn a1_a2_not_refines_b() {
        assert!(!refinement_check(PATH, "refinement: A1 || A2 <= B"));
    }

    #[test]
    fn c1_not_refines_c2() {
        assert!(!refinement_check(PATH, "refinement: C1 <= C2"));
    }

    #[test]
    fn d1_not_refines_d2() {
        assert!(!refinement_check(PATH, "refinement: D1 <= D2"));
    }
}
