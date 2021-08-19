//! A Cranelift backend for Flycatcher's compiler.

use cranelift::prelude::*;
use cranelift_codegen::binemit::{NullStackMapSink, NullTrapSink};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use flycatcherc::{FlycatcherFrontend, FlycatcherType, VariableType};
use std::io::prelude::*;
use std::path::Path;
pub use target_lexicon::{self, Triple};

/// A Cranelift backend for Flycatcher's compiler.
pub struct CraneliftBackend {

    /// The target triple that the CraneliftBackend will compile to.
    pub target: Triple,

    /// The path to output an object file.
    pub out_file: String,

    /// A list of variables declared in the module.
    variables: Vec<String>,

}

impl CraneliftBackend {

    /// Initializes a CraneliftBackend instance.
    pub fn new(target: Triple, out_file: String) -> Self {
        Self {
            target,
            out_file,
            variables: vec![]
        }
    }

    /// Converts a FlycatcherType to a Cranelift type.
    fn convert_fctype(&self, t: FlycatcherType) -> Type {
        match t {
            FlycatcherType::Boolean => types::B1,
            FlycatcherType::Size => types::I64,
            FlycatcherType::Float64 => types::F64,
            _ => panic!("This type is unsupported by the Cranelift backend.")
        }
    }

    /// Compiles HIR from a FlycatcherFrontend into an object file.
    pub fn compile(&mut 
        self, frontend: FlycatcherFrontend) -> bool {
        // Initialize a flag builder.
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        let isa = cranelift_codegen::isa::lookup(self.target.clone())
            .unwrap()
            .finish(settings::Flags::new(flag_builder));
        
        let mut module = ObjectModule::new(ObjectBuilder::new(
            isa,
            [1, 2, 3, 4, 5, 6, 7, 8],
            default_libcall_names()
        ).unwrap());

        let mut ctx = module.make_context();
        let mut func_ctx = FunctionBuilderContext::new();

        let sig_main = module.make_signature();
        let func_main = module
            .declare_function("WinMain", Linkage::Export, &sig_main)
            .unwrap();

        ctx.func.signature = sig_main;
        ctx.func.name = ExternalName::user(0, func_main.as_u32());

        {
            let mut bcx = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            
            // Declare variables from the frontend
            for item in frontend.symbols {
                let v = Variable::new(self.variables.len());
                
                let fctype = match item.1 {
                    VariableType::Declared(t) => t,
                    VariableType::Defined(t, ..) => t
                };

                bcx.declare_var(v, self.convert_fctype(fctype));
                self.variables.push(item.0);
            }

            let block0 = bcx.create_block();
            bcx.switch_to_block(block0);

            bcx.ins().return_(&[]);
            bcx.seal_all_blocks();
        }

        let mut trap_sink = NullTrapSink {};
        let mut stack_map_sink = NullStackMapSink {};

        module
            .define_function(func_main, &mut ctx, &mut trap_sink, &mut stack_map_sink)
            .unwrap();
        
        module.clear_context(&mut ctx);

        let o = module.finish();
        let res = o.emit().unwrap();

        //std::fs::write(self.out_file.clone(), res).unwrap();
        let mut f = std::fs::File::create(self.out_file.clone()).unwrap();
        f.write_all(&res).unwrap();
        
        true
    }

}