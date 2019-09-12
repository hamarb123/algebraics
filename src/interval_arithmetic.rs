// SPDX-License-Identifier: LGPL-2.1-or-later
// See Notices.txt for copyright information

use crate::util::DebugAsDisplay;
use num_bigint::BigInt;
use num_bigint::BigUint;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::One;
use num_traits::Pow;
use num_traits::Signed;
use num_traits::Unsigned;
use num_traits::Zero;
use std::borrow::Cow;
use std::fmt;
use std::mem;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

fn convert_log2_denom_floor(numer: &mut BigInt, old_log2_denom: usize, new_log2_denom: usize) {
    if new_log2_denom >= old_log2_denom {
        *numer <<= new_log2_denom - old_log2_denom;
    } else {
        *numer >>= old_log2_denom - new_log2_denom;
    }
}

fn convert_log2_denom_ceil(numer: &mut BigInt, old_log2_denom: usize, new_log2_denom: usize) {
    if new_log2_denom >= old_log2_denom {
        *numer <<= new_log2_denom - old_log2_denom;
    } else {
        let mut numer_value = mem::replace(numer, Default::default());
        numer_value = -numer_value;
        numer_value >>= old_log2_denom - new_log2_denom;
        numer_value = -numer_value;
        *numer = numer_value;
    }
}

/// inclusive interval of the form `[a / 2^n, b / 2^n]` where `a` and `b` are integers and `n` is an unsigned integer.
#[derive(Clone, Default)]
pub struct DyadicFractionInterval {
    pub lower_bound_numer: BigInt,
    pub upper_bound_numer: BigInt,
    pub log2_denom: usize,
}

