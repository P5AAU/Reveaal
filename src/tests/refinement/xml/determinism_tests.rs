#[cfg(test)]
mod test {
    use crate::{system::query_failures::QueryResult, tests::refinement::helper::run_query};

    const PATH: &str = "samples/xml/ConsTests.xml";

    #[track_caller]
    fn test_determinism(query: &str, expected_deterministic: bool) {
        let result = run_query(PATH, query);

        if let QueryResult::Determinism(is_deterministic) = result.unwrap() {
            assert!(matches!(is_deterministic, Ok(())) == expected_deterministic);
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn test_g1() {
        test_determinism("determinism: G1", true);
    }
    #[test]
    fn test_g2() {
        test_determinism("determinism: G2", true);
    }

    #[test]
    fn test_g3() {
        test_determinism("determinism: G3", true);
    }
    #[test]
    fn test_g4() {
        test_determinism("determinism: G4", true);
    }

    #[test]
    fn test_g5() {
        test_determinism("determinism: G5", true);
    }
    #[test]
    fn test_g6() {
        test_determinism("determinism: G6", true);
    }

    #[test]
    fn test_g7() {
        test_determinism("determinism: G7", true);
    }

    #[test]
    fn test_g8() {
        test_determinism("determinism: G8", true);
    }

    #[test]
    fn test_g9() {
        test_determinism("determinism: G9", false);
    }

    #[test]
    fn test_g10() {
        test_determinism("determinism: G10", true);
    }

    #[test]
    fn test_g11() {
        test_determinism("determinism: G11", true);
    }

    #[test]
    fn test_g12() {
        test_determinism("determinism: G12", true);
    }

    #[test]
    fn test_g13() {
        test_determinism("determinism: G13", true);
    }

    #[test]
    fn test_g14() {
        test_determinism("determinism: G14", false);
    }

    #[test]
    fn test_g15() {
        test_determinism("determinism: G15", true);
    }

    #[test]
    fn test_g16() {
        test_determinism("determinism: G16", false);
    }

    #[test]
    fn test_g17() {
        test_determinism("determinism: G17", true);
    }

    #[test]
    fn test_g22() {
        test_determinism("determinism: G22", true);
    }

    #[test]
    fn test_g23() {
        test_determinism("determinism: G23", false);
    }
}
