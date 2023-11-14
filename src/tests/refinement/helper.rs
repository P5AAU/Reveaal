use crate::data_reader::parse_queries;
use crate::extract_system_rep::ExecutableQueryError;
use crate::logging::setup_logger;
use crate::model_objects::Query;
use crate::system::extract_system_rep::create_executable_query;
use crate::system::query_failures::QueryResult;
use crate::transition_systems::transition_system::component_loader_to_transition_system;
use crate::transition_systems::TransitionSystemPtr;
use crate::ProjectLoader;

fn try_setup_logging() {
    #[cfg(feature = "logging")]
    let _ = setup_logger();
}

pub fn refinement_check(path: &str, query: &str) -> bool {
    try_setup_logging();

    match run_query(path, query).unwrap() {
        QueryResult::Refinement(Ok(())) => true,
        QueryResult::Refinement(Err(_)) => false,
        QueryResult::CustomError(err) => panic!("{}", err),
        _ => panic!("Not a refinement check"),
    }
}

pub fn run_query(path: &str, query: &str) -> Result<QueryResult, ExecutableQueryError> {
    let mut project_loader = ProjectLoader::new(String::from(path), crate::tests::TEST_SETTINGS);
    let query = parse_queries::parse_to_expression_tree(query)
        .unwrap()
        .remove(0);
    let q = Query {
        query: Option::from(query),
        comment: "".to_string(),
    };

    let query = create_executable_query(&q, &mut project_loader)?;

    Ok(query.execute())
}

pub fn get_system(path: &str, comp: &str) -> TransitionSystemPtr {
    let mut project_loader = ProjectLoader::new(String::from(path), crate::tests::TEST_SETTINGS);
    component_loader_to_transition_system(&mut project_loader, comp)
}