impl DyadicFractionInterval {
    pub fn new(lower_bound_numer: BigInt, upper_bound_numer: BigInt, log2_denom: usize) -> Self {
        Self {
            lower_bound_numer,
            upper_bound_numer,
            log2_denom,
        }
    }
    pub fn from_ratio_range(
        lower_bound: Ratio<BigInt>,
        upper_bound: Ratio<BigInt>,
        log2_denom: usize,
    ) -> Self {
        let denom = BigInt::one() << log2_denom;
        let lower_bound_numer = (lower_bound * &denom).floor().to_integer();
        let upper_bound_numer = (upper_bound * denom).ceil().to_integer();
        Self {
            lower_bound_numer,
            upper_bound_numer,
            log2_denom,
        }
    }
    pub fn from_ratio(ratio: Ratio<BigInt>, log2_denom: usize) -> Self {
        let (mut numer, denom) = ratio.into();
        numer <<= log2_denom;
        let ratio = Ratio::new(numer, denom);
        let lower_bound_numer = ratio.floor().to_integer();
        let upper_bound_numer = ratio.ceil().to_integer();
        Self {
            lower_bound_numer,
            upper_bound_numer,
            log2_denom,
        }
    }
    pub fn from_dyadic_fraction(numer: BigInt, log2_denom: usize) -> Self {
        Self {
            lower_bound_numer: numer.clone(),
            upper_bound_numer: numer,
            log2_denom,
        }
    }
    pub fn zero(log2_denom: usize) -> Self {
        Self {
            lower_bound_numer: BigInt::zero(),
            upper_bound_numer: BigInt::zero(),
            log2_denom,
        }
    }
    pub fn one(log2_denom: usize) -> Self {
        Self::from_dyadic_fraction(BigInt::one() << log2_denom, log2_denom)
    }
    pub fn negative_one(log2_denom: usize) -> Self {
        Self::from_dyadic_fraction(-(BigInt::one() << log2_denom), log2_denom)
    }
    pub fn set_zero(&mut self) {
        self.lower_bound_numer.set_zero();
        self.upper_bound_numer.set_zero();
    }
    pub fn set_one(&mut self) {
        self.lower_bound_numer.set_one();
        self.lower_bound_numer <<= self.log2_denom;
        self.upper_bound_numer.clone_from(&self.lower_bound_numer);
    }
    pub fn set_negative_one(&mut self) {
        self.lower_bound_numer.set_one();
        self.lower_bound_numer <<= self.log2_denom;
        self.lower_bound_numer = -mem::replace(&mut self.lower_bound_numer, Default::default());
        self.upper_bound_numer.clone_from(&self.lower_bound_numer);
    }
    pub fn into_ratio_range(self) -> (Ratio<BigInt>, Ratio<BigInt>) {
        let denom = BigInt::one() << self.log2_denom;
        (
            Ratio::new(self.lower_bound_numer, denom.clone()),
            Ratio::new(self.upper_bound_numer, denom),
        )
    }
    pub fn to_ratio_range(&self) -> (Ratio<BigInt>, Ratio<BigInt>) {
        self.clone().into_ratio_range()
    }
    pub fn convert_log2_denom(&mut self, log2_denom: usize) {
        convert_log2_denom_floor(&mut self.lower_bound_numer, self.log2_denom, log2_denom);
        convert_log2_denom_ceil(&mut self.upper_bound_numer, self.log2_denom, log2_denom);
        self.log2_denom = log2_denom;
    }
    pub fn into_converted_log2_denom(mut self, log2_denom: usize) -> Self {
        self.convert_log2_denom(log2_denom);
        self
    }
    pub fn to_converted_log2_denom(&self, log2_denom: usize) -> Self {
        self.clone().into_converted_log2_denom(log2_denom)
    }
    fn do_add_sub_mul_assign<Op: Fn(&mut BigInt, &mut BigInt, &BigInt, &BigInt, usize)>(
        &mut self,
        rhs: Cow<DyadicFractionInterval>,
        op: Op,
    ) {
        if rhs.log2_denom >= self.log2_denom {
            let shift_amount = rhs.log2_denom - self.log2_denom;
            self.lower_bound_numer <<= shift_amount;
            self.upper_bound_numer <<= shift_amount;
            self.log2_denom = rhs.log2_denom;
            op(
                &mut self.lower_bound_numer,
                &mut self.upper_bound_numer,
                &rhs.lower_bound_numer,
                &rhs.upper_bound_numer,
                self.log2_denom,
            );
        } else {
            let shift_amount = self.log2_denom - rhs.log2_denom;
            let rhs_lower_bound_numer;
            let rhs_upper_bound_numer;
            match rhs {
                Cow::Borrowed(rhs) => {
                    rhs_lower_bound_numer = &rhs.lower_bound_numer << shift_amount;
                    rhs_upper_bound_numer = &rhs.upper_bound_numer << shift_amount;
                }
                Cow::Owned(rhs) => {
                    rhs_lower_bound_numer = rhs.lower_bound_numer << shift_amount;
                    rhs_upper_bound_numer = rhs.upper_bound_numer << shift_amount;
                }
            }
            op(
                &mut self.lower_bound_numer,
                &mut self.upper_bound_numer,
                &rhs_lower_bound_numer,
                &rhs_upper_bound_numer,
                self.log2_denom,
            );
        }
    }
    fn do_add_assign(&mut self, rhs: Cow<DyadicFractionInterval>) {
        self.do_add_sub_mul_assign(
            rhs,
            |lhs_lower_bound_numer,
             lhs_upper_bound_numer,
             rhs_lower_bound_numer,
             rhs_upper_bound_numer,
             _log2_denom| {
                *lhs_lower_bound_numer += rhs_lower_bound_numer;
                *lhs_upper_bound_numer += rhs_upper_bound_numer;
            },
        );
    }
    fn do_sub_assign(&mut self, rhs: Cow<DyadicFractionInterval>) {
        self.do_add_sub_mul_assign(
            rhs,
            |lhs_lower_bound_numer,
             lhs_upper_bound_numer,
             rhs_lower_bound_numer,
             rhs_upper_bound_numer,
             _log2_denom| {
                // rhs swapped and subtracted
                *lhs_lower_bound_numer -= rhs_upper_bound_numer;
                *lhs_upper_bound_numer -= rhs_lower_bound_numer;
            },
        );
    }
    fn do_mul_assign_int(&mut self, rhs: &BigInt) {
        if rhs.is_negative() {
            mem::swap(&mut self.lower_bound_numer, &mut self.upper_bound_numer);
        }
        self.lower_bound_numer.mul_assign(rhs);
        self.upper_bound_numer.mul_assign(rhs);
    }
    fn do_mul_assign_ratio(&mut self, rhs: &Ratio<BigInt>) {
        if rhs.is_negative() {
            mem::swap(&mut self.lower_bound_numer, &mut self.upper_bound_numer);
        }
        self.lower_bound_numer = (rhs * &self.lower_bound_numer).floor().to_integer();
        self.upper_bound_numer = (rhs * &self.upper_bound_numer).ceil().to_integer();
    }
    fn do_mul_assign(&mut self, rhs: Cow<DyadicFractionInterval>) {
        self.do_add_sub_mul_assign(
            rhs,
            |lhs_lower_bound_numer,
             lhs_upper_bound_numer,
             rhs_lower_bound_numer,
             rhs_upper_bound_numer,
             log2_denom| {
                let mut bounds = [
                    Some(&*lhs_lower_bound_numer * rhs_lower_bound_numer),
                    Some(&*lhs_lower_bound_numer * rhs_upper_bound_numer),
                    Some(&*lhs_upper_bound_numer * rhs_lower_bound_numer),
                    Some(&*lhs_upper_bound_numer * rhs_upper_bound_numer),
                ];
                let mut lower_bound = None;
                for bound in &mut bounds {
                    match (&mut lower_bound, bound) {
                        (_, None) => {}
                        (None, bound) => lower_bound = bound.take(),
                        (Some(lower_bound), Some(bound)) => {
                            if *bound < *lower_bound {
                                mem::swap(lower_bound, bound)
                            }
                        }
                    }
                }
                let mut upper_bound = None;
                for bound in &mut bounds {
                    match (&mut upper_bound, bound) {
                        (_, None) => {}
                        (None, bound) => upper_bound = bound.take(),
                        (Some(upper_bound), Some(bound)) => {
                            if *bound > *upper_bound {
                                mem::swap(upper_bound, bound)
                            }
                        }
                    }
                }
                *lhs_lower_bound_numer = lower_bound.expect("known to exist") >> log2_denom;
                *lhs_upper_bound_numer = -(-upper_bound.expect("known to exist") >> log2_denom);
            },
        );
    }
    pub fn into_square(mut self) -> Self {
        let contains_zero = self.contains_zero();
        let lower_bound_numer_is_negative = self.lower_bound_numer.is_negative();
        let upper_bound_numer_is_negative = self.upper_bound_numer.is_negative();
        let mut min = if lower_bound_numer_is_negative {
            -self.lower_bound_numer
        } else {
            self.lower_bound_numer
        };
        let mut max = if upper_bound_numer_is_negative {
            -self.upper_bound_numer
        } else {
            self.upper_bound_numer
        };
        if min > max {
            mem::swap(&mut min, &mut max);
        }
        self.lower_bound_numer = if contains_zero {
            BigInt::zero()
        } else {
            (&min * &min) >> self.log2_denom
        };
        self.upper_bound_numer = (&max * &max) >> self.log2_denom;
        self
    }
    pub fn square_assign(&mut self) {
        *self = mem::replace(self, Default::default()).into_square();
    }
    pub fn square(&self) -> Self {
        self.clone().into_square()
    }
    fn do_sqrt(radicand: Cow<Self>) -> Self {
        let log2_denom = radicand.log2_denom;
        let (scaled_lower_bound_numer, scaled_upper_bound_numer) = match radicand {
            Cow::Borrowed(radicand) => (
                &radicand.lower_bound_numer << log2_denom,
                &radicand.upper_bound_numer << log2_denom,
            ),
            Cow::Owned(radicand) => (
                radicand.lower_bound_numer << log2_denom,
                radicand.upper_bound_numer << log2_denom,
            ),
        };
        let lower_bound_numer = scaled_lower_bound_numer.sqrt();
        let sqrt = scaled_upper_bound_numer.sqrt();
        let upper_bound_numer = if &sqrt * &sqrt == scaled_upper_bound_numer {
            sqrt
        } else {
            sqrt + 1
        };
        Self {
            lower_bound_numer,
            upper_bound_numer,
            log2_denom,
        }
    }
    pub fn sqrt_assign(&mut self) {
        *self = mem::replace(self, Default::default()).into_sqrt();
    }
    pub fn into_sqrt(self) -> Self {
        Self::do_sqrt(Cow::Owned(self))
    }
    pub fn sqrt(&self) -> Self {
        Self::do_sqrt(Cow::Borrowed(self))
    }
    pub fn contains_zero(&self) -> bool {
        !self.lower_bound_numer.is_positive() && !self.upper_bound_numer.is_negative()
    }
}

