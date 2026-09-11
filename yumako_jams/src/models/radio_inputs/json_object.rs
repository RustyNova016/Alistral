use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonObject(pub HashMap<String, Value>);

impl JsonObject {

}

// fn merge_json(a: &mut Value, b: Value) {
//     if let Value::Object(a) = a {
//         if let Value::Object(b) = b {
//             for (k, v) in b {
//                 match v {

//                 }
//             } 

//             return;
//         }
//     }

//     *a = b;
// }