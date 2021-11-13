use serde_json::{json, Value};

pub fn success(params: Value) -> Value {
    json!({
        "status": "success",
        "data": params.get("data").unwrap()
    })
}

pub fn failure(params: Value) -> Value {
    json!({
        "status": "error",
        "error": {"code": params.get("code").unwrap(), "messages":[params.get("data").unwrap()] }
    })
}
