// smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
// to describe demons, their fusion, and fusion mechanics for the game 
// Shin Megami Tensei III: Nocturne.
// Copyright (C) 2025  Bryan-Elliott Tam
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
