use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::{ast::SapAST, backend::compile_inner};

pub fn compile_library(file: String, ast: Vec<SapAST>) -> String {
    let body = ast
        .into_iter()
        .map(compile_inner)
        .collect::<Vec<String>>()
        .join(";\n");

    format!(
        "
// {file}

import {{
    __init_prelude_env__,
    __equals__,
    __new_binding__,
    __new_binding_cont__,
    __get_binding__,
    __new_slot_binding__,
    __is_return__,
    __extract_return__,
    __return_value__,
    __call__,
    __yield_child__,
}} from './js_prelude/prelude.js';

let __ENV__ = {{ }};
let __PRELUDE__ = {{ }};

__ENV__.__proto__ = __PRELUDE__;
__init_prelude_env__(__PRELUDE__);

const __CENV__ = __ENV__;
__CENV__.__proto__ = __PRELUDE__;
{body}
"
    )
}

pub fn compile_executable(file: String, ast: Vec<SapAST>) -> String {
    let body = compile_library(file, ast);

    format!(
        "
{body}

async function __main__() {{
let main = (function*(){{
    return yield* __ENV__['main'](__CENV__);
}});

let main_process = main();
try {{
    let cont = undefined;
    let ret = undefined;
    while (1) {{
        ret = main_process.next(cont)
        if (ret.done) {{
            break;
        }}
        
        if ( ret.value instanceof Promise ) {{
            cont = await ret.value;
        }} else {{
            cont = ret.value;
        }}
    }}
}} catch (e) {{
    console.error(e);
    return;
}}
}}

await __main__();"
    )
}

pub fn compile(file: String, ast: Vec<SapAST>, executable: bool) -> String {
    if executable {
        compile_executable(file, ast)
    } else {
        compile_library(file, ast)
    }
}

/// a.b -> a/b.sap or a/b/b.sap
/// a-> a.sap or a/a.sap
fn module_name_to_path_name(module_name: &str) -> PathBuf {
    let module_parts = module_name.split('.').collect::<Vec<&str>>();
    let final_name = *module_parts.last().unwrap();

    let mut path1 = module_parts
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("/");

    let mut path2 = path1.clone();
    path2 += "/";
    path2 += final_name;
    path2 += ".sap";
    path1 += ".sap";

    if Path::new(&path2).exists() {
        PathBuf::from(path2)
    } else if Path::new(&path1).exists() {
        PathBuf::from(path1)
    } else {
        panic!("Module {module_name} neither {path1} nor {path2} found");
    }
}

fn compile_file(module_name: &str, output_dir: &Path, executable: bool) {
    let path = module_name_to_path_name(module_name);
    let source = std::fs::read_to_string(&path).unwrap();
    let ast = crate::parser::parse(&source);
    let code = compile(module_name.to_string(), ast, executable);
    let mut file = std::fs::File::create(output_dir.join(module_name.to_string() + ".js")).unwrap();
    file.write_all(code.as_bytes()).unwrap();
}

fn compile_folder(module_name: &str, output_dir: &Path, executable: bool) {
    let module_file_path = module_name_to_path_name(module_name);
    let module_folder_path = module_file_path.parent().unwrap();
    for entry in std::fs::read_dir(module_folder_path).unwrap() {
        let entry = entry.unwrap();
        let sub_module_path = entry.path();
        let sub_module_name = sub_module_path.file_stem().unwrap().to_str().unwrap();
        if sub_module_path.is_file() && sub_module_name != module_name.split('.').last().unwrap() {
            compile_file(
                &format!("{module_name}.{sub_module_name}"),
                output_dir,
                false,
            );
        } else if sub_module_path.is_dir() {
            compile_folder(
                &format!("{module_name}.{sub_module_name}"),
                output_dir,
                false,
            );
        } else {
            compile_file(module_name, output_dir, executable);
        }
    }
}

pub fn compile_module(module_name: &str, output_dir: &Path, executable: bool) {
    let path = Path::new(module_name);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    } else {
        // remove all files in the output directory
        for entry in std::fs::read_dir(output_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                std::fs::remove_file(&path).unwrap();
            } else {
                std::fs::remove_dir_all(&path).unwrap();
            }
        }
    }
    let prelude_src = Path::new("./js_prelude");
    let prelude_dst = output_dir.join("js_prelude");
    if prelude_src.exists() {
        std::fs::create_dir_all(&prelude_dst).unwrap();
        for entry in std::fs::read_dir(prelude_src).unwrap() {
            let entry = entry.unwrap();
            let src_path = entry.path();
            let dst_path = prelude_dst.join(entry.file_name());
            std::fs::copy(&src_path, &dst_path).unwrap();
        }
    }

    if path.with_extension("sap").exists() {
        compile_file(module_name, output_dir, executable);
    } else {
        compile_folder(module_name, output_dir, executable);
    }
}

#[test]
fn test_compile_module() {
    let module_name = "test_module";
    let output_dir = Path::new("test_output");
    let executable = true;
    compile_module(module_name, output_dir, executable);
}
