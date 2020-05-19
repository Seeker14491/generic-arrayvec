use generic_arrayvec::{
    generic_array::GenericArray,
    typenum::{U10, U41, U5},
    *,
};

#[test]
fn test_vec_simple() {
    let mut vec = GenericArrayVec::<i32, U41>::new();

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 41);
    vec.extend(0..20);
    assert_eq!(vec.len(), 20);
    assert_eq!(&vec[..5], &[0, 1, 2, 3, 4]);
}

#[test]
fn test_vec_from_array() {
    let vec: GenericArrayVec<i32, _> = GenericArrayVec::generic_from([0, 1, 2, 3, 4]);

    assert_zero_to_four(&vec);
}

#[test]
fn test_vec_from_generic_array() {
    let arr: GenericArray<i32, U5> = GenericArray::clone_from_slice(&[0, 1, 2, 3, 4]);
    let vec = GenericArrayVec::generic_from(arr);

    assert_zero_to_four(&vec);
}

#[test]
fn test_vec_from_iter() {
    let vec: GenericArrayVec<i32, U10> = (0..10).collect();

    assert_zero_to_four(&vec);
}

#[test]
fn test_vec_into_generic_array() {
    let vec: GenericArrayVec<i32, _> = GenericArrayVec::generic_from([0, 1, 2, 3, 4]);
    let arr = vec.into_generic_array().unwrap();

    assert_zero_to_four(&arr);
}

#[test]
fn test_string_from() {
    let string = GenericArrayString::<U10>::generic_from("hello").unwrap();

    assert_eq!(&string, "hello");
}

#[test]
fn test_string_from_byte_string_literal() {
    let byte_string = b"hello";
    let string = GenericArrayString::<U5>::generic_from_byte_string(byte_string).unwrap();

    assert_eq!(&string, "hello");
}

#[test]
fn test_string_from_byte_string_generic() {
    let byte_string = GenericArray::from(b"hello".clone());
    let string = GenericArrayString::<U5>::generic_from_byte_string(&byte_string).unwrap();

    assert_eq!(&string, "hello");
}

fn assert_zero_to_four<T>(vec: &T)
where
    T: AsRef<[i32]>,
{
    assert_eq!(&vec.as_ref()[..5], &[0, 1, 2, 3, 4][..]);
}
