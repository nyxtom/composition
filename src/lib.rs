#![feature(fn_traits)]
#![feature(unboxed_closures)]
use std::marker::PhantomData;

pub struct Map<A, B, Args, Args2, T>(A, B, PhantomData<Args>, PhantomData<Args2>, PhantomData<T>)
where
    A: Fn<Args>,
    B: Fn<Args2>;

fn map<A, B, Args, Args2, T>(a: A, b: B) -> Map<A, B, Args, Args2, T>
where
    A: Fn<Args>,
    B: Fn<Args2>,
{
    Map(
        a,
        b,
        PhantomData::default(),
        PhantomData::default(),
        PhantomData::default(),
    )
}

fn map_ok<A, B, Args, Args2, T, E>(a: A, b: B) -> Map<A, B, Args, Args2, Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<Args2>,
{
    Map(
        a,
        b,
        PhantomData::default(),
        PhantomData::default(),
        PhantomData::default(),
    )
}

impl<A, B, Args, T> Map<A, B, Args, (T,), (T,)>
where
    A: Fn<Args, Output = T>,
    B: Fn<(T,)>,
{
    #[inline]
    fn apply_tuple(&self, args: Args) -> B::Output {
        let args = self.0.call(args);
        self.1.call((args,))
    }
}

impl<A, B, Args, T> Map<A, B, Args, T, ()>
where
    A: Fn<Args, Output = T>,
    B: Fn<T>,
{
    #[inline]
    fn apply(&self, args: Args) -> B::Output {
        let args = self.0.call(args);
        self.1.call(args)
    }
}

impl<A, B, Args, T, E> Map<A, B, Args, (T,), Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<(T,)>,
{
    #[inline]
    fn apply_ok_tuple(&self, args: Args) -> Result<B::Output, E> {
        let args = self.0.call(args);
        match args {
            Ok(args) => Ok(self.1.call((args,))),
            Err(e) => Err(e),
        }
    }
}

impl<A, B, Args, T, E> Map<A, B, Args, T, Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<T>,
{
    #[inline]
    fn apply_ok(&self, args: Args) -> Result<B::Output, E> {
        let args = self.0.call(args);
        match args {
            Ok(args) => Ok(self.1.call(args)),
            Err(e) => Err(e),
        }
    }
}

impl<A, B, Args, T, E> Fn<Args> for Map<A, B, Args, (T,), Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<(T,)>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.apply_ok_tuple(args)
    }
}

impl<A, B, Args, T, E> FnMut<Args> for Map<A, B, Args, (T,), Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<(T,)>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.apply_ok_tuple(args)
    }
}

impl<A, B, Args, T, E> FnOnce<Args> for Map<A, B, Args, (T,), Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<(T,)>,
{
    type Output = Result<B::Output, E>;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.apply_ok_tuple(args)
    }
}

impl<A, B, Args, T, E> Fn<Args> for Map<A, B, Args, T, Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<T>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.apply_ok(args)
    }
}

impl<A, B, Args, T, E> FnMut<Args> for Map<A, B, Args, T, Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<T>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.apply_ok(args)
    }
}

impl<A, B, Args, T, E> FnOnce<Args> for Map<A, B, Args, T, Result<T, E>>
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<T>,
{
    type Output = Result<B::Output, E>;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.apply_ok(args)
    }
}

impl<A, B, Args> Fn<Args> for Map<A, B, Args, (A::Output,), (A::Output,)>
where
    A: Fn<Args>,
    B: Fn<(A::Output,)>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.apply_tuple(args)
    }
}

impl<A, B, Args> FnMut<Args> for Map<A, B, Args, (A::Output,), (A::Output,)>
where
    A: Fn<Args>,
    B: Fn<(A::Output,)>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.apply_tuple(args)
    }
}

impl<A, B, Args> FnOnce<Args> for Map<A, B, Args, (A::Output,), (A::Output,)>
where
    A: Fn<Args>,
    B: Fn<(A::Output,)>,
{
    type Output = B::Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.apply_tuple(args)
    }
}

impl<A, B, Args> Fn<Args> for Map<A, B, Args, A::Output, ()>
where
    A: Fn<Args>,
    B: Fn<A::Output>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.apply(args)
    }
}

impl<A, B, Args> FnMut<Args> for Map<A, B, Args, A::Output, ()>
where
    A: Fn<Args>,
    B: Fn<A::Output>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.apply(args)
    }
}

impl<A, B, Args> FnOnce<Args> for Map<A, B, Args, A::Output, ()>
where
    A: Fn<Args>,
    B: Fn<A::Output>,
{
    type Output = B::Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.apply(args)
    }
}

fn foo() {}

fn test() -> i32 {
    3
}
fn plus(a: i32) -> i32 {
    a + 1
}
fn times(a: i32) -> i32 {
    a * 3
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn output() -> (i32, i32) {
    (4, 2)
}
fn error_in(a: i32) -> Result<i32, String> {
    if a > 4 {
        Ok(a)
    } else {
        Err("Value cannot be above 4".into())
    }
}
fn errors(a: i32) -> Result<(i32, i32), String> {
    if a > 4 {
        Ok((a + 1, a + 2))
    } else {
        Err("Value cannot be above 4".into())
    }
}

fn main() {
    map(foo, foo)();
    map(foo, test)();
    map(test, plus)();
    map(plus, plus)(4);
    map(test, plus)();
    map(multiply, plus)(4, 5);
    map(output, multiply)();
    map(map(output, multiply), plus)();
    map_ok(error_in, plus);
    map_ok(map_ok(error_in, plus), plus);
    map_ok(map_ok(error_in, plus), error_in);
    map_ok(map_ok(map_ok(error_in, plus), error_in), plus);
    map_ok(errors, multiply);
    map_ok(map_ok(errors, multiply), errors);
}