impl fmt::Debug for DyadicFractionInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DyadicFractionInterval")
            .field("log2_denom", &self.log2_denom)
            .field(
                "lower_bound_numer",
                &DebugAsDisplay(&self.lower_bound_numer),
            )
            .field(
                "upper_bound_numer",
                &DebugAsDisplay(&self.upper_bound_numer),
            )
            .finish()
    }
}

impl fmt::Display for DyadicFractionInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{} / 2^{}, {} / 2^{}]",
            self.lower_bound_numer, self.log2_denom, self.upper_bound_numer, self.log2_denom
        )
    }
}

macro_rules! forward_op_to_op_assign {
    ($op_assign_trait:ident, $op_assign:ident, $op_trait:ident, $op:ident, $rhs:ty) => {
        impl $op_trait<$rhs> for DyadicFractionInterval {
            type Output = DyadicFractionInterval;
            fn $op(mut self, rhs: $rhs) -> DyadicFractionInterval {
                self.$op_assign(rhs);
                self
            }
        }

        impl $op_trait<&'_ $rhs> for DyadicFractionInterval {
            type Output = DyadicFractionInterval;
            fn $op(mut self, rhs: &$rhs) -> DyadicFractionInterval {
                self.$op_assign(rhs);
                self
            }
        }

        impl $op_trait<$rhs> for &'_ DyadicFractionInterval {
            type Output = DyadicFractionInterval;
            fn $op(self, rhs: $rhs) -> DyadicFractionInterval {
                self.clone().$op(rhs)
            }
        }

        impl<'a, 'b> $op_trait<&'a $rhs> for &'b DyadicFractionInterval {
            type Output = DyadicFractionInterval;
            fn $op(self, rhs: &$rhs) -> DyadicFractionInterval {
                self.clone().$op(rhs)
            }
        }
    };
}

