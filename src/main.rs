#![allow(unused)]

use rayon::prelude::*;
use rayon::iter::once;
use nalgebra::{DMatrix};
use fraction::Fraction;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::cmp::{PartialEq, PartialOrd};
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::env::args;

#[derive(Debug, PartialEq, PartialOrd)]
enum NumCombo {
  BTuple(f64, f64),
  Singly(f64)
}

trait NumTrait<T> {
  fn comb(self, _: T) -> T;
  
  fn fact(self) -> T;
  
  fn perm(self, _: T) -> T;
}

impl NumTrait<u128> for u128 {
  fn comb(self, rhs: u128) -> u128 {
    let res = [self, rhs, self - rhs];
    let res = res.par_iter().map(|v| v.fact()).collect::<Vec<_>>();
    res[0] / (res[1] * res[2])
  }
  
  fn fact(self) -> u128 {
    (1..=self).fold(1, |a, b| a * b)
  }
  
  fn perm(self, rhs: u128) -> u128 {
    let res = [self, self - rhs];
    let res = res.par_iter().map(|v| v.fact()).collect::<Vec<_>>();
    res[0] / res[1]
  }
}

macro_rules! impl_fns_dry {
  ($self:ident, $method:ident) => {
    match $self {
      NumCombo::BTuple(x, y) => NumCombo::BTuple(x.$method(), y.$method()),
      NumCombo::Singly(v) => NumCombo::Singly(v.$method()),
    }
  };
  ($lhs:ident, $rhs:ident, $method:ident) => {
    match ($lhs, $rhs) {
      (NumCombo::BTuple(x, y), NumCombo::BTuple(x2, y2)) => NumCombo::BTuple(x.$method(x2), y.$method(y2)),
      (NumCombo::BTuple(x, y), NumCombo::Singly(v)) => NumCombo::BTuple(x.$method(v), y.$method(v)),
      (NumCombo::Singly(v), NumCombo::BTuple(x, y)) => NumCombo::BTuple(v.$method(x), v.$method(y)),
      (NumCombo::Singly(v), NumCombo::Singly(v2)) => NumCombo::Singly(v.$method(v2)),
    }
  };
}

macro_rules! nc_impls {
  (arith; $($imp:ident, $method:ident$(;)?)+) => {
    $(impl $imp for NumCombo {
      type Output = Self;
      fn $method(self, rhs: Self) -> Self {
        impl_fns_dry!(self, rhs, $method)
      }
    })+
  };
  (bi; $($method:ident$(,)?)+) => {
    $(impl NumCombo {
      fn $method(self, rhs: Self) -> Self {
        impl_fns_dry!(self, rhs, $method)
      }
    })+
  };
  (uni; $($method:ident$(,)?)+) => {
    $(impl NumCombo {
      fn $method(self) -> Self {
        impl_fns_dry!(self, $method)
      }
    })+
  };
}

impl Neg for NumCombo {
  type Output = Self;
  fn neg(self) -> Self {
    impl_fns_dry!(self, neg)
  }
}

nc_impls!(arith; Add, add; Div, div; Mul, mul; Rem, rem; Sub, sub);
nc_impls!(uni; acos, acosh, asin, asinh, atan, atanh, cbrt, cos, cosh, ln, log10, sin, sinh, sqrt, tan, tanh);
nc_impls!(bi; atan2, hypot, powf);

fn quad(a: f64, b: f64, c: f64) -> NumCombo {
  let d = (b.powi(2)-4.*a*c).sqrt();
  let (x, x2) = ((-b+d)/(2.*a), (-b-d)/(2.*a));
  if x.is_nan() || x == x2 {
    NumCombo::Singly(x)
  } else {
    NumCombo::BTuple(x, x2)
  }
}

fn token_finder(expr: &str) -> Option<Match> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"[\+\-]*[0-9\.]*([@#][A-Za-z]+|[\+\-/\*\(\)])?[0-9\.]*").unwrap();
  }
  RE.find(expr)
}

fn args_analyzer(expr: &str) -> f64 {
  expr.chars().for_each(|c| println!("{c}"));
  0.
}

fn main() {
  // let raw_data = "0,1;2,7;4,13;6,19;8,25"
  //   .par_split(';')
  //   .flat_map(|i| once(1.0).chain(
  //     i.par_split(',')
  //     .map(|j| j.parse().unwrap())
  //   ))
  //   .collect::<Vec<_>>();
  // let dataset = DMatrix::from_row_slice(&raw_data.len()/3, 3, &raw_data);
  // let (mxn, y) = (dataset.columns(0, 2), dataset.column(2));
  // let qr = mxn.qr();
  // let params = qr.r().try_inverse().unwrap()*qr.q().transpose()*y;
  println!("{:?}", token_finder(&args().nth(1).unwrap()));
}