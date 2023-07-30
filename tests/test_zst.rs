use vec_soa::prelude::*;

#[test]
pub fn test_basic() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, ()));
    vec.push((4, 5, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, _) in vec.iter() {
        sum += *a as u64 + *b as u64;
    }

    assert_eq!(sum, 1 + 2 + 4 + 5);
}

#[test]
pub fn test_swap_remove_one() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 1);

    vec.swap_remove(0);

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
pub fn test_clear() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 1);

    vec.clear();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
pub fn test_remove_repush() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, ()));
    vec.push((4, 5, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    vec.swap_remove(1);

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 1);

    vec.push((7, 8, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, _) in vec.iter() {
        sum += *a as u64 + *b as u64;
    }

    assert_eq!(sum, 1 + 2 + 7 + 8);
}

#[test]
pub fn test_clear_repush() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((1, 2, ()));
    vec.push((4, 5, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    vec.clear();

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    vec.push((7, 8, ()));
    vec.push((10, 11, ()));

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 2);

    let mut sum = 0;

    for (a, b, _) in vec.iter() {
        sum += *a as u64 + *b as u64;
    }

    assert_eq!(sum, 7 + 8 + 10 + 11);
}

#[test]
#[should_panic]
pub fn test_empty() {
    let vec = VecSoa3::<u16, u32, ()>::new();
    vec.index(0);
}

#[test]
#[should_panic]
pub fn test_one() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();
    vec.push((1, 2, ()));
    vec.index(1);
}

#[test]
#[should_panic]
pub fn test_swap_remove_empty() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();
    vec.swap_remove(0);
}

#[test]
#[should_panic]
pub fn test_reserve_massive() {
    let mut vec = VecSoa3::<u16, u32, ()>::new();
    vec.reserve(isize::MAX as usize + 1);
}

#[test]
#[should_panic]
pub fn test_with_capacity_massive() {
    let _ = VecSoa3::<u16, u32, ()>::with_capacity(isize::MAX as usize + 1);
}

#[test]
pub fn test_iter_zst() {
    let mut vec = VecSoa3::<(), (), ()>::with_capacity(3);

    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);

    for _ in 0..10 {
        vec.push(((), (), ()));
    }

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 10);

    let mut sum = 0;

    for _ in vec.iter() {
        sum += 1;
    }

    assert_eq!(sum, 10);
}
