use std;

#[derive(Debug)]
pub struct ErrorExpectSelectQuery;

impl std::fmt::Display for ErrorExpectSelectQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "expect a SELECT SPARQL query"
        )
    }
}

impl std::error::Error for ErrorExpectSelectQuery {}

#[derive(Debug)]
pub struct ErrorSolutionExpectedToBeBoolean {
    pub variable: &'static str,
}

impl std::fmt::Display for ErrorSolutionExpectedToBeBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the value of the variable '{}' is not a boolean or a string boolean",
            self.variable
        )
    }
}

impl std::error::Error for ErrorSolutionExpectedToBeBoolean {}

#[derive(Debug)]
pub struct ErrorSolutionExpectedToBeString {
    pub variable: &'static str,
}

impl std::fmt::Display for ErrorSolutionExpectedToBeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the value of the variable '{}' is not a literal",
            self.variable
        )
    }
}

impl std::error::Error for ErrorSolutionExpectedToBeString {}

#[derive(Debug)]
pub struct ErrorProjectionVariableDoesNotExist {
    pub variable: String,
}

impl std::fmt::Display for ErrorProjectionVariableDoesNotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the variable '{}' does not exist in the solution map",
            self.variable
        )
    }
}
impl std::error::Error for ErrorProjectionVariableDoesNotExist {}

#[derive(Debug)]
pub struct ErrorNotSelectQuery;

impl std::fmt::Display for ErrorNotSelectQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the type of the query should be SELECT")
    }
}
impl std::error::Error for ErrorNotSelectQuery {}
