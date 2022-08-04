#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]
use std::marker::PhantomData;

pub trait Func<Args, T> {
    type Output;
    fn call(&self, args: Args) -> Self::Output;
}

// Default implementation of a func for T as output
impl<A, B, Args, T> Func<Args, ()> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Fn<T>,
{
    type Output = B::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call(args)
    }
}

// Subset of (A, B) T is (T,)
impl<A, B, Args, T> Func<Args, (T,)> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Fn<(T,)>,
{
    type Output = B::Output;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call((args,))
    }
}

// Subset of (A, B) where A is already a tuple that implements Func
impl<A, B, Args, T, F> Func<Args, ((), (), F)> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Func<T, F>,
{
    type Output = B::Output;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call(args)
    }
}

// Subset of (A, B) where is A is Func and B takes (T,)
impl<A, B, Args, T, F> Func<Args, ((), (T,), F)> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Func<(T,), F>,
{
    type Output = B::Output;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call((args,))
    }
}

// Default implementation for a func where T is a result
impl<A, B, Args, T, E> Func<Args, (Result<T, E>, ())> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<T>,
{
    type Output = Result<B::Output, E>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => Ok(self.1.call(args)),
            Err(e) => Err(e),
        }
    }
}

// Subset of (A, B) T is (T,)
impl<A, B, Args, T, E> Func<Args, (Result<T, E>, ((),))> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<(T,)>,
{
    type Output = Result<B::Output, E>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => Ok(self.1.call((args,))),
            Err(e) => Err(e),
        }
    }
}

// Subset of (A, B) where A is already a tuple that implements Func
impl<A, B, Args, T, E, F> Func<Args, (Result<T, E>, (), F)> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Func<T, F>,
{
    type Output = Result<B::Output, E>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => Ok(self.1.call(args)),
            Err(e) => Err(e),
        }
    }
}

// Subset of (A, B) where is A is Func and B takes (T,)
impl<A, B, Args, T, E, F> Func<Args, (Result<T, E>, ((),), F)> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Func<(T,), F>,
{
    type Output = Result<B::Output, E>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => Ok(self.1.call((args,))),
            Err(e) => Err(e),
        }
    }
}

// Default implementation for a func where T is an option
impl<A, B, Args, T> Func<Args, (Option<T>, ())> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Fn<T>,
{
    type Output = Option<B::Output>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call(args)),
            None => None,
        }
    }
}

// Subset of (A, B) T is (T,)
impl<A, B, Args, T> Func<Args, (Option<T>, ((),))> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Fn<(T,)>,
{
    type Output = Option<B::Output>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call((args,))),
            None => None,
        }
    }
}

// Subset of (A, B) where A is already a tuple that implements Func
impl<A, B, Args, T, F> Func<Args, (Option<T>, (), F)> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Func<T, F>,
{
    type Output = Option<B::Output>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call(args)),
            None => None,
        }
    }
}

// Subset of (A, B) where is A is Func and B takes (T,)
impl<A, B, Args, T, F> Func<Args, (Option<T>, ((),), F)> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Func<(T,), F>,
{
    type Output = Option<B::Output>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call((args,))),
            None => None,
        }
    }
}

pub struct Function<F, T>(F, PhantomData<T>);

impl<F, Args, T> Fn<Args> for Function<F, T>
where
    F: Func<Args, T>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.0.call(args)
    }
}

impl<F, Args, T> FnMut<Args> for Function<F, T>
where
    F: Func<Args, T>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.0.call(args)
    }
}

impl<F, Args, T> FnOnce<Args> for Function<F, T>
where
    F: Func<Args, T>,
{
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.0.call(args)
    }
}
/*
#[async_trait]
impl<A, B, Args, Fut, T> Handler<Args, (T,)> for Map<A, B, Args, (T,), Fut>
where
    A: Fn<Args, Output = Fut> + Send + Sync,
    B: Fn<(T,)> + Send + Sync,
    Fut: Future<Output = T> + Send + Sync,
    T: Send + Sync,
    B::Output: Send + Sync,
    Args: Send + Sync,
{
    type Output = B::Output;
    async fn call(&self, args: Args) -> Self::Output {
        let fut = self.0.call(args);
        let args = fut.await;
        self.1.call((args,))
    }
}

#[async_trait]
impl<A, B, Args, Fut, T> Handler<Args, ()> for Map<A, B, Args, T, Fut>
where
    A: Fn<Args, Output = Fut> + Send + Sync,
    B: Fn<T> + Send + Sync,
    Fut: Future<Output = T> + Send + Sync,
    T: Send + Sync,
    B::Output: Send + Sync,
    Args: Send + Sync,
{
    type Output = B::Output;
    async fn call(&self, args: Args) -> Self::Output {
        let fut = self.0.call(args);
        let args = fut.await;
        self.1.call(args)
    }
}
*/

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
fn optional(a: i32) -> Option<i32> {
    if a > 10 {
        None
    } else {
        Some(a + 2)
    }
}
async fn async_test() -> i32 {
    3
}
async fn async_multi(a: i32, b: i32) -> (i32, i32) {
    (a * 2, b * 3)
}

fn assert_func<Args, T>(m: impl Func<Args, T>) {}
fn assert_fn<Args>(m: impl Fn<Args>) {}
fn assert_func_ok<Args, T, Output, E>(m: impl Func<Args, T, Output = Result<Output, E>>) {}
fn assert_fn_ok<Args, T, E>(m: impl Fn<Args, Output = Result<T, E>>) {}
fn assert_func_some<Args, T, Output>(m: impl Func<Args, T, Output = Option<Output>>) {}
fn assert_fn_some<Args, T>(m: impl Fn<Args, Output = Option<T>>) {}

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        ($head, compose!($($tail),+))
    };
}

macro_rules! func {
    ( $head:expr, $($tail:expr), +) => {
        Function(($head, compose!($($tail),+)), PhantomData::default())
    };
}

fn main() {
    assert_func((foo, foo));
    assert_func((foo, test));
    assert_func((test, plus));
    assert_func((plus, plus));
    assert_func((multiply, plus));
    assert_func((multiply, (plus, plus)));
    assert_func((plus, (plus, plus)));

    assert_func_ok((error_in, plus));
    assert_func_ok((errors, multiply));
    assert_func_ok((error_in, (plus, plus)));

    assert_func_some((optional, (times, optional)));
    assert_func_some((optional, (times, times)));
    assert_func_some((optional, (error_in, optional)));

    assert_fn(func!(plus, plus, plus));
    assert_fn(func!(
        plus, plus, plus, plus, plus, plus, plus, plus, plus, plus, plus, plus, plus, plus, plus,
        plus, plus, plus, plus, plus, plus, plus
    ));
    assert_fn_ok(func!(error_in, plus, plus));
    assert_fn_some(func!(optional, times, optional));
    assert_fn_some(func!(optional, times, times));
    assert_fn_some(func!(optional, error_in, optional));

    /*
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
        map_some(optional, times);
        map_some(map_some(optional, times), optional);
        map_some(map_some(optional, times), times);
        map_some(map_some(optional, error_in), optional);
        let _fut = async {
            map_async(async_test, plus).call(()).await;
            map_async(async_multi, multiply).call((3, 3)).await;
            let m = map_async(async_multi, async_multi);
            m.call((3, 4)).await
            /*
            map_async(map_async(async_multi, async_multi), multiply)
                .call((3, 3))
                .await;
            */
        };
    */
}
