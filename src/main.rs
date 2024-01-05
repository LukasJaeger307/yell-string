/* 
 * Copyright 2023, Lukas Jäger
 *
 * This file is part of yell-string.
 *
 * yell-string is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * yell-string is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with yell-string.  If not, see <http://www.gnu.org/licenses/>.
 */
use std::char::ToLowercase;
use std::char::ToUppercase;
use std::env;

#[cfg(test)]
mod tests {

    use crate::yellify_string;

    fn yellify_test(to_yellify : &str, expected_result : &str) {
        let yellified : String = yellify_string(&to_yellify.to_string());
        assert_eq!(yellified, expected_result.to_string());
    }

    #[test]
    fn test_yellify_string_all_lowercase_letters() {
        yellify_test(&"abc", &"AbC");
    }
    
    #[test]
    fn test_yellify_string_all_uppercase_letters() {
        yellify_test(&"ABC", &"AbC");
    }

    #[test]
    fn test_yellify_string_sentence() {
        yellify_test(&"Docker is great!", &"DoCkEr Is GrEaT!");
    }

    #[test]
    fn test_yellify_string_umlaut() {
        yellify_test(&"Grüne sind supi!", &"GrÜnE sInD sUpI!");
    }

    #[test]
    fn test_yellify_string_ss_odd_index() {
        yellify_test(&"Straße", &"StRaße");
    }
    
    #[test]
    fn test_yellify_string_ss_even_index() {
        yellify_test(&"Gruß", &"GrUß");
    }
}

fn process_alphabetic_char(alphabetic_char : &char, uppercase : &bool) -> char {
    if *uppercase {
        let mut to_uppercase : ToUppercase = alphabetic_char.to_uppercase();
        if to_uppercase.len() == 1 {
            match to_uppercase.next() {
                Some(uppercase_char) => uppercase_char,
                None => *alphabetic_char,
            }
        } else {
            *alphabetic_char      
	}
    } else {
        let mut to_lowercase : ToLowercase = alphabetic_char.to_lowercase();
        if to_lowercase.len() == 1 {
            match to_lowercase.next() {
                Some(lowercase_char) => lowercase_char,
                None => *alphabetic_char,
            }
        } else {
            *alphabetic_char
        }
    }
}

fn yellify_string(to_yellify : &str) -> String {
    let mut yellified : String = String::new();
    let mut uppercase : bool = true;
    for letter in to_yellify.chars() {
        if letter.is_alphabetic() {
            let processed_char : char = process_alphabetic_char(&letter, &uppercase);
            yellified.push(processed_char);
            uppercase = !uppercase;
        } else {
            yellified.push(letter);
        }
    }
    yellified
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut skip_this_string = true;
    for arg in args.iter() {
        if skip_this_string {
            skip_this_string = false;
        } else {
            let yellified_string : String = yellify_string(arg);
            println!("{}", yellified_string);
        }
    }
}
