#![cfg(feature = "nightly")]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/closures.js")]
extern "C" {
    fn works_call(a: &Fn());
    fn works_thread(a: &Fn(u32) -> u32) -> u32;

    fn cannot_reuse_call(a: &Fn());
    #[wasm_bindgen(catch)]
    fn cannot_reuse_call_again() -> Result<(), JsValue>;

    fn long_lived_call1(a: &Closure<Fn()>);
    fn long_lived_call2(a: &Closure<FnMut(u32) -> u32>) -> u32;

    fn many_arity_call1(a: &Closure<Fn()>);
    fn many_arity_call2(a: &Closure<Fn(u32)>);
    fn many_arity_call3(a: &Closure<Fn(u32, u32)>);
    fn many_arity_call4(a: &Closure<Fn(u32, u32, u32)>);
    fn many_arity_call5(a: &Closure<Fn(u32, u32, u32, u32)>);
    fn many_arity_call6(a: &Closure<Fn(u32, u32, u32, u32, u32)>);
    fn many_arity_call7(a: &Closure<Fn(u32, u32, u32, u32, u32, u32)>);
    fn many_arity_call8(a: &Closure<Fn(u32, u32, u32, u32, u32, u32, u32)>);

    #[wasm_bindgen(js_name = many_arity_call1)]
    fn many_arity_call_mut1(a: &Closure<FnMut()>);
    #[wasm_bindgen(js_name = many_arity_call2)]
    fn many_arity_call_mut2(a: &Closure<FnMut(u32)>);
    #[wasm_bindgen(js_name = many_arity_call3)]
    fn many_arity_call_mut3(a: &Closure<FnMut(u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call4)]
    fn many_arity_call_mut4(a: &Closure<FnMut(u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call5)]
    fn many_arity_call_mut5(a: &Closure<FnMut(u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call6)]
    fn many_arity_call_mut6(a: &Closure<FnMut(u32, u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call7)]
    fn many_arity_call_mut7(a: &Closure<FnMut(u32, u32, u32, u32, u32, u32)>);
    #[wasm_bindgen(js_name = many_arity_call8)]
    fn many_arity_call_mut8(a: &Closure<FnMut(u32, u32, u32, u32, u32, u32, u32)>);

    #[wasm_bindgen(js_name = many_arity_call1)]
    fn many_arity_stack1(a: &Fn());
    #[wasm_bindgen(js_name = many_arity_call2)]
    fn many_arity_stack2(a: &Fn(u32));
    #[wasm_bindgen(js_name = many_arity_call3)]
    fn many_arity_stack3(a: &Fn(u32, u32));
    #[wasm_bindgen(js_name = many_arity_call4)]
    fn many_arity_stack4(a: &Fn(u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call5)]
    fn many_arity_stack5(a: &Fn(u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call6)]
    fn many_arity_stack6(a: &Fn(u32, u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call7)]
    fn many_arity_stack7(a: &Fn(u32, u32, u32, u32, u32, u32));
    #[wasm_bindgen(js_name = many_arity_call8)]
    fn many_arity_stack8(a: &Fn(u32, u32, u32, u32, u32, u32, u32));

    fn long_lived_dropping_cache(a: &Closure<Fn()>);
    #[wasm_bindgen(catch)]
    fn long_lived_dropping_call() -> Result<(), JsValue>;

    fn long_fnmut_recursive_cache(a: &Closure<FnMut()>);
    #[wasm_bindgen(catch)]
    fn long_fnmut_recursive_call() -> Result<(), JsValue>;

    fn fnmut_call(a: &mut FnMut());
    fn fnmut_thread(a: &mut FnMut(u32) -> u32) -> u32;

    fn fnmut_bad_call(a: &mut FnMut());
    #[wasm_bindgen(catch)]
    fn fnmut_bad_again(a: bool) -> Result<(), JsValue>;

    fn string_arguments_call(a: &mut FnMut(String));

    fn string_ret_call(a: &mut FnMut(String) -> String);

    fn drop_during_call_save(a: &Closure<Fn()>);
    fn drop_during_call_call();

    fn js_test_closure_returner();

    fn calling_it_throws(a: &Closure<FnMut()>) -> bool;

    fn call_val(f: &JsValue);

    #[wasm_bindgen(js_name = calling_it_throws)]
    fn call_val_throws(f: &JsValue) -> bool;
}

#[wasm_bindgen_test]
fn works() {
    let a = Cell::new(false);
    works_call(&|| a.set(true));
    assert!(a.get());

    assert_eq!(works_thread(&|a| a + 1), 3);
}

