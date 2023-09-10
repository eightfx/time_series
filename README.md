
# Table of Contents

1.  [TimeSeries Library for Rust](#orgaab36fa)
    1.  [Table of Contents](#orgd36be22)
    2.  [Installation](#orgcc4b0e3)
    3.  [Usage](#org6ce0493)
    4.  [Features](#orgf8f619e)
    5.  [Highlight Feature: The map Method](#org867aba6)
        1.  [Using a Lambda Function](#org1ae0e89)
        2.  [Using a Named Function](#org03b3a93)
        3.  [Changing the Type](#orgf927a59)
    6.  [Contributing](#org7600270)


<a id="orgaab36fa"></a>

# TimeSeries Library for Rust

A simple and flexible TimeSeries library in Rust.


<a id="orgd36be22"></a>

## Table of Contents

-   Installation
-   Usage
-   Features
-   Contributing
-   License


<a id="orgcc4b0e3"></a>

## Installation

To install the TimeSeries library, add the following line to your `Cargo.toml`:


<a id="org6ce0493"></a>

## Usage

Import the library and start using it like so:

    extern crate time_series;
    
    use time_series::TimeSeries;
    use time_series::Variation; // if you want to use the variation methods like diff and pct_change

Here's a quick example that demonstrates how to create a new TimeSeries instance, add elements, and perform a map operation:

    let mut ts = TimeSeries::new();
    ts.push(1);
    ts.push(2);
    ts.push(3);
    
    let new_ts = ts.map(|&x| x * 2);
    println!("{:?}", new_ts); // Should print TimeSeries([2, 4, 6])


<a id="orgf8f619e"></a>

## Features

-   Basic TimeSeries manipulation methods such as `push`, `pop`, `len`, `is_empty`, and `slice`.
-   Variation trait for statistical calculations like `diff` and `pct_change`.
-   Generic design allows for storing any type that implements the `Clone` trait.


<a id="org867aba6"></a>

## Highlight Feature: The map Method

One of the key features of this TimeSeries library is the `map` method. This method allows you to transform a `TimeSeries<T>` into a `TimeSeries<U>` by applying a function `f: &T -> U` to each element in the series.

The function takes a closure or a named function that receives an immutable reference to the data point and should return a new data point of possibly a different type.


<a id="org1ae0e89"></a>

### Using a Lambda Function

Here's how you can use it with a lambda function:

    let mut ts = TimeSeries::new();
    ts.push(1);
    ts.push(2);
    ts.push(3);
    
    let new_ts = ts.map(|&x| x * 2);
    println!("{:?}", new_ts);  // Should print TimeSeries([2, 4, 6])


<a id="org03b3a93"></a>

### Using a Named Function

You can also use a named function to achieve the same transformation:

    fn transform(x: &i32) -> i32 {
        x * 2
    }
    
    let mut ts = TimeSeries::new();
    ts.push(1);
    ts.push(2);
    ts.push(3);
    
    let new_ts = ts.map(transform);
    println!("{:?}", new_ts);  // Should print TimeSeries([2, 4, 6])


<a id="orgf927a59"></a>

### Changing the Type

You can even change the type of data stored in the TimeSeries:

    let mut ts = TimeSeries::new();
    ts.push(1);
    ts.push(2);
    ts.push(3);
    
    let new_ts: TimeSeries<String> = ts.map(|&x| format!("Value: {}", x));
    println!("{:?}", new_ts);  // Should print TimeSeries(["Value: 1", "Value: 2", "Value: 3"])

This feature makes it incredibly easy to convert time series data into various time series indices or to apply any kind of transformations needed for your specific use-case.


<a id="org7600270"></a>

## Contributing

Contributions are welcome! Please fork the repository and open a pull request with your changes, or open an issue for discussion.