macro_rules! forward_type_to_bigint {
    ($op_assign_trait:ident, $op_assign:ident, $op_trait:ident, $op:ident, $rhs:ty) => {
        impl $op_assign_trait<$rhs> for DyadicFractionInterval {
            fn $op_assign(&mut self, rhs: $rhs) {
                self.$op_assign(BigInt::from(rhs));
            }
        }

        impl $op_assign_trait<&'_ $rhs> for DyadicFractionInterval {
            fn $op_assign(&mut self, rhs: &$rhs) {
                self.$op_assign(BigInt::from(rhs.clone()));
            }
        }

        forward_op_to_op_assign!($op_assign_trait, $op_assign, $op_trait, $op, $rhs);

        impl $op_assign_trait<Ratio<$rhs>> for DyadicFractionInterval {
            fn $op_assign(&mut self, rhs: Ratio<$rhs>) {
                let (numer, denom) = rhs.into();
                self.$op_assign(Ratio::<BigInt>::new(numer.into(), denom.into()));
            }
        }

        impl $op_assign_trait<&'_ Ratio<$rhs>> for DyadicFractionInterval {
            fn $op_assign(&mut self, rhs: &Ratio<$rhs>) {
                self.$op_assign(rhs.clone());
            }
        }

        forward_op_to_op_assign!($op_assign_trait, $op_assign, $op_trait, $op, Ratio<$rhs>);
    };
}

macro_rules! forward_types_to_bigint {
    ($op_assign_trait:ident, $op_assign:ident, $op_trait:ident, $op:ident) => {
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, u8);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, u16);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, u32);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, u64);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, u128);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, usize);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, BigUint);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, i8);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, i16);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, i32);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, i64);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, i128);
        forward_type_to_bigint!($op_assign_trait, $op_assign, $op_trait, $op, isize);
    };
}

impl AddAssign<DyadicFractionInterval> for DyadicFractionInterval {
    fn add_assign(&mut self, rhs: DyadicFractionInterval) {
        self.do_add_assign(Cow::Owned(rhs));
    }
}

