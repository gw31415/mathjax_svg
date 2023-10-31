use std::sync::Mutex;

use rusty_v8 as v8;
use v8::{Context, Global, OwnedIsolate};

fn create_origin<'s>(
    scope: &mut v8::HandleScope<'s>,
    filename: impl AsRef<str>,
    is_module: bool,
) -> v8::ScriptOrigin<'s> {
    let name: v8::Local<'s, v8::Value> = v8::String::new(scope, filename.as_ref()).unwrap().into();
    v8::ScriptOrigin::new(scope, name, 0, 0, false, 0, name, false, false, is_module)
}

fn module_callback<'s>(
    _context: v8::Local<'s, v8::Context>,
    _name: v8::Local<'s, v8::String>,
    _arr: v8::Local<'s, v8::FixedArray>,
    module: v8::Local<'s, v8::Module>,
) -> Option<v8::Local<'s, v8::Module>> {
    Some(module)
}

static INITIALIZED: Mutex<bool> = Mutex::new(false);

/// Math to Svg Converter
pub struct Converter {
    isolate: OwnedIsolate,
    context: Global<Context>,
}

const FUNC_ID: &str = "cqbuwfsowtpq"; // random character string

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

impl Converter {
    /// Initialization: JIT compilation and function object registration
    pub fn new() -> Self {
        if !*INITIALIZED.lock().unwrap() {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
            *INITIALIZED.lock().unwrap() = true;
        }

        let mut isolate = v8::Isolate::new(Default::default());
        let global_context;
        {
            let handle_scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(handle_scope);
            global_context = Global::new(handle_scope, context);
            {
                // JIT compilation and function object registration

                // Generate scope
                let context = v8::Local::new(handle_scope, context);
                let scope = &mut v8::ContextScope::new(handle_scope, context);

                // Load js file
                let code = include_str!("../js/out/index.js");
                let source = v8::String::new(scope, code).unwrap();
                let origin = create_origin(scope, "index.js", true);
                let source = v8::script_compiler::Source::new(source, Some(&origin));
                let module = v8::script_compiler::compile_module(scope, source).unwrap();

                // Instantiate module
                module.instantiate_module(scope, module_callback).unwrap();
                module.evaluate(scope).unwrap();

                let key = v8::String::new(scope, "default").unwrap();
                let obj = module
                    .get_module_namespace()
                    .to_object(scope)
                    .unwrap()
                    .get(scope, key.into())
                    .unwrap();
                let key = v8::String::new(scope, FUNC_ID).unwrap().into();
                context.global(scope).set(scope, key, obj);
            }
        }
        Self {
            isolate,
            context: global_context,
        }
    }

    /// Convert a math string to Svg
    pub fn convert_to_svg(&mut self, latex: impl AsRef<str>) -> String {
        let handle_scope = &mut v8::HandleScope::new(&mut self.isolate);
        let context = v8::Local::new(handle_scope, self.context.clone());
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        let key = v8::String::new(scope, FUNC_ID).unwrap().into();
        let obj = context.global(scope).get(scope, key).unwrap();
        let func = v8::Local::<v8::Function>::try_from(obj).unwrap();
        let args = [v8::String::new(scope, latex.as_ref()).unwrap().into()];
        let result = func.call(scope, obj, &args).unwrap();
        result.to_rust_string_lossy(scope)
    }
}
