use vec_soa::prelude::*;

#[test]
pub fn test_basic() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, 3));
    vec.push((4, 5, 6));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, c) in vec.iter() {
        sum += *a as u64 + *b as u64 + *c as u64;
    }

    assert_eq!(sum, 1 + 2 + 3 + 4 + 5 + 6);
}

#[test]
pub fn test_mut() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, 3));
    vec.push((4, 5, 6));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, c) in vec.iter_mut() {
        *a += 1;
        *b += 2;
        *c += 3;
    }

    for (a, b, c) in vec.iter() {
        sum += *a as u64 + *b as u64 + *c as u64;
    }

    assert_eq!(
        sum,
        (1 + 1) + (2 + 2) + (3 + 3) + (4 + 1) + (5 + 2) + (6 + 3)
    );
}

#[test]
pub fn test_swap_remove_one() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, 3));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 1);

    vec.swap_remove(0);

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
pub fn test_clear() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, 3));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 1);

    vec.clear();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
pub fn test_remove_repush() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, 3));
    vec.push((4, 5, 6));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    vec.swap_remove(1);

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 1);

    vec.push((7, 8, 9));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, c) in vec.iter() {
        sum += *a as u64 + *b as u64 + *c as u64;
    }

    assert_eq!(sum, 1 + 2 + 3 + 7 + 8 + 9);
}

#[test]
pub fn test_clear_repush() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, 3));
    vec.push((4, 5, 6));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    vec.clear();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((7, 8, 9));
    vec.push((10, 11, 12));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, c) in vec.iter() {
        sum += *a as u64 + *b as u64 + *c as u64;
    }

    assert_eq!(sum, 7 + 8 + 9 + 10 + 11 + 12);
}

#[test]
#[should_panic]
pub fn test_empty() {
    let vec = VecSoa3::<u16, u32, u64>::new();
    vec.index(0);
}

#[test]
#[should_panic]
pub fn test_one() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();
    vec.push((1, 2, 3));
    vec.index(1);
}

#[test]
#[should_panic]
pub fn test_swap_remove_empty() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();
    vec.swap_remove(0);
}

#[test]
#[should_panic]
pub fn test_reserve_massive() {
    let mut vec = VecSoa3::<u16, u32, u64>::new();
    vec.reserve(isize::MAX as usize + 1);
}

#[test]
#[should_panic]
pub fn test_with_capacity_massive() {
    let _ = VecSoa3::<u16, u32, u64>::with_capacity(isize::MAX as usize + 1);
}
