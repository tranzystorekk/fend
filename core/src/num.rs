use std::fmt;

mod base;
mod bigrat;
mod biguint;
mod complex;
mod exact;
mod formatting_style;
mod real;
mod unit;

pub(crate) use formatting_style::FormattingStyle;

pub(crate) type Number = unit::Value;
pub(crate) type Base = base::Base;
pub(crate) type Exact<T> = exact::Exact<T>;

pub(crate) enum RangeBound<T> {
    None,
    Open(T),
    Closed(T),
}

pub(crate) struct Range<T> {
    start: RangeBound<T>,
    end: RangeBound<T>,
}

impl<T> Range<T> {
    pub(crate) fn open(start: T, end: T) -> Self {
        Self {
            start: RangeBound::Open(start),
            end: RangeBound::Open(end),
        }
    }
}

impl Range<i32> {
    const ZERO_OR_GREATER: Self = Self {
        start: RangeBound::Closed(0),
        end: RangeBound::None,
    };
}

impl<T: fmt::Display> fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.start {
            RangeBound::None => write!(f, "(-\u{221e}, ")?, // infinity symbol
            RangeBound::Open(v) => write!(f, "({}, ", v)?,
            RangeBound::Closed(v) => write!(f, "[{}, ", v)?,
        }
        match &self.end {
            RangeBound::None => write!(f, "\u{221e})")?,
            RangeBound::Open(v) => write!(f, "{})", v)?,
            RangeBound::Closed(v) => write!(f, "{}]", v)?,
        }
        Ok(())
    }
}

#[allow(clippy::pub_enum_variant_names)]
pub(crate) struct ValueOutOfRange<T: fmt::Display, U: fmt::Display>(T, Range<U>);

impl<T: fmt::Display, U: fmt::Display> fmt::Display for ValueOutOfRange<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} must lie in the interval {}", self.0, self.1)
    }
}

impl<T: fmt::Display, U: fmt::Display> crate::error::Error for ValueOutOfRange<T, U> {}

pub(crate) enum ConvertToUsizeError {
    OutOfRange(ValueOutOfRange<biguint::FormattedBigUint, usize>),
    NegativeNumber,
    Fraction,
    InvalidRealNumber,
    ComplexNumber,
    NumberWithUnit,
    InexactNumber,
}

impl fmt::Display for ConvertToUsizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::OutOfRange(value_out_of_range_error) => write!(f, "{}", value_out_of_range_error),
            Self::NegativeNumber => write!(f, "negative numbers are not allowed"),
            Self::Fraction => write!(f, "cannot convert fraction to integer"),
            Self::InvalidRealNumber => write!(f, "number cannot be converted to an integer"),
            Self::ComplexNumber => write!(f, "cannot convert complex number to integer"),
            Self::NumberWithUnit => write!(f, "cannot convert number with unit to integer"),
            Self::InexactNumber => write!(f, "cannot convert inexact number to integer"),
        }
    }
}

impl crate::error::Error for ConvertToUsizeError {}

#[derive(Debug)]
pub(crate) enum IntegerPowerError {
    ExponentTooLarge,
    ZeroToThePowerOfZero,
}

impl fmt::Display for IntegerPowerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::ExponentTooLarge => write!(f, "exponent too large"),
            Self::ZeroToThePowerOfZero => write!(f, "zero to the power of zero is undefined"),
        }
    }
}
impl crate::error::Error for IntegerPowerError {}
