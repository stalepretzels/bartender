use rustrict::CensorStr;
use neon::prelude::*;

fn censor(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsString>(0)?; // Get the first argument as a JsString
    let rust_string: String = arg0.value(&mut cx); // Convert JsString to Rust string

    Ok(cx.string(rust_string.censor()))
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("censor", censor)?;
    Ok(())
}
