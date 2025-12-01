#![allow(unused)]
use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of, space0},
    combinator::{map_res, opt, recognize},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{delimited, pair},
    AsChar, IResult, Parser,
};

pub fn int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(pair(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<T>()
    }).parse(input)
}

pub fn newline(input: &str) -> IResult<&str, ()> {
    many1(one_of("\n\r")).map(|_| ()).parse(input)
}

pub fn space_separated_ints<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(many1(tag(" ")), int::<T>).parse(input)
}

pub fn kait_ints<T: FromStr>(input: &str) -> Vec<T> {
    input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|s| s.parse::<T>().ok())
        .collect()
}
