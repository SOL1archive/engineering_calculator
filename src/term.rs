use std::collections::LinkedList;

pub enum TermType {
    Polymomial,
    Log { base: f64 },
    Exp { base: f64 },
    SubFunction { formula: Formula }
}

#[derive(PartialEq)]
pub enum OperatorType {
    Plus,
    Multiply,
}

pub struct Term {
    operator: OperatorType,
    term_type: TermType,
    coefficient: f64,
    exponent: f64
}

impl Term {
    pub fn new(operator: OperatorType, term_type: TermType, coefficient: f64, exponent: f64) -> Term{
        Term {
            operator: operator,
            term_type: term_type,
            coefficient: coefficient,
            exponent: exponent,
        }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        if self.exponent == 0. {
            return self.coefficient;
        }

        let result = match &self.term_type {
            TermType::Polymomial => {
                x
            },
            TermType::Exp { base } => {
                base.powf(x)
            },
            TermType::Log { base } => {
                x.log(*base)
            },
            TermType::SubFunction { formula } => {
                formula.evaluate(x)
            }
        };
        let result: f64 = self.coefficient * result.powf(self.exponent);

        result
    }
}

pub struct Formula {
    formula: LinkedList<Term>,
}

impl Formula {
    pub fn new() -> Formula {
        Formula {
            formula: LinkedList::new(),
        }
    }

    pub fn push(&mut self, term: Term) {
        self.formula.push_back(term);
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result: f64 = 0.;
        let mut temp_result: f64 = 0.;
        let mut poped_formula = self.formula.iter();
        poped_formula.next();
        for (term, next_term) in self.formula.iter().zip(poped_formula) {
            let sub_result: f64 = term.evaluate(x);
            match term.operator {
                OperatorType::Plus => {
                    match next_term.operator {
                        OperatorType::Plus => {
                            result += temp_result + sub_result;
                            temp_result = 0.;
                        },
                        OperatorType::Multiply => {
                            temp_result = sub_result;
                        }
                    }
                },
                OperatorType::Multiply => {
                    match next_term.operator {
                        OperatorType::Plus => {
                            result *= temp_result * sub_result;
                            temp_result = 0.;
                        },
                        OperatorType::Multiply => {
                            temp_result = sub_result;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn differential(&self, x: f64) -> f64 {
        (self.evaluate(x + f64::EPSILON) - self.evaluate(x - f64::EPSILON)) / (2. * f64::EPSILON)
    }

    pub fn integral(&self, start: f64, end: f64) -> f64 {
        let interval: f64 = (end - start) / f64::EPSILON;
        let n: u64 = (1. / f64::EPSILON) as u64;
        let mut result: f64 = 0.;
        for i in 0..n {
            let i: f64 = i as f64;
            result += (self.evaluate(start + i * interval) + self.evaluate(start + (i + 1.) * interval)) * interval * 2.;
        }
        
        if start > end {
            result *= -1.;
        }

        result
    }
}
