// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

#[derive(Clone)] 
struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    // 新的函数，将指针转换为Box
    //重要的是要注意，Box::from_raw不会为你分配内存；它只是接管一个已经存在的原始指针的所有权。因此，在调用Box::from_raw之前，你必须确保指针指向的内存是有效且适当分配的。

    //此外，一旦你使用Box::from_raw创建了一个Box，你就不能再使用原始的*mut Foo指针了，因为它已经转移了所有权。如果你再次使用原始的指针，将会导致未定义行为。
    let  mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b=Some("hello".to_owned());
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
