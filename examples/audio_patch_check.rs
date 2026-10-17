mod host {pub const MISSING_TEXT: &[(&str,&str)] = &[];}
mod proto {pub fn joaat(_: &str)->u32 {0}}
#[path="../src/patch.rs"] mod patch;
fn main(){let p=std::env::args().nth(1).unwrap();let b=std::fs::read(p).unwrap();let patched=patch::patch(&b,&[]).unwrap();wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all()).validate_all(&patched).unwrap();assert!(patch::is_patched(&patched));println!("PASS: full pinned game.wasm reencoded and validated; diagnostic imports and function index shifts are valid; original {} bytes, instrumented {} bytes",b.len(),patched.len());}
