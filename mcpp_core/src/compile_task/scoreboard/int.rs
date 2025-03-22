use crate::compile_task::{
    evaluater::EvaluateError,
    scoreboard::{
        float::{self, get_magnif}, Calcable, Scoreboard, Types
    }
};

use super::get_temp_score;

pub fn calc(score:&Scoreboard, operator:&str, target:&Calcable) -> Result<String, EvaluateError> {
    match target {
        Calcable::Int(i) => calc_num(score, operator, *i),
        Calcable::Flt(f) => calc_float(score, operator, *f),
        Calcable::Scr(s) => calc_score(score, operator, s),
        Calcable::Mcf(f) => calc(score, operator, &Calcable::Scr(&f.ret_container)),
    }
}
/// The pure function to get corresponding scoreboard command.
fn calc_num(score:&Scoreboard, operator:&str, num:i32) -> Result<String, EvaluateError> {
    score.pure_calc_num(operator, num)
}
fn calc_float(score:&Scoreboard, operator:&str, source:f32) -> Result<String, EvaluateError> {
    score.pure_calc_num(operator, float::scale_float(source))
}
fn calc_score(score:&Scoreboard, operator:&str, source:&Scoreboard) -> Result<String, EvaluateError> {
    match source.data_type {
        Types::Int => Ok(
            score.pure_calc_score(operator, source)
        ),
        Types::Flt => {
            let temp_score = get_temp_score();
            Ok(
                format!(
                    "{}\n{}",
                    temp_score.assign(&Calcable::Scr(source))?,
                    calc_score(score, operator, &temp_score)?
                )
            )
        },
        _ => Err(EvaluateError::OperationOccuredBetweenUnsupportedTypes)
    }
}
pub fn assign(scoreboard:&Scoreboard, value:&Calcable) -> Result<String, EvaluateError> {
    match value {
        &Calcable::Int(i) => Ok(
            scoreboard.pure_assign_value(i)
        ),
        &Calcable::Scr(s) => {
            match s.data_type {
                Types::Int => Ok(
                    scoreboard.pure_assign_score(s)
                ),
                Types::Flt => {
                    let temp_score = get_temp_score();
                    Ok(
                        format!(
                            "{}\n{}\n{}",
                            temp_score.assign(&Calcable::Scr(s))?,
                            temp_score.calc("/", &Calcable::Int(get_magnif()))?,
                            scoreboard.assign(&Calcable::Scr(&temp_score))?
                        )
                    )
                },
                Types::Non => Err(
                    EvaluateError::AssignOccuredBetweenUnsupportedTypes
                )
            }
        }
        &Calcable::Flt(f) => scoreboard.assign(
            &Calcable::Int(float::scale_float(f))
        ),
        &Calcable::Mcf(f) => scoreboard.assign(
            &Calcable::Scr(&f.ret_container)
        )
    }
}