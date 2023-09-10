//! This module provides a convenient way to handle time-series data in Rust.
//! It allows the user to create and manipulate time-series data and perform arithmetic operations on them.
//! # Getting Started
//! import the module in your Rust code using:
//! ```rust
//! use time_series::TimeSeries;
//! ```
//! # Creating a new TimeSeries
//! To create a new TimeSeries, use the new() method:
//! ```rust
//! let mut ts: TimeSeries<f64> = TimeSeries::new();
//! ```
//! This creates a new TimeSeries that can hold floating-point numbers.
//! You can add elements to the TimeSeries using the push() method:
//! ```rust
//! ts.push(1.0);
//! ```
//! # Arithmetic operations
//! The TimeSeries type defines arithmetic operations for types that implement the Add, Sub, Mul, and Div traits. The following operations are available:
//!
//! - TimeSeries\<T\> @ TimeSeries\<T\>
//! - TimeSeries\<T\> @ &TimeSeries\<T\>
//! - &TimeSeries\<T\> @ TimeSeries\<T\>
//! - &TimeSeries\<T\> @ &TimeSeries\<T\>
//! However, @ refers to the four arithmetic operations +, -, *, /.
//! For example, to add two TimeSeries, use the + operator:
//! ```rust
//! let ts1: TimeSeries<f64> = TimeSeries::new();
//! let ts2: TimeSeries<f64> = TimeSeries::new();
//! let ts3 = &ts1 + &ts2;
//! let ts4 = ts1 + ts2;
//! ```
//! ### Attention.
//! I don't know why, but it seems that an error is detected by rust-analyzer regarding TimeSeries\<T\> @ TimeSeries\<T\>. It actually works, but may be a bit of a hindrance when coding.
//! # Mapping
//! You can apply a function to each element of a TimeSeries using the map() method. For example:
//! ```rust
//! let mut ts:TimeSeries<f64> = TimeSeries::new();
//! ts.push(1.);
//! ts.push(2.);
//! ts.push(3.);
//! let ts2 = ts.map(|x| x * 2.0);
//! dbg!(ts2);
//! ```

use std::ops::*;
use std::iter::{IntoIterator, Iterator};
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct TimeSeries<T>(pub Vec<T>);