impl AddAssign<&'_ DyadicFractionInterval> for DyadicFractionInterval {
    fn add_assign(&mut self, rhs: &DyadicFractionInterval) {
        self.do_add_assign(Cow::Borrowed(rhs));
    }
}

impl AddAssign<BigInt> for DyadicFractionInterval {
    fn add_assign(&mut self, mut rhs: BigInt) {
        rhs <<= self.log2_denom;
        self.lower_bound_numer.add_assign(&rhs);
        self.upper_bound_numer.add_assign(rhs);
    }
}

impl AddAssign<&'_ BigInt> for DyadicFractionInterval {
    fn add_assign(&mut self, rhs: &BigInt) {
        #![allow(clippy::suspicious_op_assign_impl)]
        let rhs = rhs << self.log2_denom;
        self.lower_bound_numer.add_assign(&rhs);
        self.upper_bound_numer.add_assign(rhs);
    }
}

impl AddAssign<Ratio<BigInt>> for DyadicFractionInterval {
    fn add_assign(&mut self, rhs: Ratio<BigInt>) {
        self.add_assign(DyadicFractionInterval::from_ratio(rhs, self.log2_denom))
    }
}

impl AddAssign<&'_ Ratio<BigInt>> for DyadicFractionInterval {
    fn add_assign(&mut self, rhs: &Ratio<BigInt>) {
        self.add_assign(rhs.clone())
    }
}

forward_types_to_bigint!(AddAssign, add_assign, Add, add);
forward_op_to_op_assign!(AddAssign, add_assign, Add, add, DyadicFractionInterval);
forward_op_to_op_assign!(AddAssign, add_assign, Add, add, Ratio<BigInt>);

impl SubAssign<DyadicFractionInterval> for DyadicFractionInterval {
    fn sub_assign(&mut self, rhs: DyadicFractionInterval) {
        self.do_sub_assign(Cow::Owned(rhs));
    }
}

impl SubAssign<&'_ DyadicFractionInterval> for DyadicFractionInterval {
    fn sub_assign(&mut self, rhs: &DyadicFractionInterval) {
        self.do_sub_assign(Cow::Borrowed(rhs));
    }
}

impl SubAssign<BigInt> for DyadicFractionInterval {
    fn sub_assign(&mut self, mut rhs: BigInt) {
        rhs <<= self.log2_denom;
        self.lower_bound_numer.sub_assign(&rhs);
        self.upper_bound_numer.sub_assign(rhs);
    }
}

impl SubAssign<&'_ BigInt> for DyadicFractionInterval {
    fn sub_assign(&mut self, rhs: &BigInt) {
        #![allow(clippy::suspicious_op_assign_impl)]
        let rhs = rhs << self.log2_denom;
        self.lower_bound_numer.sub_assign(&rhs);
        self.upper_bound_numer.sub_assign(rhs);
    }
}

impl SubAssign<Ratio<BigInt>> for DyadicFractionInterval {
    fn sub_assign(&mut self, rhs: Ratio<BigInt>) {
        self.sub_assign(DyadicFractionInterval::from_ratio(rhs, self.log2_denom))
    }
}

impl SubAssign<&'_ Ratio<BigInt>> for DyadicFractionInterval {
    fn sub_assign(&mut self, rhs: &Ratio<BigInt>) {
        self.sub_assign(rhs.clone())
    }
}

forward_types_to_bigint!(SubAssign, sub_assign, Sub, sub);
forward_op_to_op_assign!(SubAssign, sub_assign, Sub, sub, DyadicFractionInterval);
forward_op_to_op_assign!(SubAssign, sub_assign, Sub, sub, Ratio<BigInt>);

impl MulAssign<DyadicFractionInterval> for DyadicFractionInterval {
    fn mul_assign(&mut self, rhs: DyadicFractionInterval) {
        self.do_mul_assign(Cow::Owned(rhs));
    }
}

impl MulAssign<&'_ DyadicFractionInterval> for DyadicFractionInterval {
    fn mul_assign(&mut self, rhs: &DyadicFractionInterval) {
        self.do_mul_assign(Cow::Borrowed(rhs));
    }
}

