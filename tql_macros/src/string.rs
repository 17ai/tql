/*
 * Copyright (C) 2015  Boucher, Antoni <bouanto@zoho.com>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! String proximity lookup function.

use std::cmp;

/// Variadic minimum macro. It returns the minimum of its arguments.
macro_rules! min {
    ( $e:expr ) => {
        $e
    };
    ( $e:expr, $( $rest:expr ),* ) => {
        cmp::min($e, min!($( $rest ),*))
    };
}

/// Finds a near match of `str_to_check` in `strings`.
#[allow(needless_lifetimes)]
pub fn find_near<'a, T>(str_to_check: &str, strings: T) -> Option<&'a String>
    where T: Iterator<Item = &'a String>
{
    let mut result = None;
    let mut best_distance = str_to_check.len();
    for string in strings {
        let distance = levenshtein_distance(&string, str_to_check);
        if distance < best_distance {
            best_distance = distance;
            if distance < 3 {
                result = Some(string);
            }
        }
    }
    result
}

/// Returns the Levensthein distance between `string1` and `string2`.
#[allow(needless_range_loop)]
fn levenshtein_distance(string1: &str, string2: &str) -> usize {
    fn distance(i: usize, j: usize, d: &[Vec<usize>], string1: &str, string2: &str) -> usize {
        match (i, j) {
            (i, 0) => i,
            (0, j) => j,
            (i, j) => {
                let delta =
                    if string1.chars().nth(i - 1) == string2.chars().nth(j - 1) {
                        0
                    }
                    else {
                        1
                    };
                min!( d[i - 1][j] + 1
                    , d[i][j - 1] + 1
                    , d[i - 1][j - 1] + delta
                    )
            },
        }
    }

    let mut d = vec![];
    for i in 0 .. string1.len() + 1 {
        d.push(vec![]);
        for j in 0 .. string2.len() + 1 {
            let dist = distance(i, j, &d, string1, string2);
            d[i].push(dist);
        }
    }
    d[string1.len()][string2.len()]
}

/// Returns " was" if count equals 1, "s were" otherwise.
pub fn plural_verb<'a>(count: usize) -> &'a str {
    if count == 1 {
        " was"
    }
    else {
        "s were"
    }
}