impl<T> TimeSeries<T>
where T:Clone,
{
    pub fn new() -> Self {
	Self(Vec::new())
    }
    pub fn push(&mut self, value: T) {
	self.0.push(value);
    }
    pub fn pop(&mut self) -> Option<T>{
	if self.0.is_empty() {
	    return None;
	}
	Some(self.0.remove(0))
    }
    pub fn is_empty(&self) -> bool {
	self.0.is_empty()
    }
    pub fn first(&self) -> Option<T> {
	self.0.first().cloned()
    }
    pub fn last(&self) -> Option<T> {
	self.0.last().cloned()
    }
    pub fn clear(&mut self) {
	self.0.clear();
    }
    pub fn get(&self, index: usize) -> Option<T> {
	self.0.get(index).cloned()
    }
    pub fn filter<F>(&self, f: F) -> Self
    where
	F: Fn(&T) -> bool,
    {
	Self(self.0.iter().filter(|&item| f(item)).cloned().collect())
    }
    pub fn reverse(&self) -> Self {
	let mut reversed = self.0.clone();
	reversed.reverse();
	Self(reversed)
    }
    pub fn append(&mut self, other: &Self) {
	self.0.extend(other.0.clone());
    }


    /// Given a function f: T \-\> U that converts data to indicator, give a function map: TimeSeries\<T\> \-\> TimeSeries\<U\> that converts time series data to time series indices
    pub fn map<U, F>(&self, f: F) -> TimeSeries<U>
    where
	F: Fn(&T) -> U,
    {
	TimeSeries(self.0.iter().map(f).collect())
    }

    pub fn len(&self) -> usize{
	self.0.len()
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> Self{
	Self(self.0[range].to_vec())
    }
}

pub trait Variation{
    fn diff(&self, offet:usize) -> Self;
    fn pct_change(&self, offset:usize) ->Self;
}

impl<T> Variation for TimeSeries<T>
where T:Clone,
for<'a> &'a T: Add<Output = T>,
for<'a> &'a T: Sub<Output = T>,
for<'a> &'a T: Mul<Output = T>,
for<'a> &'a T: Div<Output = T>
{
    fn diff(&self, offset:usize) -> Self{
	let length = self.len();
	&self.slice(offset..length) - &self.slice(0..length-offset)
    }

    fn pct_change(&self, offset:usize) ->Self {
	let length = self.len();
	(&self.slice(offset..length) - &self.slice(0..length-offset)) / &self.slice(0..length-offset)
    }

    
}



impl<T> Default for TimeSeries<T> {
    fn default() -> Self {
	Self(Vec::new())
    }
}

impl<T, E> TimeSeries<Result<T, E>> {
    pub fn unwrap(self) -> Result<TimeSeries<T>, E> {
        let mut vec = Vec::new();
        
        for item in self.0.into_iter() {
            match item {
                Ok(val) => vec.push(val),
                Err(err) => return Err(err),
            }
        }
        
        Ok(TimeSeries(vec))
    }
}


#[auto_impl_ops::auto_ops]
impl<T> Add<&TimeSeries<T>> for TimeSeries<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = TimeSeries<T>;
    fn add(self, other: &Self) -> Self::Output {
        TimeSeries(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}


#[auto_impl_ops::auto_ops]
impl<T> Sub<&TimeSeries<T>> for TimeSeries<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = TimeSeries<T>;
    fn sub(self, other: &Self) -> Self::Output {
        TimeSeries(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}
#[auto_impl_ops::auto_ops]
impl<T> Mul<&TimeSeries<T>> for TimeSeries<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = TimeSeries<T>;
    fn mul(self, other: &Self) -> Self::Output {
        TimeSeries(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a * b)
                .collect(),
        )
    }
}

#[auto_impl_ops::auto_ops]
impl<T> Div<&TimeSeries<T>> for TimeSeries<T>
where
    for<'a> &'a T: Div<Output = T>,
{
    type Output = TimeSeries<T>;
    fn div(self, other: &Self) -> Self::Output {
        TimeSeries(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a / b)
                .collect(),
        )
    }
}


impl<T> Index<usize> for TimeSeries<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for TimeSeries<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl<T> Index<std::ops::Range<usize>> for TimeSeries<T> {
    type Output = [T];

    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<T> IndexMut<std::ops::Range<usize>> for TimeSeries<T> {
    fn index_mut(&mut self, range: std::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl<T> Index<RangeFrom<usize>> for TimeSeries<T> {
    type Output = [T];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<T> IndexMut<RangeFrom<usize>> for TimeSeries<T> {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}
impl<T> Index<RangeTo<usize>> for TimeSeries<T> {
    type Output = [T];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<T> IndexMut<RangeTo<usize>> for TimeSeries<T> {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

// impl<T> Index<Range<usize>> for TimeSeries<T> {
//     type Output = [T];

//     fn index(&self, range: Range<usize>) -> &Self::Output {
//         &self.0[range]
//     }
// }

// impl<T> IndexMut<Range<usize>> for TimeSeries<T> {
//     fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
//         &mut self.0[range]
//     }
// }

impl<T> AsRef<[T]> for TimeSeries<T> {
    fn as_ref(&self) -> &[T] {
	self.0.as_ref()
    }
}

impl<T> AsMut<[T]> for TimeSeries<T> {
    fn as_mut(&mut self) -> &mut [T] {
	self.0.as_mut()
    }
}


impl<T> IntoIterator for TimeSeries<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a TimeSeries<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut TimeSeries<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}



impl<T> FromIterator<T> for TimeSeries<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T> Extend<T> for TimeSeries<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}
