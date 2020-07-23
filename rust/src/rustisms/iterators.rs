// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

#[allow(dead_code)]
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Makes v1_iter mut behind the scenes

    // .iter() returns immutable references
    // .into_iter() returns owned values
    // .iter_mut() returns mutable references

    // Iterator adapters
    // Ex: Map
    let v1: Vec<i32> = vec![1, 2, 3];

    // map specifies a function to run on each iter item
    // collect iterates through each item and bundles them into a collection
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // Filter
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // This method "consumes" the iterator
    // Hence why we need to make it mut
}

// sum consumes iter
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Implement Iterator trait on our own classes
struct Counter {
    count: u32,
}

impl Counter {
    #[allow(dead_code)]
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    #[allow(dead_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // Creates a map of our counter value and a new counter that skips 1
        .map(|(a, b)| a * b) // Multiplies each pair together
        .filter(|x| x % 3 == 0) // Keep the ones divisible by 3
        .sum(); // Add them all up
    assert_eq!(18, sum);
}