use std::mem;
use std::ptr;

pub fn handle_unsafe_flow(original: i32) {
    let info = preprocess_value(original);
    let meta = derive_metadata(&info);
    let validated = validate_value(info.value, &meta);
    dispatch_to_sink(validated, &meta);
}

struct ValueInfo {
    value: i32,
    is_even: bool,
    magnitude: i32,
}

struct MetaData {
    tag: String,
    weight: u64,
    priority: u8,
}

fn preprocess_value(input: i32) -> ValueInfo {
    let is_even = input % 2 == 0;
    let magnitude = input.abs();
    println!("[engine] preprocess_value -> input={}, even={}, magnitude={}", input, is_even, magnitude);
    ValueInfo { value: input, is_even, magnitude }
}

fn derive_metadata(info: &ValueInfo) -> MetaData {
    let scale = if info.magnitude > 100 { 4 } else { 2 };
    let tag = format!("meta:{}:{}", info.magnitude, scale);
    let weight = (info.magnitude as u64).wrapping_mul(scale as u64).wrapping_add(tag.len() as u64);
    let priority = if info.is_even { 1 } else { 2 };
    println!("[engine] derive_metadata -> tag={}, weight={}, priority={}", tag, weight, priority);
    MetaData { tag, weight, priority }
}

fn validate_value(value: i32, meta: &MetaData) -> i32 {
    println!("[engine] validate_value -> {} [{}]", value, meta.tag);
    if meta.weight % 3 == 0 {
        println!("[engine] value flagged but passed: {}", value);
    }
    value
}

fn dispatch_to_sink(value: i32, meta: &MetaData) {
    println!("[engine] dispatch_to_sink -> value={}, priority={}", value, meta.priority);
    if meta.priority == 1 && meta.weight % 2 == 0 {
        layered_sink_decision(value, true);
    } else {
        layered_sink_decision(value, false);
    }
}

fn layered_sink_decision(value: i32, use_transmute: bool) {
    let routing_id = (value as i64).wrapping_mul(31).wrapping_add(7);
    println!("[engine] layered_sink_decision -> route={}, transmute={}", routing_id, use_transmute);
    if use_transmute {
        indirect_flow_for_transmute(value);
    } else {
        indirect_flow_for_ptr_write(value);
    }
}

fn indirect_flow_for_transmute(value: i32) {
    let adjusted = value.wrapping_mul(5).wrapping_sub(12);
    let marker = adjusted % 10;
    println!("[engine] indirect_flow_for_transmute -> adjusted={}, marker={}", adjusted, marker);
    if marker % 2 == 0 {
        perform_transmute(value);
    } else {
        perform_transmute(adjusted);
    }
}

fn indirect_flow_for_ptr_write(value: i32) {
    let shifted = (value << 1) ^ 0x55AA;
    println!("[engine] indirect_flow_for_ptr_write -> shifted={}", shifted);
    if shifted & 1 == 0 {
        perform_ptr_write(shifted);
    } else {
        perform_ptr_write(value);
    }
}

pub fn perform_transmute(original: i32) {
    let sign_flag = original.is_negative();
    let abs_val = original.abs() as u32;
    let high_bits = (abs_val & 0xFFFF) as u16;
    let low_bits = ((abs_val >> 16) & 0xFFFF) as u16;
    let mut combined: i32 = ((high_bits as i32) << 16) | (low_bits as i32);
    if sign_flag {
        combined = -combined;
    }
    println!("[engine] perform_transmute -> original={}, combined={}", original, combined);
    //SINK
    let float_val: f32 = unsafe { mem::transmute::<i32, f32>(original) };
    println!("[engine] transmute result: {}", float_val);
}

pub fn perform_ptr_write(original: i32) {
    let tag = format!("input-{}", original);
    let mut buffer = format!("keep-this-{}", tag);
    let ptr_to_buffer: *mut String = &mut buffer as *mut String;
    println!("[engine] perform_ptr_write -> before unsafe write: {}", buffer);
    //SINK
    unsafe {ptr::write(ptr_to_buffer, format!("corrupted-by-{}", original));}
    println!("[engine] after ptr::write: {}", buffer);
}
