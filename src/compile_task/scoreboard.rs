use std::{fmt, vec};

use super::MCFunction;

const NAMESPACE:&str = "MCPP.var";

#[test]
fn valid_check() {
    let test = Scoreboard {
        name  : "Hoge".to_string(),
        scope : vec!["TEST".to_string()]
    };
    let test2 = Scoreboard {
        name  : "Fuga".to_string(),
        scope : vec!["TEST".to_string()]
    };
    assert_eq!(test.calc("+", &Calcable::Int(5)), "scoreboard add #TEST.Hoge MCPP.var 5");
    assert_eq!(test.calc("-", &Calcable::Int(5)), "scoreboard remove #TEST.Hoge MCPP.var 5");
    assert_eq!(test.calc("*", &Calcable::Int(5)), "scoreboard players set #CONSTANT.5 MCPP.var 5\nscoreboard players operation #TEST.Hoge MCPP.var *= #CONSTANT.5 MCPP.var");
    assert_eq!(test.calc("*", &Calcable::Scr(&test2)), "scoreboard players operation #TEST.Hoge MCPP.var *= #TEST.Fuga MCPP.var");
}
#[derive(Debug, Clone)]
pub struct 
Scoreboard {
    pub name  : String,
    pub scope : Vec<String>
}
pub enum Calcable<'a> {
    Int(i32),
    Scr(&'a Scoreboard),
    Mcf(&'a MCFunction)
}
impl fmt::Display for Calcable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Calcable::Int(i) => i.to_string(),
            Calcable::Scr(s) => s.to_string(),
            Calcable::Mcf(f) => format!("{}(...)", f.name)
        })
    }
}
impl From<i32> for Scoreboard {
    fn from(value: i32) -> Self {
        return Scoreboard {
            name  : value.to_string(),
            scope : vec!["CONSTANT".to_string()]
        };
    }
}

impl PartialEq for Scoreboard {
    fn eq(&self, other: &Self) -> bool {
        self.mcname() == other.mcname()
    }
}

impl Scoreboard {
    pub fn mcname(&self) -> String {
        let mut last = self.scope.to_vec();
        last.push(self.name.to_string());
        return format!("{}{}", "#", last.join("."));
    }
    fn calc_num(&self, operator:&str, num:i32) -> String {
        if ["+", "-"].contains(&operator) {
            let add_or_rem = if &operator == &"+" { "add" } else {"remove"};
            return format!("scoreboard {} {} {} {}", add_or_rem, self.mcname(), NAMESPACE, num.to_string());
        } else {
            let source = Scoreboard::from(num);
            return format!("{}\n{}", source.assign(&Calcable::Int(num)), self.calc_score(operator, &source));
        }
    }
    fn calc_score(&self, operator:&str, source:&Scoreboard) -> String {
        return format!("scoreboard players operation {} {} {}= {} {}", self.mcname(), NAMESPACE, &operator, source.mcname(), NAMESPACE);
    }
    pub fn calc(&self, operator:&str, source:&Calcable) -> String {
        return match &source {
            &Calcable::Int(i) => self.calc_num(operator, *i),
            &Calcable::Scr(s) => self.calc_score(operator, &s),
            &Calcable::Mcf(f) => { format!("{}\n{}", f.call(), self.calc_score(operator, &f.ret_container)) }
        };
    }
    pub fn assign(&self, source:&Calcable) -> String {
        return match &source {
            &Calcable::Int(i) => format!("scoreboard players set {} {} {}", self.mcname(), NAMESPACE, i),
            &Calcable::Scr(s) => format!("scoreboard players operation {} {} = {} {}", self.mcname(), NAMESPACE, s.mcname(), NAMESPACE),
            &Calcable::Mcf(f) => self.assign(&Calcable::Scr(&f.ret_container))
        };
    }
    pub fn free(&self) -> String {
        format!("scoreboard players reset {} {}", self.mcname(), NAMESPACE)
    }
}

impl std::fmt::Display for Scoreboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.mcname())
    }
}