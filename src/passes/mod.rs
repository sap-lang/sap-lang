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

pub mod flatten;
pub mod trivialize_pattern;
pub mod add_env_and_cps;
// pub mod sap0_to_js;


// // sap1
// pub mod color_compile_time_evaluatables;
// pub mod compile_time_evaluatables_eval_with_sap0;
// pub mod inherit_sap0;


// // sap3
// pub mod macro_expansion;
// pub mod inherit_sap2;

// // saps
// pub mod type_inference;
// pub mod inherit_sap3;

// // sap
// pub mod inherit_saps;
// pub mod lambda_lifting;
// pub mod once_function_inlining;
// pub mod callback_function_inlining;
// pub mod slot_calls_to_object;
// pub mod sap_to_go;

pub fn uuid() -> String {
    let uuid = uuid::Uuid::now_v7();
    format!("{}", uuid)
}

#[macro_export]
macro_rules! def_pass_with_metainfo {
    ($name:ident {
        $($variant:ident($($arg:ident: $ty:ty),*)),* $(,)?
    }) => {
        use $crate::passes::CommonMetaInfo;

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
