// SPDX-License-Identifier: LGPL-2.1-or-later
// See Notices.txt for copyright information

use crate::polynomial::DivisorIsOne;
use crate::polynomial::PolynomialCoefficient;
use crate::polynomial::PolynomialReducingFactorSupported;
use crate::traits::AlwaysExactDiv;
use crate::traits::AlwaysExactDivAssign;
use crate::traits::ExactDiv;
use crate::traits::ExactDivAssign;
use crate::traits::ExtendedGCD;
use crate::traits::ExtendedGCDResult;
use crate::traits::GCD;
use crate::util::BaseAndExponent;
use num_bigint::BigInt;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::CheckedAdd;
use num_traits::CheckedDiv;
use num_traits::CheckedMul;
use num_traits::CheckedSub;
use num_traits::FromPrimitive;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::convert::identity;
use std::convert::TryInto;
use std::fmt;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait ModularReduce: Clone + Eq {
    fn modular_reduce_assign<M: Modulus<Value = Self>>(&mut self, modulus: M);
    fn modular_reduce<M: Modulus<Value = Self>>(mut self, modulus: M) -> Self {
        self.modular_reduce_assign(modulus);
        self
    }
    fn modular_add_ref_assign<M: Modulus<Value = Self>>(&mut self, rhs: &Self, modulus: M);
    fn modular_add_move_assign<M: Modulus<Value = Self>>(&mut self, rhs: Self, modulus: M) {
        self.modular_add_ref_assign(&rhs, modulus);
    }
    fn modular_add_ref_ref<M: Modulus<Value = Self>>(&self, rhs: &Self, modulus: M) -> Self {
        self.clone().modular_add_move_ref(rhs, modulus)
    }
    fn modular_add_move_ref<M: Modulus<Value = Self>>(mut self, rhs: &Self, modulus: M) -> Self {
        self.modular_add_ref_assign(rhs, modulus);
        self
    }
    fn modular_add_ref_move<M: Modulus<Value = Self>>(&self, rhs: Self, modulus: M) -> Self {
        self.clone().modular_add_move_move(rhs, modulus)
    }
    fn modular_add_move_move<M: Modulus<Value = Self>>(mut self, rhs: Self, modulus: M) -> Self {
        self.modular_add_move_assign(rhs, modulus);
        self
    }
    fn modular_neg_assign<M: Modulus<Value = Self>>(&mut self, modulus: M);
    fn modular_neg_ref<M: Modulus<Value = Self>>(&self, modulus: M) -> Self {
        self.clone().modular_neg_move(modulus)
    }
    fn modular_neg_move<M: Modulus<Value = Self>>(mut self, modulus: M) -> Self {
        self.modular_neg_assign(modulus);
        self
    }
    fn modular_sub_ref_assign<M: Modulus<Value = Self>>(&mut self, rhs: &Self, modulus: M) {
        self.modular_add_move_assign(rhs.modular_neg_ref(&modulus), modulus);
    }
    fn modular_sub_move_assign<M: Modulus<Value = Self>>(&mut self, rhs: Self, modulus: M) {
        self.modular_sub_ref_assign(&rhs, modulus);
    }
    fn modular_sub_ref_ref<M: Modulus<Value = Self>>(&self, rhs: &Self, modulus: M) -> Self {
        self.clone().modular_sub_move_ref(rhs, modulus)
    }
    fn modular_sub_move_ref<M: Modulus<Value = Self>>(mut self, rhs: &Self, modulus: M) -> Self {
        self.modular_sub_ref_assign(rhs, modulus);
        self
    }
    fn modular_sub_ref_move<M: Modulus<Value = Self>>(&self, rhs: Self, modulus: M) -> Self {
        self.clone().modular_sub_move_move(rhs, modulus)
    }
    fn modular_sub_move_move<M: Modulus<Value = Self>>(mut self, rhs: Self, modulus: M) -> Self {
        self.modular_sub_move_assign(rhs, modulus);
        self
    }
    fn modular_mul_ref_assign<M: Modulus<Value = Self>>(&mut self, rhs: &Self, modulus: M);
    fn modular_mul_move_assign<M: Modulus<Value = Self>>(&mut self, rhs: Self, modulus: M) {
        self.modular_mul_ref_assign(&rhs, modulus);
    }
    fn modular_mul_ref_ref<M: Modulus<Value = Self>>(&self, rhs: &Self, modulus: M) -> Self {
        self.clone().modular_mul_move_ref(rhs, modulus)
    }
    fn modular_mul_move_ref<M: Modulus<Value = Self>>(mut self, rhs: &Self, modulus: M) -> Self {
        self.modular_mul_ref_assign(rhs, modulus);
        self
    }
    fn modular_mul_ref_move<M: Modulus<Value = Self>>(&self, rhs: Self, modulus: M) -> Self {
        self.clone().modular_mul_move_move(rhs, modulus)
    }
    fn modular_mul_move_move<M: Modulus<Value = Self>>(mut self, rhs: Self, modulus: M) -> Self {
        self.modular_mul_move_assign(rhs, modulus);
        self
    }
    fn modular_reduce_from_bigint<M: Modulus<Value = Self>>(v: BigInt, modulus: M) -> Self;
    fn modular_reduce_from_biguint<M: Modulus<Value = Self>>(v: BigUint, modulus: M) -> Self {
        Self::modular_reduce_from_bigint(v.into(), modulus)
    }
    fn modular_reduce_from_u8<M: Modulus<Value = Self>>(v: u8, modulus: M) -> Self {
        Self::modular_reduce_from_u16(v.into(), modulus)
    }
    fn modular_reduce_from_u16<M: Modulus<Value = Self>>(v: u16, modulus: M) -> Self {
        Self::modular_reduce_from_u32(v.into(), modulus)
    }
    fn modular_reduce_from_u32<M: Modulus<Value = Self>>(v: u32, modulus: M) -> Self {
        Self::modular_reduce_from_u64(v.into(), modulus)
    }
    fn modular_reduce_from_u64<M: Modulus<Value = Self>>(v: u64, modulus: M) -> Self {
        Self::modular_reduce_from_u128(v.into(), modulus)
    }
    fn modular_reduce_from_u128<M: Modulus<Value = Self>>(v: u128, modulus: M) -> Self {
        Self::modular_reduce_from_biguint(v.into(), modulus)
    }
    fn modular_reduce_from_usize<M: Modulus<Value = Self>>(v: usize, modulus: M) -> Self {
        Self::modular_reduce_from_u128(v as _, modulus)
    }
    fn modular_reduce_from_i8<M: Modulus<Value = Self>>(v: i8, modulus: M) -> Self {
        Self::modular_reduce_from_i16(v.into(), modulus)
    }
    fn modular_reduce_from_i16<M: Modulus<Value = Self>>(v: i16, modulus: M) -> Self {
        Self::modular_reduce_from_i32(v.into(), modulus)
    }
    fn modular_reduce_from_i32<M: Modulus<Value = Self>>(v: i32, modulus: M) -> Self {
        Self::modular_reduce_from_i64(v.into(), modulus)
    }
    fn modular_reduce_from_i64<M: Modulus<Value = Self>>(v: i64, modulus: M) -> Self {
        Self::modular_reduce_from_i128(v.into(), modulus)
    }
    fn modular_reduce_from_i128<M: Modulus<Value = Self>>(v: i128, modulus: M) -> Self {
        Self::modular_reduce_from_bigint(v.into(), modulus)
    }
    fn modular_reduce_from_isize<M: Modulus<Value = Self>>(v: isize, modulus: M) -> Self {
        Self::modular_reduce_from_i128(v as _, modulus)
    }
}

