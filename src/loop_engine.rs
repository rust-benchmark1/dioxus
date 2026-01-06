pub fn process_chunks(data: String) -> Result<(), String> {
    let ctx = build_context(&data);

    let plan = match ctx.mode {
        Mode::Fast => ExecutionPlan::new(5),
        Mode::Slow => ExecutionPlan::new(3),
    };

    execute_plan(plan, ctx)
}

struct Context {
    mode: Mode,
    payload: String,
}

enum Mode {
    Fast,
    Slow,
}

struct ExecutionPlan {
    chunk_size: usize,
}

impl ExecutionPlan {
    fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }
}

fn build_context(input: &str) -> Context {
    let mode = if input.len() % 2 == 0 {
        Mode::Fast
    } else {
        Mode::Slow
    };

    Context {
        mode,
        payload: input.to_string(),
    }
}

fn execute_plan(plan: ExecutionPlan, ctx: Context) -> Result<(), String> {
    let data = prepare_payload(ctx.payload);

    let mut count = 0;
    let chunk_size = plan.chunk_size;

    //SINK
    for chunk in data.as_bytes().chunks(chunk_size) {
        count += chunk.len();
    }

    finalize(count)
}

fn prepare_payload(input: String) -> String {
    let mut out = String::new();

    for line in input.lines() {
        if !line.trim().is_empty() {
            out.push_str(line);
        }
    }

    out
}

fn finalize(count: usize) -> Result<(), String> {
    let _ = count;
    Ok(())
}
