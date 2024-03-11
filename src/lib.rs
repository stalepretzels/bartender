use neon::prelude::*;
use rustrict::CensorStr;

fn censor(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    Ok(cx.string(&input.censor()))
}

fn censor_sx(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    Ok(cx.string(&censor))
}

fn censor_zea(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    Ok(cx.string(&censor))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("censor_std", censor)?;
    cx.export_function("censor_sx", censor_sx)?;
    cx.export_function("censor_zea", censor_zea)?;
    Ok(())
}