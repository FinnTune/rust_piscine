pub fn add_curry(i: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + i)
}

pub fn twice<F>(func: F) -> Box<dyn Fn(i32) -> i32>
    where
        F: Fn(i32) -> i32 + 'static,
{
    Box::new(move |x| func(func(x)))
}
