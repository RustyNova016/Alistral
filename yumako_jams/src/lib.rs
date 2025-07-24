pub mod client;
pub mod error;
pub mod json;
pub mod modules;
pub mod radio_item;
pub mod radio_stream;
pub mod radio_variables;
// pub mod repository; // TODO

pub use crate::error::Error;
pub use crate::radio_stream::RadioResult;
pub use crate::radio_stream::RadioStream;

// #[test]
// fn stream_compile_test() {
//     let stream = json!(
//             {
//         "name": "Test Radio",
//         "stack": [
//             {
//                 "step_type": "listen_seeder",
//                 "id": "listen_seeder"
//             },
//             {
//                 "step_type": "minimum_listen_filter",
//                 "id": "minimum_listen_filter",
//                 "variables": {
//                     "count": 5
//                 }
//             }
//         ],
//         "inputs": {
//             "minimum_listens": "minimum_listen_filter.count"
//         }
//     }
//         );

//     let stream: Radio = serde_json::from_value(stream).unwrap();
//     let binding = Default::default();
//     let _stream= stream.to_stream(&binding).unwrap();
// }
