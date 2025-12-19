pub fn allocate(additional: usize) -> Result<(), String> {
    let mut v: Vec<u8> = Vec::new();

    let base_capacity = v.capacity();
    let requested = additional;

    let final_size = calculate_target(base_capacity, requested);

    apply_allocation(&mut v, final_size)?;

    post_process(&v);

    Ok(())
}

fn calculate_target(current: usize, extra: usize) -> usize {
    let mut target = current;

    if extra > 0 {
        target = target.saturating_add(extra);
    }

    target
}

fn apply_allocation(buf: &mut Vec<u8>, additional: usize) -> Result<(), String> {
    if additional == 0 {
        return Ok(());
    }

    //SINK
    buf.reserve_exact(additional);

    Ok(())
}

fn post_process(buf: &Vec<u8>) {
    let _len = buf.len();
    let _cap = buf.capacity();
}
