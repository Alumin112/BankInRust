use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
};

pub type Money = f64;

pub trait Currency: Sized + Display + Debug + PartialEq {
    const SYMBOL: &'static str;
    const SELF_TO_ITC_RATIO: f64;

    fn new(amount: Money) -> Self;
    fn amount(&self) -> Money;

    fn from_currency<T: Currency>(currency: T) -> Self {
        Self::new(currency.amount() * (T::SELF_TO_ITC_RATIO / Self::SELF_TO_ITC_RATIO) as Money)
    }

    fn to_currency<T: Currency>(self) -> T {
        T::new(self.amount() * (Self::SELF_TO_ITC_RATIO / T::SELF_TO_ITC_RATIO) as Money)
    }

    fn to_itc(self) -> ITC {
        self.to_currency()
    }

    fn from_itc(currency: ITC) -> Self {
        Self::from_currency(currency)
    }

    fn as_currency<T: Currency>(&self) -> T {
        T::new(self.amount() * (Self::SELF_TO_ITC_RATIO / T::SELF_TO_ITC_RATIO) as Money)
    }
}

#[macro_export]
macro_rules! currency {
    ($x:tt, $y:expr, $z:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $x(pub crate::currency::Money);

        impl crate::currency::Currency for $x {
            const SYMBOL: &'static str = $z;
            const SELF_TO_ITC_RATIO: f64 = $y;

            fn new(amount: crate::currency::Money) -> Self {
                Self(amount)
            }

            fn amount(&self) -> crate::currency::Money {
                self.0
            }
        }

        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}{}", Self::SYMBOL, self.amount())
            }
        }
    };
}

currency!(ITC, 1.0, "ITC::");
