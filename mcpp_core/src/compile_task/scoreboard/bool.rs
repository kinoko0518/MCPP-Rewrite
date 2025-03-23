use crate::compile_task::evaluater::EvaluateError;

use super::{Calcable, Scoreboard, Types, NAMESPACE};

pub fn calc(score:&Scoreboard, operator:&str, value:&Calcable) -> Result<String, EvaluateError> {
    match value {
        Calcable::Scr(s) => Ok(
            calc_score(score, operator, s)?,
        ),
        Calcable::Bln(b) => Ok(
            calc_bool(score, operator, *b)?
        ),
        _ => Err(
            EvaluateError::OperationOccuredBetweenUnsupportedTypes(
                score.data_type.clone(),
                value.get_type()
            )
        )
    }
}
fn calc_score(score:&Scoreboard, operator:&str, source:&Scoreboard) -> Result<String, EvaluateError> {
    match source.data_type {
        Types::Bln => {
            match operator {
                "&" => Ok(
                    format!(
                        "execute unless {} unless {} store {} {}",
                        score.pure_compare_value("==", 0)?,
                        score.pure_compare_value("==", 0)?,
                        score.mcname(),
                        NAMESPACE
                    )
                ),
                "|" => Ok(
                    format!(
                        "execute unless {} run {}",
                        score.pure_compare_value("==", 0)?,
                        score.pure_assign_value(1)
                    )
                ),
                _ => Err(EvaluateError::UnknownOperatorGiven(operator.to_string()))
            }
        }
        _ => return Err(
            EvaluateError::OperationOccuredBetweenUnsupportedTypes(
                source.data_type.clone(),
                score.data_type.clone()
            )
        )
    }
}
fn calc_bool(score:&Scoreboard, operator:&str, source:bool) -> Result<String, EvaluateError> {
    match operator {
        "&" => {
            if source {
                // N & 1 = N
                // It have no meaning at all ngl lol
                Ok(
                    format!(
                        "execute unless {} run {}",
                        score.pure_compare_value("==", 0)?,
                        score.pure_assign_value(1)
                    )
                )
            } else {
                // N & 0 = 0
                // It completedly, have no meaning at all
                Ok(
                    format!(
                        "execute unless {} run {}",
                        score.pure_compare_value("==", 1)?,
                        score.pure_assign_value(0)
                    )
                )
            }
        },
        "|" => {
            if source {
                // N | 1 = 1
                Ok(
                    format!("{}", score.pure_assign_value(1))
                )
            } else {
                // N | 0 = N
                Ok(
                    String::new()
                )
            }
        },
        _ => Err(EvaluateError::UnknownOperatorGiven(operator.to_string()))
    }
}
pub fn assign(score:&Scoreboard, value:&Calcable) -> Result<String, EvaluateError> {
    match value {
        Calcable::Bln(b) => Ok(
            if *b {
                score.pure_assign_value(1)
            } else {
                score.pure_assign_value(0)
            }
        ),
        Calcable::Scr(s) => {
            match s.data_type {
                Types::Bln => Ok(score.pure_assign_score(&s)),
                _ => Err(
                    EvaluateError::AssignOccuredBetweenUnsupportedTypes(value.get_type(), score.data_type.clone())
                )
            }
        },
        _ => Err(EvaluateError::AssignOccuredBetweenUnsupportedTypes(value.get_type(), score.data_type.clone()))
    }
}
pub fn compare(score:&Scoreboard, operator:&str, value:&Calcable) -> Result<String, EvaluateError> {
    match operator {
        "==" => match value {
            Calcable::Scr(s) => match s.data_type {
                Types::Bln => Ok(score.pure_compare_score(operator, s)),
                _ => Err(
                    EvaluateError::ComparementOccuredBetweenUnsupportedTypes(score.data_type.clone(), value.get_type())
                )
            },
            Calcable::Bln(b) => if *b {
                score.pure_compare_value(operator, 1)
            } else {
                score.pure_compare_value(operator, 0)
            },
            _ => Err(EvaluateError::ComparementOccuredBetweenUnsupportedTypes(score.data_type.clone(), value.get_type()))
        },
        _ => Err(EvaluateError::UnknownOperatorGiven(operator.to_string()))
    }
}