impl MulAssign<BigInt> for DyadicFractionInterval {
    fn mul_assign(&mut self, rhs: BigInt) {
        self.do_mul_assign_int(&rhs);
    }
}

impl MulAssign<&'_ BigInt> for DyadicFractionInterval {
    fn mul_assign(&mut self, rhs: &BigInt) {
        self.do_mul_assign_int(rhs);
    }
}

impl MulAssign<Ratio<BigInt>> for DyadicFractionInterval {
    fn mul_assign(&mut self, rhs: Ratio<BigInt>) {
        self.do_mul_assign_ratio(&rhs);
    }
}

impl MulAssign<&'_ Ratio<BigInt>> for DyadicFractionInterval {
    fn mul_assign(&mut self, rhs: &Ratio<BigInt>) {
        self.do_mul_assign_ratio(rhs);
    }
}

forward_types_to_bigint!(MulAssign, mul_assign, Mul, mul);
forward_op_to_op_assign!(MulAssign, mul_assign, Mul, mul, DyadicFractionInterval);
forward_op_to_op_assign!(MulAssign, mul_assign, Mul, mul, Ratio<BigInt>);

impl DivAssign<BigInt> for DyadicFractionInterval {
    fn div_assign(&mut self, rhs: BigInt) {
        self.do_mul_assign_ratio(&Ratio::new(BigInt::one(), rhs));
    }
}

impl DivAssign<&'_ BigInt> for DyadicFractionInterval {
    fn div_assign(&mut self, rhs: &BigInt) {
        self.do_mul_assign_ratio(&Ratio::new(BigInt::one(), rhs.clone()));
    }
}

impl DivAssign<Ratio<BigInt>> for DyadicFractionInterval {
    fn div_assign(&mut self, rhs: Ratio<BigInt>) {
        self.do_mul_assign_ratio(&rhs.recip());
    }
}

impl DivAssign<&'_ Ratio<BigInt>> for DyadicFractionInterval {
    fn div_assign(&mut self, rhs: &Ratio<BigInt>) {
        self.do_mul_assign_ratio(&rhs.recip());
    }
}

forward_types_to_bigint!(DivAssign, div_assign, Div, div);
forward_op_to_op_assign!(DivAssign, div_assign, Div, div, Ratio<BigInt>);

impl<E: Unsigned + Integer> Pow<E> for DyadicFractionInterval {
    type Output = DyadicFractionInterval;
    fn pow(mut self, mut exponent: E) -> DyadicFractionInterval {
        if exponent.is_zero() {
            self.set_one();
            self
        } else if exponent.is_one() {
            self
        } else {
            let contains_zero = self.contains_zero();
            let DyadicFractionInterval {
                lower_bound_numer: mut base_lower_bound_numer,
                upper_bound_numer: mut base_upper_bound_numer,
                log2_denom,
            } = self;
            let mut lower_bound_numer_is_negative = base_lower_bound_numer.is_negative();
            let mut upper_bound_numer_is_negative = base_upper_bound_numer.is_negative();
            if lower_bound_numer_is_negative {
                base_lower_bound_numer = -base_lower_bound_numer;
            }
            if upper_bound_numer_is_negative {
                base_upper_bound_numer = -base_upper_bound_numer;
            }
            let mut bounds_swapped = base_lower_bound_numer < base_upper_bound_numer;
            if bounds_swapped {
                mem::swap(&mut base_lower_bound_numer, &mut base_upper_bound_numer);
            }
            if exponent.is_even() {
                lower_bound_numer_is_negative = false;
                upper_bound_numer_is_negative = false;
                bounds_swapped = false;
                if contains_zero {
                    base_lower_bound_numer.set_zero();
                }
            }
            let mut retval_upper_bound_numer = BigInt::one() << log2_denom;
            let mut retval_lower_bound_numer = retval_upper_bound_numer.clone();
            let mut neg_retval_upper_bound_numer = -retval_upper_bound_numer;
            loop {
                if exponent.is_odd() {
                    retval_lower_bound_numer *= &base_lower_bound_numer;
                    retval_lower_bound_numer >>= log2_denom;
                    neg_retval_upper_bound_numer *= &base_upper_bound_numer;
                    neg_retval_upper_bound_numer >>= log2_denom;
                }
                let two = E::one() + E::one();
                exponent = exponent / two;
                if exponent.is_zero() {
                    break;
                }
                base_lower_bound_numer = &base_lower_bound_numer * &base_lower_bound_numer;
                base_lower_bound_numer >>= log2_denom;
                base_upper_bound_numer = -&base_upper_bound_numer * &base_upper_bound_numer;
                base_lower_bound_numer >>= log2_denom;
                base_upper_bound_numer = -base_upper_bound_numer;
            }
            retval_upper_bound_numer = -neg_retval_upper_bound_numer;
            if bounds_swapped {
                mem::swap(&mut retval_lower_bound_numer, &mut retval_upper_bound_numer);
            }
            if lower_bound_numer_is_negative {
                retval_lower_bound_numer = -retval_lower_bound_numer;
            }
            if upper_bound_numer_is_negative {
                retval_upper_bound_numer = -retval_upper_bound_numer;
            }
            DyadicFractionInterval {
                lower_bound_numer: retval_lower_bound_numer,
                upper_bound_numer: retval_upper_bound_numer,
                log2_denom,
            }
        }
    }
}