pub trait ModularReducePow<E = Self>: ModularReduce {
    fn pow_modular_reduce<M: Modulus<Value = Self>>(&self, exponent: &E, modulus: M) -> Self;
}

pub trait Modulus: Clone + Eq {
    type Value: Clone + Eq;
    fn to_modulus(&self) -> &Self::Value;
    fn into_modulus(self) -> Self::Value {
        self.to_modulus().clone()
    }
}

impl<T: Modulus> Modulus for &'_ T {
    type Value = T::Value;
    fn to_modulus(&self) -> &Self::Value {
        (**self).to_modulus()
    }
}

pub trait StaticModulus: Modulus + 'static + Copy + Default {
    fn get_modulus() -> Self::Value;
}

pub trait PrimePowerModulus: Modulus
where
    <Self as Modulus>::Value: Integer + Clone,
{
    fn base_and_exponent(&self) -> BaseAndExponent<<Self as Modulus>::Value>;
}

pub trait PrimeModulus: PrimePowerModulus
where
    <Self as Modulus>::Value: Integer + Clone,
{
}

macro_rules! impl_int_modulus {
    ($t:ty, $wide:ty, $to_wide:expr, $from_wide:expr, $from_bigint:ident) => {
        impl Modulus for $t {
            type Value = Self;
            fn to_modulus(&self) -> &Self::Value {
                self
            }
            fn into_modulus(self) -> Self::Value {
                self
            }
        }

        impl ModularReduce for $t {
            fn modular_reduce_assign<M: Modulus<Value = Self>>(&mut self, modulus: M) {
                let modulus = modulus.to_modulus();
                if !modulus.is_zero() {
                    *self = self.mod_floor(modulus);
                }
            }
            fn modular_reduce<M: Modulus<Value = Self>>(self, modulus: M) -> Self {
                let modulus = modulus.to_modulus();
                if !modulus.is_zero() {
                    self.mod_floor(modulus)
                } else {
                    self
                }
            }
            fn modular_add_ref_assign<M: Modulus<Value = Self>>(&mut self, rhs: &Self, modulus: M) {
                let mut wide = $to_wide(self.clone());
                wide += $to_wide(rhs.clone());
                wide %= $to_wide(modulus.to_modulus().clone());
                *self = $from_wide(wide);
            }
            fn modular_neg_assign<M: Modulus<Value = Self>>(&mut self, modulus: M) {
                let modulus = modulus.to_modulus();
                *self = modulus.to_modulus() - self.clone();
                if *self == *modulus {
                    self.set_zero();
                }
            }
            fn modular_mul_ref_assign<M: Modulus<Value = Self>>(&mut self, rhs: &Self, modulus: M) {
                let mut wide = $to_wide(self.clone());
                wide *= $to_wide(rhs.clone());
                wide %= $to_wide(modulus.to_modulus().clone());
                *self = $from_wide(wide);
            }
            fn modular_reduce_from_bigint<M: Modulus<Value = Self>>(v: BigInt, modulus: M) -> Self {
                v.mod_floor(&modulus.into_modulus().into())
                    .$from_bigint()
                    .expect(concat!(stringify!($from_bigint), " failed"))
            }
        }
    };
}

macro_rules! impl_prim_int_modulus {
    ($t:ty, $wide:ty, $to_wide:expr, $from_wide:expr, $from_bigint:ident) => {
        impl_int_modulus!($t, $wide, $to_wide, $from_wide, $from_bigint);

        impl<E: Integer + Clone + FromPrimitive> ModularReducePow<E> for $t {
            fn pow_modular_reduce<M: Modulus<Value = Self>>(
                &self,
                exponent: &E,
                modulus: M,
            ) -> Self {
                if exponent.is_zero() {
                    return One::one();
                }
                let modulus = *modulus.to_modulus();
                let mut base = *self;
                base.modular_reduce_assign(modulus);
                if exponent.is_one() {
                    return base;
                }
                let mut exponent = exponent.clone();
                let mut retval = None;
                loop {
                    if exponent.is_odd() {
                        match &mut retval {
                            None => retval = Some(base.clone()),
                            Some(retval) => {
                                retval.modular_mul_move_assign(base, modulus);
                            }
                        }
                    }
                    exponent = exponent / E::from_u8(2).expect("2 doesn't fit in exponent type");
                    if exponent.is_zero() {
                        break;
                    }
                    base.modular_mul_move_assign(base, modulus);
                }
                retval.unwrap_or_else(|| unreachable!())
            }
        }
    };
}

macro_rules! impl_bigint_modulus {
    ($t:ty, $from_bigint:ident) => {
        impl_int_modulus!($t, $t, identity, identity, $from_bigint);

        impl<E: Clone + Into<$t>> ModularReducePow<E> for $t {
            fn pow_modular_reduce<M: Modulus<Value = Self>>(
                &self,
                exponent: &E,
                modulus: M,
            ) -> Self {
                self.modpow(&exponent.clone().into(), modulus.to_modulus())
            }
        }
    };
}

fn convert_to<I: TryInto<O>, O>(v: I) -> O
where
    I::Error: fmt::Debug,
{
    v.try_into().unwrap()
}

fn convert_to_i128(v: BigInt) -> i128 {
    v.to_i128().expect("can't convert to i128")
}

fn convert_to_u128(v: BigUint) -> u128 {
    v.to_u128().expect("can't convert to u128")
}

/// helper trait for impl_bigint_modulus!
trait BigIntToOptionBigInt: Sized {
    fn bigint_to_option_bigint(self) -> Option<Self> {
        Some(self)
    }
}

