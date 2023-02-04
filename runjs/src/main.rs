// main.rs

// use relative paths to import from ./run.rs
mod utils;

use clap::Parser;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::Extension;
use std::rc::Rc;

#[op]
fn op_run() -> Result<(), AnyError> {
    utils::run();
    Ok(())
}

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}
#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}
#[op]
fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[derive(Parser)]
struct Cli {
    // Note: PathBuf is like a String but for file system paths that work cross-platform.
    path: std::path::PathBuf,
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder("runjs")
        .ops(vec![
            op_run::decl(),
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./runtime/main.js");
    js_runtime
        .execute_script("[runjs:runtime.js]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let args = Cli::parse();
    println!("Looking for {}", args.path.display());

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js(args.path.to_str().unwrap())) {
        eprintln!("error: {}", error);
    }
}
