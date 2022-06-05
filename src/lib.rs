use boa_engine::{Context, JsValue as BoaJsValue};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(source: &str) -> Result<JsValue, JsValue> {
    let mut context = Context::default();
    context
        .eval(source)
        .map_err(|res: BoaJsValue| boa_to_js(&res, &mut context))
        .map(|res: BoaJsValue| boa_to_js(&res, &mut context))
}

fn boa_to_js(value: &BoaJsValue, context: &mut Context) -> JsValue {
    match value {
        BoaJsValue::Null => JsValue::NULL,
        BoaJsValue::Undefined => JsValue::UNDEFINED,
        BoaJsValue::Boolean(value) => JsValue::from(*value),
        BoaJsValue::String(value) => JsValue::from_str(value),
        BoaJsValue::Rational(value) => JsValue::from(*value),
        BoaJsValue::Integer(value) => JsValue::from(*value),
        BoaJsValue::BigInt(value) => JsValue::bigint_from_str(&value.to_string()),
        BoaJsValue::Object(_) => JsValue::from_serde(&value.to_json(context).unwrap()).unwrap(),
        BoaJsValue::Symbol(value) => JsValue::symbol(value.description().as_deref()),
    }
}
