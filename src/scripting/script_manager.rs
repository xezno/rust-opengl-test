use std::thread;

use netcorehost::{nethost, pdcstr};

// ============================================================================
//
// script_manager.rs
//
// Purpose: Manages wren scripting through ruwren
//
// ============================================================================

pub struct ScriptManager {}

impl ScriptManager {
    pub fn new() -> ScriptManager {
        thread::Builder::new()
            .name("dotnet thread".to_string())
            .spawn(|| 'thread: loop {
                let hostfxr = nethost::load_hostfxr().unwrap();
                let context = hostfxr
                    .initialize_for_runtime_config(pdcstr!(
                        "content/addons/test/bin/Release/net6.0/win-x64/test.runtimeconfig.json"
                    ))
                    .unwrap();
                let fn_loader = context
                    .get_delegate_loader_for_assembly(pdcstr!(
                        "content/addons/test/bin/Release/net6.0/win-x64/test.dll"
                    ))
                    .unwrap();
                let hello = fn_loader
                    .get_function_pointer_with_default_signature(
                        pdcstr!("Test.Program, Test"),
                        pdcstr!("Hello"),
                    )
                    .unwrap();
                let result = unsafe { hello(std::ptr::null(), 0) };
                println!("C# exited with {}", result);

                break 'thread;
            })
            .unwrap();
        ScriptManager {}
    }
}
