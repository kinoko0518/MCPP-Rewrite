mod int;
mod float;

use std::{fmt, vec};

use super::{evaluater::EvaluateError, MCFunction};

const NAMESPACE:&str = "MCPP.var";

#[test]
fn valid_check() {
    let test = Scoreboard {
        name  : "Hoge".to_string(),
        data_type : Types::Int,
        scope : vec!["TEST".to_string()]
    };
    let test2 = Scoreboard {
        name  : "Fuga".to_string(),
        data_type : Types::Int,
        scope : vec!["TEST".to_string()]
    };
    assert_eq!(test.calc("+", &Calcable::Int(5)).unwrap(), "scoreboard players add #TEST.Hoge MCPP.var 5");
    assert_eq!(test.calc("-", &Calcable::Int(5)).unwrap(), "scoreboard players remove #TEST.Hoge MCPP.var 5");
    assert_eq!(test.calc("*", &Calcable::Int(5)).unwrap(), "scoreboard players set #CONSTANT.5 MCPP.var 5\nscoreboard players operation #TEST.Hoge MCPP.var *= #CONSTANT.5 MCPP.var");
    assert_eq!(test.calc("*", &Calcable::Scr(&test2)).unwrap(), "scoreboard players operation #TEST.Hoge MCPP.var *= #TEST.Fuga MCPP.var");

    let float_test = Scoreboard {
        name : "Piyo".to_string(),
        data_type : Types::Flt,
        scope : vec!["TEST".to_string()]
    };
    assert_eq!(
        float_test.calc("+", &Calcable::Scr(&test2)).unwrap(),
        "scoreboard players operation #Calc.TEMP MCPP.var = #TEST.Fuga MCPP.var\n\
        scoreboard players set #CONSTANT.1000 MCPP.var 1000\n\
        scoreboard players operation #Calc.TEMP MCPP.var *= #CONSTANT.1000 MCPP.var\n\
        scoreboard players operation #TEST.Piyo MCPP.var += #Calc.TEMP MCPP.var"
    );
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
    Scr(&'a Scoreboard),
    Mcf(&'a MCFunction)
}
#[derive(Debug, Clone)]
pub enum Types {
    Int,
    Flt,
    Non,
}
impl fmt::Display for Calcable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Calcable::Int(i) => i.to_string(),
            Calcable::Scr(s) => s.to_string(),
            Calcable::Flt(f) => f.to_string(),
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
            Types::Non => Err(EvaluateError::OperationOccuredBetweenUnsupportedTypes)
        }
    }
    pub fn assign(&self, source:&Calcable) -> Result<String, EvaluateError> {
        match self.data_type {
            Types::Int => int::assign(&self, source),
            Types::Flt => float::assign(&self, source),
            Types::Non => Err(EvaluateError::AssignOccuredBetweenUnsupportedTypes)
        }
    }
    pub fn free(&self) -> String {
        format!("scoreboard players reset {} {}", self.mcname(), NAMESPACE)
    }
    
    fn pure_calc_num(&self, operator:&str, num:i32) -> Result<String, EvaluateError> {
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
    fn pure_calc_score(&self, operator:&str, source:&Scoreboard) -> String {
        format!(
            "scoreboard players operation {} {} {}= {} {}",
            self.mcname(),
            NAMESPACE,
            &operator,
            source.mcname(),
            NAMESPACE
        )
    }
    fn pure_assign_value(&self, value:i32) -> String {
        format!(
            "scoreboard players set {} {} {}",
            self.mcname(),
            NAMESPACE,
            value
        )
    }
    fn pure_assign_score(&self, value:&Scoreboard) -> String {
        format!(
            "scoreboard players operation {} {} = {} {}",
            self.mcname(),
            NAMESPACE,
            value.mcname(),
            NAMESPACE
        )
    }
}

impl std::fmt::Display for Scoreboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.mcname())
    }
}