#[wasm_bindgen_test]
fn cannot_reuse() {
    cannot_reuse_call(&|| {});
    assert!(cannot_reuse_call_again().is_err());
}

#[wasm_bindgen_test]
fn long_lived() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::new(move || hit2.set(true));
    assert!(!hit.get());
    long_lived_call1(&a);
    assert!(hit.get());

    let hit = Rc::new(Cell::new(false));
    {
        let hit = hit.clone();
        let a = Closure::new(move |x| {
            hit.set(true);
            x + 3
        });
        assert_eq!(long_lived_call2(&a), 5);
    }
    assert!(hit.get());
}

#[wasm_bindgen_test]
fn many_arity() {
    many_arity_call1(&Closure::new(|| {}));
    many_arity_call2(&Closure::new(|a| assert_eq!(a, 1)));
    many_arity_call3(&Closure::new(|a, b| assert_eq!((a, b), (1, 2))));
    many_arity_call4(&Closure::new(|a, b, c| assert_eq!((a, b, c), (1, 2, 3))));
    many_arity_call5(&Closure::new(|a, b, c, d| {
        assert_eq!((a, b, c, d), (1, 2, 3, 4))
    }));
    many_arity_call6(&Closure::new(|a, b, c, d, e| {
        assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5))
    }));
    many_arity_call7(&Closure::new(|a, b, c, d, e, f| {
        assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6))
    }));
    many_arity_call8(&Closure::new(|a, b, c, d, e, f, g| {
        assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7))
    }));

    let s = String::new();
    many_arity_call_mut1(&Closure::once(move || drop(s)));
    let s = String::new();
    many_arity_call_mut2(&Closure::once(move |a| {
        drop(s);
        assert_eq!(a, 1);
    }));
    let s = String::new();
    many_arity_call_mut3(&Closure::once(move |a, b| {
        drop(s);
        assert_eq!((a, b), (1, 2));
    }));
    let s = String::new();
    many_arity_call_mut4(&Closure::once(move |a, b, c| {
        drop(s);
        assert_eq!((a, b, c), (1, 2, 3));
    }));
    let s = String::new();
    many_arity_call_mut5(&Closure::once(move |a, b, c, d| {
        drop(s);
        assert_eq!((a, b, c, d), (1, 2, 3, 4));
    }));
    let s = String::new();
    many_arity_call_mut6(&Closure::once(move |a, b, c, d, e| {
        drop(s);
        assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5));
    }));
    let s = String::new();
    many_arity_call_mut7(&Closure::once(move |a, b, c, d, e, f| {
        drop(s);
        assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6));
    }));
    let s = String::new();
    many_arity_call_mut8(&Closure::once(move |a, b, c, d, e, f, g| {
        drop(s);
        assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7));
    }));

    many_arity_stack1(&(|| {}));
    many_arity_stack2(&(|a| assert_eq!(a, 1)));
    many_arity_stack3(&(|a, b| assert_eq!((a, b), (1, 2))));
    many_arity_stack4(&(|a, b, c| assert_eq!((a, b, c), (1, 2, 3))));
    many_arity_stack5(&(|a, b, c, d| assert_eq!((a, b, c, d), (1, 2, 3, 4))));
    many_arity_stack6(&(|a, b, c, d, e| assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5))));
    many_arity_stack7(&(|a, b, c, d, e, f| assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6))));
    many_arity_stack8(
        &(|a, b, c, d, e, f, g| assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7))),
    );
}

struct Dropper(Rc<Cell<bool>>);
impl Drop for Dropper {
    fn drop(&mut self) {
        assert!(!self.0.get());
        self.0.set(true);
    }
}

#[wasm_bindgen_test]
fn call_fn_once_twice() {
    let dropped = Rc::new(Cell::new(false));
    let dropper = Dropper(dropped.clone());
    let called = Rc::new(Cell::new(false));

    let c = Closure::once({
        let called = called.clone();
        move || {
            assert!(!called.get());
            called.set(true);
            drop(dropper);
        }
    });

    many_arity_call_mut1(&c);
    assert!(called.get());
    assert!(dropped.get());

    assert!(calling_it_throws(&c));
}

#[wasm_bindgen_test]
fn once_into_js() {
    let dropped = Rc::new(Cell::new(false));
    let dropper = Dropper(dropped.clone());
    let called = Rc::new(Cell::new(false));

    let f = Closure::once_into_js({
        let called = called.clone();
        move || {
            assert!(!called.get());
            called.set(true);
            drop(dropper);
        }
    });

    call_val(&f);
    assert!(called.get());
    assert!(dropped.get());

    assert!(call_val_throws(&f));
}