impl BigIntToOptionBigInt for BigInt {}

impl_prim_int_modulus!(i8, i16, i16::from, convert_to, to_i8);
impl_prim_int_modulus!(u8, u16, u16::from, convert_to, to_u8);
impl_prim_int_modulus!(i16, i32, i32::from, convert_to, to_i16);
impl_prim_int_modulus!(u16, u32, u32::from, convert_to, to_u16);
impl_prim_int_modulus!(i32, i64, i64::from, convert_to, to_i32);
impl_prim_int_modulus!(u32, u64, u64::from, convert_to, to_u32);
impl_prim_int_modulus!(i64, i128, i128::from, convert_to, to_i64);
impl_prim_int_modulus!(u64, u128, u128::from, convert_to, to_u64);
impl_prim_int_modulus!(i128, BigInt, BigInt::from, convert_to_i128, to_i128);
impl_prim_int_modulus!(u128, BigUint, BigUint::from, convert_to_u128, to_u128);
impl_prim_int_modulus!(isize, i128, convert_to::<isize, i128>, convert_to, to_isize);
impl_prim_int_modulus!(usize, u128, convert_to::<usize, u128>, convert_to, to_usize);
impl_bigint_modulus!(BigInt, bigint_to_option_bigint);
impl_bigint_modulus!(BigUint, to_biguint);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModularInteger<V, M> {
    value: V,
    modulus: M,
}

impl<V, M> ModularInteger<V, M> {
    pub fn value(&self) -> &V {
        &self.value
    }
    pub fn modulus(&self) -> &M {
        &self.modulus
    }
}

impl<V, M: PartialEq> ModularInteger<V, M> {
    pub fn has_matching_moduli(&self, rhs: &Self) -> bool {
        self.modulus == rhs.modulus
    }
    fn require_matching_moduli(&self, rhs: &Self) {
        assert!(self.has_matching_moduli(rhs), "moduli don't match");
    }
}

impl<V, M> Into<(V, M)> for ModularInteger<V, M> {
    fn into(self) -> (V, M) {
        (self.value, self.modulus)
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> ModularInteger<V, M> {
    pub fn new<T: Into<V>>(value: T, modulus: M) -> Self {
        let value = value.into().modular_reduce(&modulus);
        Self { value, modulus }
    }
    /// `*self = -*self`
    pub fn neg_assign(&mut self) {
        V::modular_neg_assign(&mut self.value, &self.modulus);
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> From<(V, M)> for ModularInteger<V, M> {
    fn from((value, modulus): (V, M)) -> Self {
        Self::new(value, modulus)
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> Add for ModularInteger<V, M> {
    type Output = ModularInteger<V, M>;
    fn add(self, rhs: Self) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_add_move_move(rhs.value, &self.modulus),
            modulus: self.modulus,
        }
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> AddAssign for ModularInteger<V, M> {
    fn add_assign(&mut self, rhs: Self) {
        self.require_matching_moduli(&rhs);
        self.value.modular_add_move_assign(rhs.value, &self.modulus);
    }
}

impl<'r, V: ModularReduce + Eq, M: Modulus<Value = V>> AddAssign<&'r ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    fn add_assign(&mut self, rhs: &Self) {
        self.require_matching_moduli(&rhs);
        self.value.modular_add_ref_assign(&rhs.value, &self.modulus);
    }
}

impl<'r, V: ModularReduce + Eq, M: Modulus<Value = V>> Add<&'r ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn add(self, rhs: &ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(rhs);
        ModularInteger {
            value: self.value.modular_add_move_ref(&rhs.value, &self.modulus),
            modulus: self.modulus,
        }
    }
}

impl<'l, V: ModularReduce + Eq, M: Modulus<Value = V>> Add<ModularInteger<V, M>>
    for &'l ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn add(self, rhs: ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_add_ref_move(rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        }
    }
}

impl<'l, 'r, V: ModularReduce + Eq, M: Modulus<Value = V>> Add<&'r ModularInteger<V, M>>
    for &'l ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn add(self, rhs: &ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(rhs);
        ModularInteger {
            value: self.value.modular_add_ref_ref(&rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        }
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> CheckedAdd for ModularInteger<V, M> {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        if !self.has_matching_moduli(rhs) {
            return None;
        }
        Some(ModularInteger {
            value: self.value.modular_add_ref_ref(&rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        })
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> Neg for ModularInteger<V, M> {
    type Output = ModularInteger<V, M>;
    fn neg(mut self) -> Self::Output {
        self.value = self.value.modular_neg_move(&self.modulus);
        self
    }
}

impl<'a, V: ModularReduce + Eq, M: Modulus<Value = V>> Neg for &'a ModularInteger<V, M> {
    type Output = ModularInteger<V, M>;
    fn neg(self) -> Self::Output {
        let value = self.value.modular_neg_ref(&self.modulus);
        ModularInteger {
            value,
            modulus: self.modulus.clone(),
        }
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> Sub<ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn sub(self, rhs: ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_sub_move_move(rhs.value, &self.modulus),
            modulus: self.modulus,
        }
    }
}

impl<'r, V: ModularReduce + Eq, M: Modulus<Value = V>> Sub<&'r ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn sub(self, rhs: &ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_sub_move_ref(&rhs.value, &self.modulus),
            modulus: self.modulus,
        }
    }
}

impl<'l, V: ModularReduce + Eq, M: Modulus<Value = V>> Sub<ModularInteger<V, M>>
    for &'l ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn sub(self, rhs: ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_sub_ref_move(rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        }
    }
}

impl<'l, 'r, V: ModularReduce + Eq, M: Modulus<Value = V>> Sub<&'r ModularInteger<V, M>>
    for &'l ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn sub(self, rhs: &ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_sub_ref_ref(&rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        }
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> SubAssign<ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.require_matching_moduli(&rhs);
        self.value.modular_sub_move_assign(rhs.value, &self.modulus);
    }
}

impl<'r, V: ModularReduce + Eq, M: Modulus<Value = V>> SubAssign<&'r ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.require_matching_moduli(&rhs);
        self.value.modular_sub_ref_assign(&rhs.value, &self.modulus);
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> CheckedSub for ModularInteger<V, M> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        if !self.has_matching_moduli(rhs) {
            return None;
        }
        Some(ModularInteger {
            value: self.value.modular_sub_ref_ref(&rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        })
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> Mul for ModularInteger<V, M> {
    type Output = ModularInteger<V, M>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_mul_move_move(rhs.value, &self.modulus),
            modulus: self.modulus,
        }
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> MulAssign for ModularInteger<V, M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.require_matching_moduli(&rhs);
        self.value.modular_mul_move_assign(rhs.value, &self.modulus);
    }
}

