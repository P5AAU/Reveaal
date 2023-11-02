#[cfg(test)]
mod test {
    use crate::{system::query_failures::QueryResult, tests::refinement::helper::run_query};

    const PATH: &str = "samples/xml/ConsTests.xml";

    #[track_caller]
    fn test_consistency(query: &str, expected: bool) {
        let result = run_query(PATH, query);
        if let QueryResult::Consistency(_) = result.as_ref().unwrap() {
            assert!(expected == matches!(result, Ok(QueryResult::Consistency(Ok(())))));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn test_g1() {
        test_consistency("consistency: G1", true);
    }
    #[test]
    fn test_g2() {
        test_consistency("consistency: G2", true);
    }

    #[test]
    fn test_g3() {
        test_consistency("consistency: G3", false);
    }

    #[test]
    fn test_g4() {
        test_consistency("consistency: G4", false);
    }

    #[test]
    fn test_g5() {
        test_consistency("consistency: G5", false);
    }

    #[test]
    fn test_g6() {
        test_consistency("consistency: G6", true);
    }

    #[test]
    fn test_g7() {
        test_consistency("consistency: G7", false);
    }

    #[test]
    fn test_g8() {
        test_consistency("consistency: G8", true);
    }

    #[test]
    fn test_g9() {
        test_consistency("consistency: G9", false);
    }

    #[test]
    fn test_g10() {
        test_consistency("consistency: G10", false);
    }

    #[test]
    fn test_g11() {
        test_consistency("consistency: G11", false);
    }

    #[test]
    fn test_g12() {
        test_consistency("consistency: G12", false);
    }

    #[test]
    fn test_g13() {
        test_consistency("consistency: G13", true);
    }

    #[test]
    fn test_g14() {
        test_consistency("consistency: G14", false);
    }

    #[test]
    fn test_g15() {
        test_consistency("consistency: G15", true);
    }

    #[test]
    fn test_g16() {
        test_consistency("consistency: G16", false);
    }

    #[test]
    fn test_g17() {
        test_consistency("consistency: G17", true);
    }

    #[test]
    fn test_g18() {
        test_consistency("consistency: G18", true);
    }

    #[test]
    fn test_g19() {
        test_consistency("consistency: G19", false);
    }

    #[test]
    fn test_g20() {
        test_consistency("consistency: G20", true);
    }

    #[test]
    fn test_g21() {
        test_consistency("consistency: G21", true);
    }
}
