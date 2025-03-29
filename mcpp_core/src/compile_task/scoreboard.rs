mod int;
mod float;
mod bool;

use std::{fmt, vec};

use super::{evaluater::EvaluateError, MCFunction};

const NAMESPACE:&str = "MCPP.var";

#[test]
fn compare_test() {
    let hoge = Scoreboard {
        name : "Hoge".to_string(),
        data_type : Types::Flt,
        scope : vec!["TEST".to_string()]
    };
    println!("{:?}", hoge.compare(">=", &Calcable::Int(810)).unwrap());
}

#[derive(Debug, Clone)]
pub struct 
Scoreboard {
    pub name  : String,
    pub data_type : Types,
    pub scope : Vec<String>
}
pub enum Calcable<'a> {
    Int(i32),
    Flt(f32),
    Bln(bool),
    Scr(&'a Scoreboard),
    Mcf(&'a MCFunction)
}
impl Calcable<'_> {
    pub fn get_type(&self) -> Types {
        match self {
            Self::Int(_) => Types::Int,
            Self::Flt(_) => Types::Flt,
            Self::Bln(_) => Types::Bln,
            Self::Scr(s) => s.data_type.clone(),
            Self::Mcf(f) => f.ret_container.data_type.clone()
        }
    }
}
#[derive(Debug, Clone)]
pub enum Types {
    Int, // Int
    Flt, // Float
    Bln, // Boolean
    Non, // None
}
impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}", match self {
                Self::Int => "int",
                Self::Flt => "float",
                Self::Bln => "bool",
                Self::Non => "none"
            }
        )
    }
}
impl fmt::Display for Calcable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Calcable::Int(i) => i.to_string(),
            Calcable::Scr(s) => s.to_string(),
            Calcable::Flt(f) => f.to_string(),
            Calcable::Bln(b) => b.to_string(),
            Calcable::Mcf(f) => format!("{}(...)", f.name)
        })
    }
}
impl From<i32> for Scoreboard {
    fn from(value: i32) -> Self {
        return Scoreboard {
            name  : value.to_string(),
            data_type : Types::Int,
            scope : vec!["CONSTANT".to_string()]
        };
    }
}

impl PartialEq for Scoreboard {
    fn eq(&self, other: &Self) -> bool {
        self.mcname() == other.mcname()
    }
}

pub fn get_temp_score() -> Scoreboard {
    Scoreboard {
        name : "TEMP".to_string(),
        data_type : Types::Int,
        scope : vec!["Calc".to_string()]
    }
}

impl Scoreboard {
    /// The pure fuction to get a name of corresponding scoreboard in Minecraft.
    pub fn mcname(&self) -> String {
        let mut last = self.scope.to_vec();
        last.push(self.name.to_string());
        return format!("{}{}", "#", last.join("."));
    }
    pub fn calc(&self, operator:&str, source:&Calcable) -> Result<String, EvaluateError> {
        match self.data_type {
            Types::Int => int::calc(&self, operator, source),
            Types::Flt => float::calc(&self, operator, source),
            Types::Bln => bool::calc(self, operator, source),
            Types::Non => Err(EvaluateError::OperationOccuredBetweenUnsupportedTypes(Types::Non, source.get_type()))
        }
    }
    pub fn assign(&self, source:&Calcable) -> Result<String, EvaluateError> {
        match self.data_type {
            Types::Int => int::assign(&self, source),
            Types::Flt => float::assign(&self, source),
            Types::Bln => bool::assign(&self, source),
            Types::Non => Err(EvaluateError::AssignOccuredBetweenUnsupportedTypes(source.get_type(), Types::Non))
        }
    }
    pub fn compare(&self, operator:&str, source:&Calcable) -> Result<(String, String), EvaluateError> {
        let mut do_invert = false;
        let _oper = if operator == "!=" {
            do_invert = true;
            "=="
        } else {
            operator
        };
        let result = match self.data_type {
            Types::Int => int::compare(self, _oper, source),
            Types::Flt => float::compare(self, _oper, source),
            Types::Bln => Ok((String::new(), bool::compare(&self, _oper, source)?)),
            _ => Err(EvaluateError::ComparementOccuredBetweenUnsupportedTypes(self.data_type.clone(), source.get_type()))
        }?;
        Ok(
            (
                result.0,
                format!(
                    "{} {}",
                    if do_invert {"unless"} else {"if"},
                    result.1
                )
            )
        )
    }
    pub fn free(&self) -> String {
        format!("scoreboard players reset {} {}", self.mcname(), NAMESPACE)
    }
    
    pub fn pure_calc_num(&self, operator:&str, num:i32) -> Result<String, EvaluateError> {
        match operator {
            "+" | "-" => Ok(
                format!(
                    "scoreboard players {} {} {} {}",
                    if &operator == &"+" {"add"} else {"remove"},
                    self.mcname(),
                    NAMESPACE,
                    num.to_string()
                )
            ),
            _ => {
                let source = Scoreboard::from(num);
                Ok(
                    format!(
                        "{}\n{}",
                        source.assign(&Calcable::Int(num))?,
                        int::calc(&self, operator, &Calcable::Scr(&source))?
                    )
                )
            }
        }
    }
    pub fn pure_calc_score(&self, operator:&str, source:&Scoreboard) -> String {
        format!(
            "scoreboard players operation {} {} {}= {} {}",
            self.mcname(),
            NAMESPACE,
            &operator,
            source.mcname(),
            NAMESPACE
        )
    }
    pub fn pure_assign_value(&self, value:i32) -> String {
        format!(
            "scoreboard players set {} {} {}",
            self.mcname(),
            NAMESPACE,
            value
        )
    }
    pub fn pure_assign_score(&self, value:&Scoreboard) -> String {
        format!(
            "scoreboard players operation {} {} = {} {}",
            self.mcname(),
            NAMESPACE,
            value.mcname(),
            NAMESPACE
        )
    }
    pub fn pure_compare_score(&self, operator:&str, value:&Scoreboard) -> String {
        format!(
            "score {} {} {} {} {}",
            self.mcname(),
            NAMESPACE,
            operator,
            value.mcname(),
            NAMESPACE
        )
    }
    pub fn pure_compare_value(&self, operator:&str, value:i32) -> Result<String, EvaluateError> {
        Ok(
            format!(
                "score {} {} matches {}",
                self.mcname(),
                NAMESPACE,
                match operator {
                    "==" => value.to_string(),
                    ">=" => format!("{}..", value),
                    "<=" => format!("..{}", value),
                    _ => return Err(EvaluateError::UnknownOperatorGiven(operator.to_string()))
                }
            )
        )
    }
    pub fn pure_compare_value_not_equal(&self, operator:&str, value:i32) -> (String, String) {
        let constant = Scoreboard::from(value);
        (constant.pure_assign_value(value), constant.pure_compare_score(operator, &constant))
    }
}

impl std::fmt::Display for Scoreboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.mcname())
    }
}