pub fn time<Function, FunctionOutput>(f: Function) -> (f64, FunctionOutput)
where
    Function: FnOnce() -> FunctionOutput,
{
    use std::time::Instant;

    let now = Instant::now();
    let res = f();
    let elapsed = now.elapsed().as_secs_f64();
    return (elapsed, res);
}
