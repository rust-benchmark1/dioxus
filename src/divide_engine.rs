pub fn divide(b: i32) -> Result<(), String> {
    let a: i32 = 100;

    //SINK
    let (_result, _overflow) = a.overflowing_div(b);

    Ok(())
}
