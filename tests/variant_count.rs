#![allow(dead_code)]

use variant_count::VariantCount;

#[derive(VariantCount)]
enum Test {
    First(i32),
    Second(Option<String>),
    Third,
}

mod foo {
    use variant_count::VariantCount;

    #[derive(VariantCount)]
    pub enum TestGen<'a, T: 'a, U>
        where U: 'a
    {
        First(T),
        Second(Option<&'a U>),
        Third(&'a T),
        Fourth(U),
    }
}

#[test]
fn variant_count() {
    assert_eq!(Test::VARIANT_COUNT, 3);
    assert_eq!(foo::TestGen::<i32, bool>::VARIANT_COUNT, 4);
}
