// ============================================================================
//
// script_manager.rs
//
// Purpose: Manages wren scripting through ruwren
//
// ============================================================================

use netcorehost::{nethost, pdcstr};
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    thread,
};

macro_rules! csharp_str {
    ($s:expr) => {
        CString::new($s).unwrap().into_raw()
    };
}

macro_rules! csharp_str_to_string {
    ($s:expr) => {
        unsafe { CStr::from_ptr($s as *const c_char).to_str().unwrap() }
    };
}

pub struct ScriptManager {}

#[repr(C)]
struct Test {
    pub name: *mut c_char,
    pub age: i32,
    pub callback: extern "C" fn(arg: *mut c_void),
}

extern "C" fn callback(arg: *mut c_void) {
    println!("stringy: {}", csharp_str_to_string!(arg));
}

impl ScriptManager {
    pub fn new() -> ScriptManager {
        thread::Builder::new()
            .name(".NET Scripting thread".to_string())
            .spawn(|| 'thread: loop {
                let hostfxr = nethost::load_hostfxr().unwrap();
                let context = hostfxr
                    .initialize_for_runtime_config(pdcstr!(
                        "content/addons/test/bin/Release/net5.0/win-x64/test.runtimeconfig.json"
                    ))
                    .unwrap();
                let fn_loader = context
                    .get_delegate_loader_for_assembly(pdcstr!(
                        "content/addons/test/bin/Release/net5.0/win-x64/test.dll"
                    ))
                    .unwrap();
                let hello = fn_loader
                    .get_function_pointer_with_default_signature(
                        pdcstr!("Test.Program, Test"),
                        pdcstr!("Hello"),
                    )
                    .unwrap();

                let test = Test {
                    name: csharp_str!("Alex"),
                    age: 32,
                    callback: callback,
                };
                let result = unsafe { hello(&test as *const Test as *const c_void, 0) };
                println!("C# exited with {}", result);

                break 'thread;
            })
            .unwrap();
        ScriptManager {}
    }
}