impl<E: Unsigned + Integer> Pow<E> for &'_ DyadicFractionInterval {
    type Output = DyadicFractionInterval;
    fn pow(self, exponent: E) -> DyadicFractionInterval {
        self.clone().pow(exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    type DFI = DyadicFractionInterval;

    macro_rules! assert_same {
        ($a:expr, $b:expr) => {
            let a = $a;
            let b = $b;
            let a = a.borrow();
            let b = b.borrow();
            let is_same = a.lower_bound_numer == b.lower_bound_numer
                && a.upper_bound_numer == b.upper_bound_numer
                && a.log2_denom == b.log2_denom;
            assert!(is_same, "{:?} != {:?}", a, b);
        };
        ($a:expr, $b:expr,) => {
            assert_same!($a, $b)
        };
        ($a:expr, $b:expr, $($msg:tt)+) => {
            let a = $a;
            let b = $b;
            let a = a.borrow();
            let b = b.borrow();
            let is_same = a.lower_bound_numer == b.lower_bound_numer
                && a.upper_bound_numer == b.upper_bound_numer
                && a.log2_denom == b.log2_denom;
            assert!(is_same, "{:?} != {:?}: {}", a, b, format_args!($($msg)+));
        };
    }

    fn r(n: i128, d: i128) -> Ratio<BigInt> {
        Ratio::new(n.into(), d.into())
    }

    fn ri(v: i128) -> Ratio<BigInt> {
        bi(v).into()
    }

    fn bi(v: i128) -> BigInt {
        v.into()
    }

    #[test]
    fn test_from_ratio_range() {
        assert_same!(
            DFI::from_ratio_range(r(2, 3), r(5, 7), 8),
            DFI::new(bi(170), bi(183), 8)
        );
        assert_same!(
            DFI::from_ratio_range(ri(-1), r(-5, 7), 8),
            DFI::new(bi(-256), bi(-182), 8)
        );
        assert_same!(
            DFI::from_ratio_range(r(5, 32), r(45, 32), 5),
            DFI::new(bi(5), bi(45), 5)
        );
        assert_same!(
            DFI::from_ratio_range(r(7, 32), r(8, 32), 5),
            DFI::new(bi(7), bi(8), 5)
        );
    }

    #[test]
    fn test_from_ratio() {
        assert_same!(DFI::from_ratio(r(2, 3), 8), DFI::new(bi(170), bi(171), 8));
        assert_same!(
            DFI::from_ratio(r(-2, 3), 8),
            DFI::new(bi(-171), bi(-170), 8)
        );
        assert_same!(DFI::from_ratio(r(1, 8), 8), DFI::new(bi(32), bi(32), 8));
    }

    #[test]
    fn test_convert_log2_denom() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_square() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_sqrt() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_debug() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_display() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_add() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_add_int() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_add_ratio() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_sub() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_sub_int() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_sub_ratio() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_mul() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_mul_int() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_mul_ratio() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_div_int() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_div_ratio() {
        unimplemented!("add more test cases");
    }

    #[test]
    fn test_pow() {
        unimplemented!("add more test cases");
    }
}