impl<'r, V: ModularReduce + Eq, M: Modulus<Value = V>> MulAssign<&'r ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.require_matching_moduli(&rhs);
        self.value.modular_mul_ref_assign(&rhs.value, &self.modulus);
    }
}

impl<'r, V: ModularReduce + Eq, M: Modulus<Value = V>> Mul<&'r ModularInteger<V, M>>
    for ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn mul(self, rhs: &ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_mul_move_ref(&rhs.value, &self.modulus),
            modulus: self.modulus,
        }
    }
}

impl<'l, V: ModularReduce + Eq, M: Modulus<Value = V>> Mul<ModularInteger<V, M>>
    for &'l ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn mul(self, rhs: ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_mul_ref_move(rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        }
    }
}

impl<'l, 'r, V: ModularReduce + Eq, M: Modulus<Value = V>> Mul<&'r ModularInteger<V, M>>
    for &'l ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn mul(self, rhs: &ModularInteger<V, M>) -> Self::Output {
        self.require_matching_moduli(&rhs);
        ModularInteger {
            value: self.value.modular_mul_ref_ref(&rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        }
    }
}

impl<V: ModularReduce + Eq, M: Modulus<Value = V>> CheckedMul for ModularInteger<V, M> {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        if !self.has_matching_moduli(rhs) {
            return None;
        }
        Some(ModularInteger {
            value: self.value.modular_mul_ref_ref(&rhs.value, &self.modulus),
            modulus: self.modulus.clone(),
        })
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>>
    ModularInteger<V, M>
{
    pub fn checked_inverse(&self) -> Option<Self> {
        if self.value.is_zero() {
            return None;
        }
        let ExtendedGCDResult { gcd, x, .. } = self.value.extended_gcd(self.modulus.to_modulus());
        if gcd.is_one() {
            Some(ModularInteger::new(x, self.modulus.clone()))
        } else {
            None
        }
    }
    pub fn inverse(&self) -> Self {
        self.checked_inverse()
            .expect("value has no modular inverse")
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>>
    DivAssign for ModularInteger<V, M>
{
    fn div_assign(&mut self, rhs: ModularInteger<V, M>) {
        self.mul_assign(rhs.inverse())
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>>
    DivAssign<&'_ ModularInteger<V, M>> for ModularInteger<V, M>
{
    fn div_assign(&mut self, rhs: &ModularInteger<V, M>) {
        self.mul_assign(rhs.inverse())
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>> Div
    for ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn div(self, rhs: ModularInteger<V, M>) -> ModularInteger<V, M> {
        self.mul(rhs.inverse())
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>>
    Div<ModularInteger<V, M>> for &'_ ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn div(self, rhs: ModularInteger<V, M>) -> ModularInteger<V, M> {
        self.mul(&rhs.inverse())
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>>
    Div<&'_ ModularInteger<V, M>> for ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn div(self, rhs: &ModularInteger<V, M>) -> ModularInteger<V, M> {
        self.mul(rhs.inverse())
    }
}

impl<
        'a,
        'b,
        V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD,
        M: Modulus<Value = V>,
    > Div<&'a ModularInteger<V, M>> for &'b ModularInteger<V, M>
{
    type Output = ModularInteger<V, M>;
    fn div(self, rhs: &ModularInteger<V, M>) -> ModularInteger<V, M> {
        self.mul(&rhs.inverse())
    }
}

impl<V: ModularReduce + Eq + One + Zero + GCD<Output = V> + ExtendedGCD, M: Modulus<Value = V>>
    CheckedDiv for ModularInteger<V, M>
{
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        self.checked_mul(&rhs.checked_inverse()?)
    }
}

macro_rules! impl_exact_div {
    (($($lifetimes:tt),*), $v:ident, $m:ident, $lhs:ty, $rhs:ty) => {
        impl<$($lifetimes,)* $v, $m> ExactDiv<$rhs> for $lhs
        where
            $v: ModularReduce + Eq + One + Zero + GCD<Output = $v> + ExtendedGCD, $m: Modulus<Value = $v>
        {
            type Output = ModularInteger<$v, $m>;
            fn exact_div(self, rhs: $rhs) -> Self::Output {
                self.div(rhs)
            }
            fn checked_exact_div(self, rhs: $rhs) -> Option<Self::Output> {
                self.checked_div(rhs.borrow())
            }
        }

        impl<$($lifetimes,)* $v, $m> AlwaysExactDiv<$rhs> for $lhs
        where
            $v: ModularReduce + Integer + GCD<Output = $v> + ExtendedGCD, $m: Modulus<Value = $v> + PrimeModulus
        {
        }
    };
    (assign ($($lifetimes:tt),*), $v:ident, $m:ident, $lhs:ty, $rhs:ty) => {
        impl_exact_div!(($($lifetimes),*), $v, $m, $lhs, $rhs);

        impl<$($lifetimes,)* $v, $m> ExactDivAssign<$rhs> for $lhs
        where
            $v: ModularReduce + Eq + One + Zero + GCD<Output = $v> + ExtendedGCD, $m: Modulus<Value = $v>
        {
            fn exact_div_assign(&mut self, rhs: $rhs) {
                self.div_assign(rhs);
            }
            fn checked_exact_div_assign(&mut self, rhs: $rhs) -> Result<(), ()> {
                (&*self)
                    .checked_exact_div(rhs)
                    .map(|v| {
                        *self = v;
                    })
                    .ok_or(())
            }
        }

        impl<$($lifetimes,)* $v, $m> AlwaysExactDivAssign<$rhs> for $lhs
        where
            $v: ModularReduce + Integer + GCD<Output = $v> + ExtendedGCD, $m: Modulus<Value = $v> + PrimeModulus
        {
        }
    };
}

impl_exact_div!(assign (), V, M, ModularInteger<V, M>, ModularInteger<V, M>);
impl_exact_div!(assign ('r), V, M, ModularInteger<V, M>, &'r ModularInteger<V, M>);
impl_exact_div!(('l), V, M, &'l ModularInteger<V, M>, ModularInteger<V, M>);
impl_exact_div!(('l, 'r), V, M, &'l ModularInteger<V, M>, &'r ModularInteger<V, M>);

impl<V, M> PolynomialCoefficient for ModularInteger<V, M>
where
    V: ModularReducePow<usize> + Eq + One + Zero + fmt::Debug + Hash,
    M: Modulus<Value = V> + fmt::Debug + Hash,
{
    type Element = Self;
    type Divisor = DivisorIsOne;
    fn is_element_zero(element: &Self::Element) -> bool {
        element.value.is_zero()
    }
    fn is_element_one(element: &Self::Element) -> bool {
        element.value.is_one()
    }
    fn is_coefficient_zero(coefficient: &Self) -> bool {
        coefficient.value.is_zero()
    }
    fn is_coefficient_one(coefficient: &Self) -> bool {
        coefficient.value.is_one()
    }
    fn set_element_zero(element: &mut Self::Element) {
        element.value.set_zero();
    }
    fn set_element_one(element: &mut Self::Element) {
        element.value = V::modular_reduce(V::one(), &element.modulus);
    }
    fn set_coefficient_zero(coefficient: &mut Self) {
        Self::set_element_zero(coefficient);
    }
    fn set_coefficient_one(coefficient: &mut Self) {
        Self::set_element_one(coefficient);
    }
    fn make_zero_element(element: Cow<Self::Element>) -> Self::Element {
        let modulus = match element {
            Cow::Borrowed(element) => element.modulus.clone(),
            Cow::Owned(element) => element.modulus,
        };
        Self::new(V::zero(), modulus)
    }
    fn make_one_element(element: Cow<Self::Element>) -> Self::Element {
        let modulus = match element {
            Cow::Borrowed(element) => element.modulus.clone(),
            Cow::Owned(element) => element.modulus,
        };
        Self::new(V::one(), modulus)
    }
    fn make_zero_coefficient_from_element(element: Cow<Self::Element>) -> Self {
        Self::make_zero_element(element)
    }
    fn make_one_coefficient_from_element(element: Cow<Self::Element>) -> Self {
        Self::make_one_element(element)
    }
    fn make_zero_coefficient_from_coefficient(coefficient: Cow<Self>) -> Self {
        Self::make_zero_element(coefficient)
    }
    fn make_one_coefficient_from_coefficient(coefficient: Cow<Self>) -> Self {
        Self::make_one_element(coefficient)
    }
    fn negate_element(element: &mut Self::Element) {
        element.neg_assign();
    }
    fn mul_element_by_usize(element: Cow<Self::Element>, multiplier: usize) -> Self::Element {
        let mut element = element.into_owned();
        Self::mul_assign_element_by_usize(&mut element, multiplier);
        element
    }
    fn mul_assign_element_by_usize(element: &mut Self::Element, multiplier: usize) {
        element.value.modular_mul_move_assign(
            V::modular_reduce_from_usize(multiplier, &element.modulus),
            &element.modulus,
        );
    }
    fn divisor_to_element(
        _v: Cow<Self::Divisor>,
        other_element: Cow<Self::Element>,
    ) -> Self::Element {
        Self::make_one_element(other_element)
    }
    fn coefficients_to_elements(coefficients: Cow<[Self]>) -> (Vec<Self::Element>, Self::Divisor) {
        (coefficients.into_owned(), One::one())
    }
    fn make_coefficient(element: Cow<Self::Element>, _divisor: Cow<Self::Divisor>) -> Self {
        element.into_owned()
    }
    fn reduce_divisor(_elements: &mut [Self::Element], _divisor: &mut Self::Divisor) {}
    fn get_reduced_divisor(
        elements: &[Self::Element],
        _divisor: &Self::Divisor,
    ) -> (Vec<Self::Element>, Self::Divisor) {
        (elements.to_vec(), One::one())
    }
    fn coefficient_to_element(coefficient: Cow<Self>) -> (Self::Element, Self::Divisor) {
        (coefficient.into_owned(), One::one())
    }
    fn divisor_pow_usize(_base: Self::Divisor, _exponent: usize) -> Self::Divisor {
        One::one()
    }
    fn element_pow_usize(base: Self::Element, exponent: usize) -> Self::Element {
        let ModularInteger { value, modulus } = base;
        let value = value.pow_modular_reduce(&exponent, &modulus);
        ModularInteger { value, modulus }
    }
}

impl<V, M> PolynomialReducingFactorSupported for ModularInteger<V, M>
where
    V: ModularReducePow<usize> + Integer + fmt::Debug + Hash,
    M: Modulus<Value = V> + PrimeModulus + fmt::Debug + Hash,
{
    fn get_nonzero_reducing_factor(
        elements: &[Self::Element],
        _divisor: &Self::Divisor,
    ) -> Option<Self> {
        Some(elements.last()?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tests::test_op_helper;
    use crate::util::tests::test_unary_op_helper;

    fn test_overflow_for_type<
        T: Modulus<Value = T> + ModularReduce + Sub<Output = T> + Copy + Into<BigInt> + fmt::Debug,
        BigIntToT: Fn(&BigInt) -> Option<T>,
    >(
        modulus: T,
        big_int_to_t: BigIntToT,
    ) where
        i32: TryInto<T>,
        <i32 as TryInto<T>>::Error: fmt::Debug,
    {
        let index_to_t = |index: i32| -> T {
            if index < 0 {
                modulus - (-index).try_into().unwrap()
            } else {
                index.try_into().unwrap()
            }
        };
        for a in -5..=5 {
            for b in -5..=5 {
                let lhs = index_to_t(a);
                let rhs = index_to_t(b);
                let big_modulus: BigInt = modulus.into();
                let big_lhs: BigInt = lhs.into();
                let big_rhs: BigInt = rhs.into();
                let add_result = big_int_to_t(&((&big_lhs + &big_rhs) % &big_modulus))
                    .expect("can't convert BigInt to T");
                let neg_result = big_int_to_t(&((&big_modulus - &big_rhs) % &big_modulus))
                    .expect("can't convert BigInt to T");
                let sub_result =
                    big_int_to_t(&((&big_lhs + &big_modulus - &big_rhs) % &big_modulus))
                        .expect("can't convert BigInt to T");
                let mul_result = big_int_to_t(&((&big_lhs * &big_rhs) % &big_modulus))
                    .expect("can't convert BigInt to T");
                test_op_helper(
                    ModularInteger::<T, T>::new(lhs, modulus),
                    ModularInteger::new(rhs, modulus),
                    &ModularInteger::new(add_result, modulus),
                    |l, r| *l += r,
                    |l, r| *l += r,
                    |l, r| l + r,
                    |l, r| l + r,
                    |l, r| l + r,
                    |l, r| l + r,
                );
                test_unary_op_helper(
                    ModularInteger::<T, T>::new(rhs, modulus),
                    &ModularInteger::new(neg_result, modulus),
                    |v| -v,
                    |v| -v,
                );
                test_op_helper(
                    ModularInteger::<T, T>::new(lhs, modulus),
                    ModularInteger::new(rhs, modulus),
                    &ModularInteger::new(sub_result, modulus),
                    |l, r| *l -= r,
                    |l, r| *l -= r,
                    |l, r| l - r,
                    |l, r| l - r,
                    |l, r| l - r,
                    |l, r| l - r,
                );
                test_op_helper(
                    ModularInteger::<T, T>::new(lhs, modulus),
                    ModularInteger::new(rhs, modulus),
                    &ModularInteger::new(mul_result, modulus),
                    |l, r| *l *= r,
                    |l, r| *l *= r,
                    |l, r| l * r,
                    |l, r| l * r,
                    |l, r| l * r,
                    |l, r| l * r,
                );
            }
        }
    }

    #[test]
    fn test_overflow() {
        // arguments are biggest prime that fits in the corresponding type
        test_overflow_for_type(251u8, BigInt::to_u8);
        test_overflow_for_type(127i8, BigInt::to_i8);
        test_overflow_for_type(65521u16, BigInt::to_u16);
        test_overflow_for_type(32749i16, BigInt::to_i16);
        test_overflow_for_type(4_294_967_291u32, BigInt::to_u32);
        test_overflow_for_type(2_147_483_647i32, BigInt::to_i32);
        test_overflow_for_type(18_446_744_073_709_551_557u64, BigInt::to_u64);
        test_overflow_for_type(9_223_372_036_854_775_783i64, BigInt::to_i64);
        test_overflow_for_type(
            340_282_366_920_938_463_463_374_607_431_768_211_297u128,
            BigInt::to_u128,
        );
        test_overflow_for_type(
            170_141_183_460_469_231_731_687_303_715_884_105_727i128,
            BigInt::to_i128,
        );
    }

    #[test]
    fn test_add() {
        let test = |l: ModularInteger<i32, i32>,
                    r: ModularInteger<i32, i32>,
                    expected: &ModularInteger<i32, i32>| {
            test_op_helper(
                l,
                r,
                expected,
                |l, r| *l += r,
                |l, r| *l += r,
                |l, r| l + r,
                |l, r| l + r,
                |l, r| l + r,
                |l, r| l + r,
            );
        };

        for m in 0..10 {
            for a in 0..m {
                for b in 0..m {
                    let mut expected = a + b;
                    if m != 0 {
                        expected %= m;
                    }
                    test((a, m).into(), (b, m).into(), &(expected, m).into());
                }
            }
        }
    }

    #[test]
    fn test_sub() {
        let test = |l: ModularInteger<i32, i32>,
                    r: ModularInteger<i32, i32>,
                    expected: &ModularInteger<i32, i32>| {
            test_op_helper(
                l,
                r,
                expected,
                |l, r| *l -= r,
                |l, r| *l -= r,
                |l, r| l - r,
                |l, r| l - r,
                |l, r| l - r,
                |l, r| l - r,
            );
        };

        for m in 0..10 {
            for a in 0..m {
                for b in 0..m {
                    let mut expected = a - b;
                    if m != 0 {
                        expected %= m;
                        if expected < 0 {
                            expected += m;
                        }
                    }
                    test((a, m).into(), (b, m).into(), &(expected, m).into());
                }
            }
        }
    }

    #[test]
    fn test_mul() {
        let test = |l: ModularInteger<i32, i32>,
                    r: ModularInteger<i32, i32>,
                    expected: &ModularInteger<i32, i32>| {
            test_op_helper(
                l,
                r,
                expected,
                |l, r| *l *= r,
                |l, r| *l *= r,
                |l, r| l * r,
                |l, r| l * r,
                |l, r| l * r,
                |l, r| l * r,
            );
        };

        for m in 0..10 {
            for a in 0..m {
                for b in 0..m {
                    let mut expected = a * b;
                    if m != 0 {
                        expected %= m;
                    }
                    test((a, m).into(), (b, m).into(), &(expected, m).into());
                }
            }
        }
    }

    #[test]
    fn test_div() {
        let test = |l: i64, r: i64, expected: Option<i64>, modulus: i64| {
            dbg!((l, r, expected, modulus));
            assert_eq!(
                ModularInteger::new(l, modulus).checked_div(&ModularInteger::new(r, modulus)),
                expected.map(|expected| ModularInteger::new(expected, modulus))
            );
            let expected = match expected {
                None => return,
                Some(expected) => expected,
            };
            test_op_helper(
                ModularInteger::new(l, modulus),
                ModularInteger::new(r, modulus),
                &ModularInteger::new(expected, modulus),
                |l, r| *l /= r,
                |l, r| *l /= r,
                |l, r| l / r,
                |l, r| l / r,
                |l, r| l / r,
                |l, r| l / r,
            );
        };

        test(0, 0, None, 1);
        test(0, 0, None, 2);
        test(0, 1, Some(0), 2);
        test(1, 0, None, 2);
        test(1, 1, Some(1), 2);
        test(0, 0, None, 3);
        test(0, 1, Some(0), 3);
        test(0, 2, Some(0), 3);
        test(1, 0, None, 3);
        test(1, 1, Some(1), 3);
        test(1, 2, Some(2), 3);
        test(2, 0, None, 3);
        test(2, 1, Some(2), 3);
        test(2, 2, Some(1), 3);
        test(0, 0, None, 4);
        test(0, 1, Some(0), 4);
        test(0, 2, None, 4);
        test(0, 3, Some(0), 4);
        test(1, 0, None, 4);
        test(1, 1, Some(1), 4);
        test(1, 2, None, 4);
        test(1, 3, Some(3), 4);
        test(2, 0, None, 4);
        test(2, 1, Some(2), 4);
        test(2, 2, None, 4);
        test(2, 3, Some(2), 4);
        test(3, 0, None, 4);
        test(3, 1, Some(3), 4);
        test(3, 2, None, 4);
        test(3, 3, Some(1), 4);
        test(0, 0, None, 5);
        test(0, 1, Some(0), 5);
        test(0, 2, Some(0), 5);
        test(0, 3, Some(0), 5);
        test(0, 4, Some(0), 5);
        test(1, 0, None, 5);
        test(1, 1, Some(1), 5);
        test(1, 2, Some(3), 5);
        test(1, 3, Some(2), 5);
        test(1, 4, Some(4), 5);
        test(2, 0, None, 5);
        test(2, 1, Some(2), 5);
        test(2, 2, Some(1), 5);
        test(2, 3, Some(4), 5);
        test(2, 4, Some(3), 5);
        test(3, 0, None, 5);
        test(3, 1, Some(3), 5);
        test(3, 2, Some(4), 5);
        test(3, 3, Some(1), 5);
        test(3, 4, Some(2), 5);
        test(4, 0, None, 5);
        test(4, 1, Some(4), 5);
        test(4, 2, Some(2), 5);
        test(4, 3, Some(3), 5);
        test(4, 4, Some(1), 5);
        test(0, 0, None, 6);
        test(0, 1, Some(0), 6);
        test(0, 2, None, 6);
        test(0, 3, None, 6);
        test(0, 4, None, 6);
        test(0, 5, Some(0), 6);
        test(1, 0, None, 6);
        test(1, 1, Some(1), 6);
        test(1, 2, None, 6);
        test(1, 3, None, 6);
        test(1, 4, None, 6);
        test(1, 5, Some(5), 6);
        test(2, 0, None, 6);
        test(2, 1, Some(2), 6);
        test(2, 2, None, 6);
        test(2, 3, None, 6);
        test(2, 4, None, 6);
        test(2, 5, Some(4), 6);
        test(3, 0, None, 6);
        test(3, 1, Some(3), 6);
        test(3, 2, None, 6);
        test(3, 3, None, 6);
        test(3, 4, None, 6);
        test(3, 5, Some(3), 6);
        test(4, 0, None, 6);
        test(4, 1, Some(4), 6);
        test(4, 2, None, 6);
        test(4, 3, None, 6);
        test(4, 4, None, 6);
        test(4, 5, Some(2), 6);
        test(5, 0, None, 6);
        test(5, 1, Some(5), 6);
        test(5, 2, None, 6);
        test(5, 3, None, 6);
        test(5, 4, None, 6);
        test(5, 5, Some(1), 6);
        test(0, 0, None, 7);
        test(0, 1, Some(0), 7);
        test(0, 2, Some(0), 7);
        test(0, 3, Some(0), 7);
        test(0, 4, Some(0), 7);
        test(0, 5, Some(0), 7);
        test(0, 6, Some(0), 7);
        test(1, 0, None, 7);
        test(1, 1, Some(1), 7);
        test(1, 2, Some(4), 7);
        test(1, 3, Some(5), 7);
        test(1, 4, Some(2), 7);
        test(1, 5, Some(3), 7);
        test(1, 6, Some(6), 7);
        test(2, 0, None, 7);
        test(2, 1, Some(2), 7);
        test(2, 2, Some(1), 7);
        test(2, 3, Some(3), 7);
        test(2, 4, Some(4), 7);
        test(2, 5, Some(6), 7);
        test(2, 6, Some(5), 7);
        test(3, 0, None, 7);
        test(3, 1, Some(3), 7);
        test(3, 2, Some(5), 7);
        test(3, 3, Some(1), 7);
        test(3, 4, Some(6), 7);
        test(3, 5, Some(2), 7);
        test(3, 6, Some(4), 7);
        test(4, 0, None, 7);
        test(4, 1, Some(4), 7);
        test(4, 2, Some(2), 7);
        test(4, 3, Some(6), 7);
        test(4, 4, Some(1), 7);
        test(4, 5, Some(5), 7);
        test(4, 6, Some(3), 7);
        test(5, 0, None, 7);
        test(5, 1, Some(5), 7);
        test(5, 2, Some(6), 7);
        test(5, 3, Some(4), 7);
        test(5, 4, Some(3), 7);
        test(5, 5, Some(1), 7);
        test(5, 6, Some(2), 7);
        test(6, 0, None, 7);
        test(6, 1, Some(6), 7);
        test(6, 2, Some(3), 7);
        test(6, 3, Some(2), 7);
        test(6, 4, Some(5), 7);
        test(6, 5, Some(4), 7);
        test(6, 6, Some(1), 7);
        test(0, 0, None, 8);
        test(0, 1, Some(0), 8);
        test(0, 2, None, 8);
        test(0, 3, Some(0), 8);
        test(0, 4, None, 8);
        test(0, 5, Some(0), 8);
        test(0, 6, None, 8);
        test(0, 7, Some(0), 8);
        test(1, 0, None, 8);
        test(1, 1, Some(1), 8);
        test(1, 2, None, 8);
        test(1, 3, Some(3), 8);
        test(1, 4, None, 8);
        test(1, 5, Some(5), 8);
        test(1, 6, None, 8);
        test(1, 7, Some(7), 8);
        test(2, 0, None, 8);
        test(2, 1, Some(2), 8);
        test(2, 2, None, 8);
        test(2, 3, Some(6), 8);
        test(2, 4, None, 8);
        test(2, 5, Some(2), 8);
        test(2, 6, None, 8);
        test(2, 7, Some(6), 8);
        test(3, 0, None, 8);
        test(3, 1, Some(3), 8);
        test(3, 2, None, 8);
        test(3, 3, Some(1), 8);
        test(3, 4, None, 8);
        test(3, 5, Some(7), 8);
        test(3, 6, None, 8);
        test(3, 7, Some(5), 8);
        test(4, 0, None, 8);
        test(4, 1, Some(4), 8);
        test(4, 2, None, 8);
        test(4, 3, Some(4), 8);
        test(4, 4, None, 8);
        test(4, 5, Some(4), 8);
        test(4, 6, None, 8);
        test(4, 7, Some(4), 8);
        test(5, 0, None, 8);
        test(5, 1, Some(5), 8);
        test(5, 2, None, 8);
        test(5, 3, Some(7), 8);
        test(5, 4, None, 8);
        test(5, 5, Some(1), 8);
        test(5, 6, None, 8);
        test(5, 7, Some(3), 8);
        test(6, 0, None, 8);
        test(6, 1, Some(6), 8);
        test(6, 2, None, 8);
        test(6, 3, Some(2), 8);
        test(6, 4, None, 8);
        test(6, 5, Some(6), 8);
        test(6, 6, None, 8);
        test(6, 7, Some(2), 8);
        test(7, 0, None, 8);
        test(7, 1, Some(7), 8);
        test(7, 2, None, 8);
        test(7, 3, Some(5), 8);
        test(7, 4, None, 8);
        test(7, 5, Some(3), 8);
        test(7, 6, None, 8);
        test(7, 7, Some(1), 8);
        test(0, 0, None, 9);
        test(0, 1, Some(0), 9);
        test(0, 2, Some(0), 9);
        test(0, 3, None, 9);
        test(0, 4, Some(0), 9);
        test(0, 5, Some(0), 9);
        test(0, 6, None, 9);
        test(0, 7, Some(0), 9);
        test(0, 8, Some(0), 9);
        test(1, 0, None, 9);
        test(1, 1, Some(1), 9);
        test(1, 2, Some(5), 9);
        test(1, 3, None, 9);
        test(1, 4, Some(7), 9);
        test(1, 5, Some(2), 9);
        test(1, 6, None, 9);
        test(1, 7, Some(4), 9);
        test(1, 8, Some(8), 9);
        test(2, 0, None, 9);
        test(2, 1, Some(2), 9);
        test(2, 2, Some(1), 9);
        test(2, 3, None, 9);
        test(2, 4, Some(5), 9);
        test(2, 5, Some(4), 9);
        test(2, 6, None, 9);
        test(2, 7, Some(8), 9);
        test(2, 8, Some(7), 9);
        test(3, 0, None, 9);
        test(3, 1, Some(3), 9);
        test(3, 2, Some(6), 9);
        test(3, 3, None, 9);
        test(3, 4, Some(3), 9);
        test(3, 5, Some(6), 9);
        test(3, 6, None, 9);
        test(3, 7, Some(3), 9);
        test(3, 8, Some(6), 9);
        test(4, 0, None, 9);
        test(4, 1, Some(4), 9);
        test(4, 2, Some(2), 9);
        test(4, 3, None, 9);
        test(4, 4, Some(1), 9);
        test(4, 5, Some(8), 9);
        test(4, 6, None, 9);
        test(4, 7, Some(7), 9);
        test(4, 8, Some(5), 9);
        test(5, 0, None, 9);
        test(5, 1, Some(5), 9);
        test(5, 2, Some(7), 9);
        test(5, 3, None, 9);
        test(5, 4, Some(8), 9);
        test(5, 5, Some(1), 9);
        test(5, 6, None, 9);
        test(5, 7, Some(2), 9);
        test(5, 8, Some(4), 9);
        test(6, 0, None, 9);
        test(6, 1, Some(6), 9);
        test(6, 2, Some(3), 9);
        test(6, 3, None, 9);
        test(6, 4, Some(6), 9);
        test(6, 5, Some(3), 9);
        test(6, 6, None, 9);
        test(6, 7, Some(6), 9);
        test(6, 8, Some(3), 9);
        test(7, 0, None, 9);
        test(7, 1, Some(7), 9);
        test(7, 2, Some(8), 9);
        test(7, 3, None, 9);
        test(7, 4, Some(4), 9);
        test(7, 5, Some(5), 9);
        test(7, 6, None, 9);
        test(7, 7, Some(1), 9);
        test(7, 8, Some(2), 9);
        test(8, 0, None, 9);
        test(8, 1, Some(8), 9);
        test(8, 2, Some(4), 9);
        test(8, 3, None, 9);
        test(8, 4, Some(2), 9);
        test(8, 5, Some(7), 9);
        test(8, 6, None, 9);
        test(8, 7, Some(5), 9);
        test(8, 8, Some(1), 9);
        test(0, 0, None, 10);
        test(0, 1, Some(0), 10);
        test(0, 2, None, 10);
        test(0, 3, Some(0), 10);
        test(0, 4, None, 10);
        test(0, 5, None, 10);
        test(0, 6, None, 10);
        test(0, 7, Some(0), 10);
        test(0, 8, None, 10);
        test(0, 9, Some(0), 10);
        test(1, 0, None, 10);
        test(1, 1, Some(1), 10);
        test(1, 2, None, 10);
        test(1, 3, Some(7), 10);
        test(1, 4, None, 10);
        test(1, 5, None, 10);
        test(1, 6, None, 10);
        test(1, 7, Some(3), 10);
        test(1, 8, None, 10);
        test(1, 9, Some(9), 10);
        test(2, 0, None, 10);
        test(2, 1, Some(2), 10);
        test(2, 2, None, 10);
        test(2, 3, Some(4), 10);
        test(2, 4, None, 10);
        test(2, 5, None, 10);
        test(2, 6, None, 10);
        test(2, 7, Some(6), 10);
        test(2, 8, None, 10);
        test(2, 9, Some(8), 10);
        test(3, 0, None, 10);
        test(3, 1, Some(3), 10);
        test(3, 2, None, 10);
        test(3, 3, Some(1), 10);
        test(3, 4, None, 10);
        test(3, 5, None, 10);
        test(3, 6, None, 10);
        test(3, 7, Some(9), 10);
        test(3, 8, None, 10);
        test(3, 9, Some(7), 10);
        test(4, 0, None, 10);
        test(4, 1, Some(4), 10);
        test(4, 2, None, 10);
        test(4, 3, Some(8), 10);
        test(4, 4, None, 10);
        test(4, 5, None, 10);
        test(4, 6, None, 10);
        test(4, 7, Some(2), 10);
        test(4, 8, None, 10);
        test(4, 9, Some(6), 10);
        test(5, 0, None, 10);
        test(5, 1, Some(5), 10);
        test(5, 2, None, 10);
        test(5, 3, Some(5), 10);
        test(5, 4, None, 10);
        test(5, 5, None, 10);
        test(5, 6, None, 10);
        test(5, 7, Some(5), 10);
        test(5, 8, None, 10);
        test(5, 9, Some(5), 10);
        test(6, 0, None, 10);
        test(6, 1, Some(6), 10);
        test(6, 2, None, 10);
        test(6, 3, Some(2), 10);
        test(6, 4, None, 10);
        test(6, 5, None, 10);
        test(6, 6, None, 10);
        test(6, 7, Some(8), 10);
        test(6, 8, None, 10);
        test(6, 9, Some(4), 10);
        test(7, 0, None, 10);
        test(7, 1, Some(7), 10);
        test(7, 2, None, 10);
        test(7, 3, Some(9), 10);
        test(7, 4, None, 10);
        test(7, 5, None, 10);
        test(7, 6, None, 10);
        test(7, 7, Some(1), 10);
        test(7, 8, None, 10);
        test(7, 9, Some(3), 10);
        test(8, 0, None, 10);
        test(8, 1, Some(8), 10);
        test(8, 2, None, 10);
        test(8, 3, Some(6), 10);
        test(8, 4, None, 10);
        test(8, 5, None, 10);
        test(8, 6, None, 10);
        test(8, 7, Some(4), 10);
        test(8, 8, None, 10);
        test(8, 9, Some(2), 10);
        test(9, 0, None, 10);
        test(9, 1, Some(9), 10);
        test(9, 2, None, 10);
        test(9, 3, Some(3), 10);
        test(9, 4, None, 10);
        test(9, 5, None, 10);
        test(9, 6, None, 10);
        test(9, 7, Some(7), 10);
        test(9, 8, None, 10);
        test(9, 9, Some(1), 10);
    }
}