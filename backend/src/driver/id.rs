/*
 * use nanoid::nanoid;

fn main() {
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
    ];

   let id = nanoid!(10, &alphabet); //=> "4f90d13a42"
}

Use nano id with a custom alphabet to generate unique IDs like 'w21lmlkbpjwsqugnuq4l' with a declarative macro id!()
*/

pub const ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

macro_rules! id {
    () => {
        nanoid::nanoid!(21, &crate::driver::id::ALPHABET)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_macro() {
        let id = id!();
        assert_eq!(id.len(), 21);
        assert!(id.chars().all(|c| ALPHABET.contains(&c)));
        println!("Generated ID: {}", id);
    }
}