#[wasm_bindgen_test]
fn long_lived_dropping() {
    let hit = Rc::new(Cell::new(false));
    let hit2 = hit.clone();
    let a = Closure::new(move || hit2.set(true));
    long_lived_dropping_cache(&a);
    assert!(!hit.get());
    assert!(long_lived_dropping_call().is_ok());
    assert!(hit.get());
    drop(a);
    assert!(long_lived_dropping_call().is_err());
}

#[wasm_bindgen_test]
fn long_fnmut_recursive() {
    let a = Closure::new(|| {
        assert!(long_fnmut_recursive_call().is_err());
    });
    long_fnmut_recursive_cache(&a);
    assert!(long_fnmut_recursive_call().is_ok());
}

#[wasm_bindgen_test]
fn fnmut() {
    let mut a = false;
    fnmut_call(&mut || a = true);
    assert!(a);

    let mut x = false;
    assert_eq!(
        fnmut_thread(&mut |a| {
            x = true;
            a + 1
        }),
        3
    );
    assert!(x);
}

#[wasm_bindgen_test]
fn fnmut_bad() {
    let mut x = true;
    let mut hits = 0;
    fnmut_bad_call(&mut || {
        hits += 1;
        if fnmut_bad_again(hits == 1).is_err() {
            return;
        }
        x = false;
    });
    assert_eq!(hits, 1);
    assert!(x);

    assert!(fnmut_bad_again(true).is_err());
}

#[wasm_bindgen_test]
fn string_arguments() {
    let mut x = false;
    string_arguments_call(&mut |s| {
        assert_eq!(s, "foo");
        x = true;
    });
    assert!(x);
}

#[wasm_bindgen_test]
fn string_ret() {
    let mut x = false;
    string_ret_call(&mut |mut s| {
        assert_eq!(s, "foo");
        s.push_str("bar");
        x = true;
        s
    });
    assert!(x);
}

#[wasm_bindgen_test]
fn drop_drops() {
    static mut HIT: bool = false;

    struct A;

    impl Drop for A {
        fn drop(&mut self) {
            unsafe {
                HIT = true;
            }
        }
    }
    let a = A;
    let x: Closure<Fn()> = Closure::new(move || drop(&a));
    drop(x);
    unsafe {
        assert!(HIT);
    }
}

#[wasm_bindgen_test]
fn drop_during_call_ok() {
    static mut HIT: bool = false;
    struct A;
    impl Drop for A {
        fn drop(&mut self) {
            unsafe {
                HIT = true;
            }
        }
    }

    let rc = Rc::new(RefCell::new(None));
    let rc2 = rc.clone();
    let x = 3;
    let a = A;
    let x: Closure<Fn()> = Closure::new(move || {
        // "drop ourselves"
        drop(rc2.borrow_mut().take().unwrap());

        // `A` should not have been destroyed as a result
        unsafe {
            assert!(!HIT);
        }

        // allocate some heap memory to try to paper over our `3`
        drop(String::from("1234567890"));

        // make sure our closure memory is still valid
        assert_eq!(x, 3);

        // make sure `A` is bound to our closure environment.
        drop(&a);
        unsafe {
            assert!(!HIT);
        }
    });
    drop_during_call_save(&x);
    *rc.borrow_mut() = Some(x);
    drop(rc);
    unsafe {
        assert!(!HIT);
    }
    drop_during_call_call();
    unsafe {
        assert!(HIT);
    }
}

#[wasm_bindgen_test]
fn test_closure_returner() {
    type ClosureType = FnMut() -> BadStruct;

    use js_sys::{Object, Reflect};
    use wasm_bindgen::JsCast;

    js_test_closure_returner();

    #[wasm_bindgen]
    pub struct ClosureHandle(Closure<ClosureType>);

    #[wasm_bindgen]
    pub struct BadStruct {}

    #[wasm_bindgen]
    pub fn closure_returner() -> Result<Object, JsValue> {
        let o = Object::new();

        let some_fn = Closure::wrap(Box::new(move || BadStruct {}) as Box<ClosureType>);
        Reflect::set(
            &o,
            &JsValue::from("someKey"),
            &some_fn.as_ref().unchecked_ref(),
        )
        .unwrap();
        Reflect::set(
            &o,
            &JsValue::from("handle"),
            &JsValue::from(ClosureHandle(some_fn)),
        )
        .unwrap();

        Ok(o)
    }
}
