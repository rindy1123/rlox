use std::cmp;
use std::ops;

#[derive(Debug, Clone)]
pub enum LiteralType {
    Str(String),
    Num(f64),
    False,
    True,
    Nil,
    Error(String),
}

pub fn convert_bool_to_literal_bool(b: bool) -> LiteralType {
    if b {
        LiteralType::True
    } else {
        LiteralType::False
    }
}

pub mod comparison {
    use super::LiteralType;

    pub fn gt(left: LiteralType, right: LiteralType) -> LiteralType {
        match (left, right) {
            (LiteralType::Num(left_num), LiteralType::Num(right_num)) => {
                if left_num > right_num {
                    LiteralType::True
                } else {
                    LiteralType::False
                }
            }
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }

    pub fn ge(left: LiteralType, right: LiteralType) -> LiteralType {
        match (left, right) {
            (LiteralType::Num(left_num), LiteralType::Num(right_num)) => {
                if left_num >= right_num {
                    LiteralType::True
                } else {
                    LiteralType::False
                }
            }
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }

    pub fn lt(left: LiteralType, right: LiteralType) -> LiteralType {
        match (left, right) {
            (LiteralType::Num(left_num), LiteralType::Num(right_num)) => {
                if left_num < right_num {
                    LiteralType::True
                } else {
                    LiteralType::False
                }
            }
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }

    pub fn le(left: LiteralType, right: LiteralType) -> LiteralType {
        match (left, right) {
            (LiteralType::Num(left_num), LiteralType::Num(right_num)) => {
                if left_num <= right_num {
                    LiteralType::True
                } else {
                    LiteralType::False
                }
            }
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }
}

impl ops::Sub for LiteralType {
    type Output = Self;

    fn sub(self, right: Self) -> Self::Output {
        match (self, right) {
            (Self::Num(left_num), Self::Num(right_num)) => LiteralType::Num(left_num - right_num),
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }
}

impl ops::Add for LiteralType {
    type Output = Self;

    fn add(self, right: Self) -> Self::Output {
        match (self, right) {
            (Self::Num(left_num), Self::Num(right_num)) => LiteralType::Num(left_num + right_num),
            (Self::Str(left_str), Self::Str(right_str)) => LiteralType::Str(left_str + &right_str),
            _ => LiteralType::Error("Operands must be numbers or strings.".to_string()),
        }
    }
}

impl ops::Mul for LiteralType {
    type Output = Self;

    fn mul(self, right: Self) -> Self::Output {
        match (self, right) {
            (Self::Num(left_num), Self::Num(right_num)) => LiteralType::Num(left_num * right_num),
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }
}

impl ops::Div for LiteralType {
    type Output = Self;

    fn div(self, right: Self) -> Self::Output {
        match (self, right) {
            (Self::Num(left_num), Self::Num(right_num)) => LiteralType::Num(left_num / right_num),
            _ => LiteralType::Error("Operands must be numbers.".to_string()),
        }
    }
}

impl ops::Neg for LiteralType {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let LiteralType::Num(n) = self {
            return LiteralType::Num(-n);
        }
        LiteralType::Error("Operand must be a number.".to_string())
    }
}

impl ops::Not for LiteralType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::False | Self::Nil => LiteralType::True,
            _ => LiteralType::False,
        }
    }
}

impl cmp::PartialEq for LiteralType {
    fn eq(&self, right: &Self) -> bool {
        match self {
            Self::Num(left_num) => {
                if let LiteralType::Num(right_num) = right.clone() {
                    return *left_num == right_num;
                };
                false
            }
            Self::Str(left_str) => {
                if let LiteralType::Str(right_str) = right.clone() {
                    return *left_str == right_str;
                };
                false
            }
            Self::Nil => matches!(right, Self::Nil),
            Self::True => matches!(right, Self::True),
            Self::False => matches!(right, Self::False),
            _ => panic!("Do not use this value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ops {
        use super::*;

        #[test]
        fn test_add_num() {
            let left = LiteralType::Num(2.0);
            let right = LiteralType::Num(1.0);
            assert_eq!(left + right, LiteralType::Num(3.0))
        }

        #[test]
        fn test_add_str() {
            let left = LiteralType::Str("Hello".to_string());
            let right = LiteralType::Str(" World".to_string());
            assert_eq!(left + right, LiteralType::Str("Hello World".to_string()))
        }

        #[test]
        fn test_sub() {
            let left = LiteralType::Num(-2.0);
            let right = LiteralType::Num(1.0);
            assert_eq!(left - right, LiteralType::Num(-3.0))
        }

        #[test]
        fn test_mul() {
            let left = LiteralType::Num(2.0);
            let right = LiteralType::Num(3.0);
            assert_eq!(left * right, LiteralType::Num(6.0))
        }

        #[test]
        fn test_div() {
            let left = LiteralType::Num(9.0);
            let right = LiteralType::Num(3.0);
            assert_eq!(left / right, LiteralType::Num(3.0))
        }

        #[test]
        fn test_neg() {
            let right = LiteralType::Num(3.0);
            assert_eq!(-right, LiteralType::Num(-3.0))
        }

        #[test]
        fn test_not() {
            assert_eq!(!LiteralType::False, LiteralType::True);
            assert_eq!(!LiteralType::Nil, LiteralType::True);
            assert_eq!(!LiteralType::Str("Hello".to_string()), LiteralType::False);
        }
    }

    mod eq {
        use super::*;

        #[test]
        fn test_eq_num() {
            let left = LiteralType::Num(123.0);
            let right = LiteralType::Num(123.0);
            assert_eq!(left, right)
        }

        #[test]
        fn test_ne_num() {
            let left = LiteralType::Num(123.0);
            let right = LiteralType::Num(1.0);
            assert_ne!(left, right)
        }

        #[test]
        fn test_eq_others() {
            let nil = LiteralType::Nil;
            assert_eq!(nil, LiteralType::Nil)
        }

        #[test]
        fn test_ne_others() {
            let nil = LiteralType::Nil;
            assert_ne!(nil, LiteralType::True)
        }
    }

    mod cmp {
        use super::*;

        #[test]
        fn test_gt() {
            let left = LiteralType::Num(123.0);
            let right = LiteralType::Num(1.0);
            assert_eq!(comparison::gt(left, right), LiteralType::True)
        }

        #[test]
        fn test_ge() {
            let left = LiteralType::Num(123.0);
            let mut right = LiteralType::Num(1.0);
            assert_eq!(comparison::ge(left.clone(), right), LiteralType::True);
            right = LiteralType::Num(123.0);
            assert_eq!(comparison::ge(left, right), LiteralType::True)
        }

        #[test]
        fn test_lt() {
            let left = LiteralType::Num(1.0);
            let right = LiteralType::Num(123.0);
            assert_eq!(comparison::lt(left, right), LiteralType::True)
        }

        #[test]
        fn test_le() {
            let left = LiteralType::Num(1.0);
            let mut right = LiteralType::Num(123.0);
            assert_eq!(comparison::le(left.clone(), right), LiteralType::True);
            right = LiteralType::Num(1.0);
            assert_eq!(comparison::le(left, right), LiteralType::True)
        }
    }
}
