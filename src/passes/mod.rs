use sap_parser::diagnostics::Diagnostic;

#[derive(Debug, Clone, Copy)]
pub enum CompileTimeExecutionColor {
    WhitePureRuntime,
    GrayByParameter,
    BlackPureCompileTime,
}

#[derive(Clone, Copy)]
pub struct CommonMetaInfo {
    diagnostic: Diagnostic,
    file_name: Option<&'static str>,
    compile_time_execution_color: Option<CompileTimeExecutionColor>,
}

impl std::fmt::Debug for CommonMetaInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}:{}",
            self.file_name.unwrap_or("<unknown>"),
            self.diagnostic.start_line,
            self.diagnostic.start_col
        ))
    }
}

impl CommonMetaInfo {
    pub fn new(diagnostic: Diagnostic, file_name: Option<&'static str>) -> Self {
        Self {
            diagnostic,
            file_name,
            compile_time_execution_color: None,
        }
    }
}

// sap0

pub mod flatten_and_ops_to_apply;
// pub mod pattern_to_ifs; // side effect
// pub mod to_anf;

// pub mod topological_sort;
// pub mod build_env;

// pub mod color_slots;
// pub mod color_function_implicit_params;
// pub mod decolor_calling_function_implicit_params_with_auto_params_from_env;
// pub mod to_decl_and_set;
// pub mod flatten_anf_with_mangling_decls;
// pub mod color_cofunctions;
// pub mod cofunction_to_state_machine;
// pub mod sap0_to_js;

// pub mod create_function_replica_on_call;

// // sap1
// pub mod color_compile_time_evaluatables;
// pub mod compile_time_evaluatables_eval_with_sap0;
// pub mod inherit_sap0;

// // sap2
// pub mod color_macros;
// pub mod first_order_macro_expansion_eval_with_sap1;
// pub mod inherit_sap1;

// // sap3
// pub mod secound_order_macro_expansion_eval_with_sap2;
// pub mod inherit_sap2;

// // saps
// pub mod type_inference;
// pub mod escape_analysis;
// pub mod inherit_sap3;

// // sap
// pub mod inherit_saps;
// pub mod lambda_lifting;
// pub mod once_function_inlining;
// pub mod callback_function_inlining;
// pub mod slot_calls_to_object;
// pub mod sap_to_tinygo;
// // pub mod sap_to_mlir;

pub fn uuid() -> String {
    let uuid = uuid::Uuid::now_v7();
    format!("{}", uuid)
}

#[macro_export]
macro_rules! def_pass_with_metainfo {
    ($name:ident {
        $($variant:ident($($arg:ident: $ty:ty),*)),* $(,)?
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub inner: Inner,
            pub info: CommonMetaInfo,
        }

        impl $name {
            $(
                #[allow(non_snake_case)]
                pub fn $variant($($arg: $ty),*, info: CommonMetaInfo) -> Self {
                    Self {
                        inner: Inner::$variant($($arg),*),
                        info,
                    }
                }
            )*
        }

        #[derive(Debug, Clone)]
        pub enum Inner {
            $(
                $variant($($ty),*),
            )*
        }
    };
}
