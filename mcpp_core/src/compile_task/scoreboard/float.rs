use crate::compile_task::{
    evaluater::EvaluateError,
    scoreboard::{
        float, Calcable, Scoreboard, Types
    }
};
use super::get_temp_score;
pub const ACCURATION:u32 = 3;

pub fn get_magnif() -> i32 {
    10_i32.pow(ACCURATION)
}
pub fn scale_float(input:f32) -> i32 {
    (input * (10_i32.pow(float::ACCURATION) as f32)) as i32
}

pub fn calc(score:&Scoreboard, operator:&str, target:&Calcable) -> Result<String, EvaluateError> {
    match target {
        Calcable::Int(i) => calc_num(score, operator, *i),
        Calcable::Flt(f) => calc_float(score, operator, *f),
        Calcable::Scr(s) => calc_score(score, operator, s),
        Calcable::Mcf(f) => calc(score, operator, &Calcable::Scr(&f.ret_container)),
    }
}
fn calc_num(score:&Scoreboard, operator:&str, num:i32) -> Result<String, EvaluateError> {
    match operator {
        "+" | "-" => score.pure_calc_num(operator, num * get_magnif()),
        _ => score.pure_calc_num(operator, num)
    }
}
fn calc_float(score:&Scoreboard, operator:&str, num:f32) -> Result<String, EvaluateError> {
    match operator {
        "+" | "-" => score.pure_calc_num(operator, scale_float(num)),
        "*" => Ok(
            format!(
                "{}\n{}",
                score.pure_calc_num(operator, scale_float(num))?,
                score.pure_calc_num("/", get_magnif().pow(2))?
            )
        ),
        "/" => Ok(
            format!(
                "{}\n{}",
                score.pure_calc_num("*", get_magnif())?,
                score.pure_calc_num("/", scale_float(num))?
            )
        ),
        _ => Err(EvaluateError::OperationOccuredBetweenUnsupportedTypes(score.data_type.clone(), Types::Flt))
    }
}
fn calc_score(score:&Scoreboard, operator:&str, source:&Scoreboard) -> Result<String, EvaluateError> {
    match source.data_type {
        Types::Int => {
            let temp_score = super::get_temp_score();
            Ok(
                format!(
                    "{}\n{}\n{}",
                    temp_score.assign(&Calcable::Scr(source))?,
                    temp_score.calc("*", &Calcable::Int(get_magnif()))?,
                    score.pure_calc_score(operator, &temp_score)
                )
            )
        },
        Types::Flt => Ok(
            source.pure_calc_score(operator, source)
        ),
        _ => Err(
            EvaluateError::OperationOccuredBetweenUnsupportedTypes(score.data_type.clone(), source.data_type.clone())
        )
    }
}
pub fn assign(score:&Scoreboard, value:&Calcable) -> Result<String, EvaluateError> {
    match value {
        Calcable::Int(i) => Ok(
            score.pure_assign_value(i * get_magnif())
        ),
        Calcable::Flt(f) => Ok(
            score.pure_assign_value(scale_float(*f))
        ),
        Calcable::Scr(s) => match s.data_type {
            Types::Int => {
                let temp_score = get_temp_score();
                Ok(
                    format!(
                        "{}\n{}\n{}",
                        temp_score.assign(&Calcable::Scr(s))?,
                        temp_score.calc("*", &Calcable::Int(get_magnif()))?,
                        score.pure_assign_score(&temp_score)
                    )
                )
            },
            Types::Flt => Ok(score.pure_assign_score(s)),
            _ => Err(EvaluateError::AssignOccuredBetweenUnsupportedTypes(Types::Flt, value.get_type()))
        },
        Calcable::Mcf(f) => assign(
            score,
            &Calcable::Scr(&f.ret_container)
        )
    }
}