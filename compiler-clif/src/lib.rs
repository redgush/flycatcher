//! A Cranelift backend for Flycatcher's compiler.

use cranelift::prelude::*;
use cranelift_codegen::binemit::{NullStackMapSink, NullTrapSink};
use cranelift_codegen::ir::MemFlags;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_module::{DataContext, default_libcall_names, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use flycatcherc::{FlycatcherFrontend, FlycatcherType, Hir, HirMeta, VariableType};
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

    /// The ID of the LibC `malloc` function.
    malloc: Option<FuncId>,

}

impl CraneliftBackend {

    /// Initializes a CraneliftBackend instance.
    pub fn new(target: Triple, out_file: String) -> Self {
        Self {
            target,
            out_file,
            variables: vec![],
            malloc: None,
        }
    }

    /// Converts a FlycatcherType to a Cranelift type.
    fn convert_fctype(&self, t: FlycatcherType) -> Type {
        match t {
            FlycatcherType::Boolean => types::B1,
            FlycatcherType::Size => types::I64,
            FlycatcherType::Float64 => types::F64,
            FlycatcherType::NullString => Type::triple_pointer_type(&self.target),
            _ => panic!("This type is unsupported by the Cranelift backend.")
        }
    }

    /// Recursively converts a Flycatcher object into its Cranelift representation.
    fn convert_expression(&mut self, hir: HirMeta, context: &mut FunctionBuilder, module: &mut ObjectModule) -> Value {
        match hir.item {
            Hir::Boolean(b) => context.ins().iconst(types::B1, match b {
                true => 1,
                false => 0,
            }),
            Hir::Integer(i) => context.ins().iconst(types::I64, i),
            Hir::Float(f) => context.ins().f64const(f),
            Hir::Add(l, r) => {
                let left = self.convert_expression(*l, context, module);
                let right = self.convert_expression(*r, context, module);
                context.ins().iadd(
                    left,
                    right,
                )
            },
            Hir::Subtract(l, r) => {
                let left = self.convert_expression(*l, context, module);
                let right = self.convert_expression(*r, context, module);
                context.ins().isub(
                    left,
                    right,
                )
            },
            Hir::Multiply(l, r) => {
                let left = self.convert_expression(*l, context, module);
                let right = self.convert_expression(*r, context, module);
                context.ins().imul(
                    left,
                    right,
                )
            },
            Hir::Divide(l, r) => {
                let left = self.convert_expression(*l, context, module);
                let right = self.convert_expression(*r, context, module);
                context.ins().fdiv(
                    left,
                    right,
                )
            },
            Hir::Named(n) => {
                // Resolve named variable.
                let i = self.variables.iter().position(|x| x == &n).unwrap();
                let v = Variable::with_u32(i as u32);

                context.use_var(v)
            },
            Hir::NullString(s) => {
                let malloc = module.declare_func_in_func(self.malloc.unwrap(), &mut context.func);
                
                let tmp_bytesize = context.ins().iconst(Type::triple_pointer_type(&self.target), s.len() as i64 + 1);
                
                let call = context.ins().call(malloc, &[tmp_bytesize]);
                let addr = context.inst_results(call)[0];

                let v;
                if let Some(i) = self.variables.iter().position(|x| x == "^") {
                    v = Variable::new(i);
                } else {
                    v = Variable::new(self.variables.len());
                    // A temporary invalid variable name (^) is used to store the address of the
                    // string, temporarily.
                    self.variables.push("^".into());

                    context.declare_var(v, Type::triple_pointer_type(&self.target));
                }

                context.def_var(v, addr);

                let mut offset = 0;
                for byte in s.as_bytes() {
                    let byte = context.ins().iconst(Type::int(8).unwrap(), *byte as i64);
                    let var = context.use_var(v);

                    context.ins().store(
                        MemFlags::new(),
                        byte,
                        var,
                        offset
                    );
                    offset += 1;
                }

                {
                    // Insert null byte at end of string
                    let byte = context.ins().iconst(Type::int(8).unwrap(), 0);
                    let var = context.use_var(v);

                    context.ins().store(
                        MemFlags::new(),
                        byte,
                        var,
                        offset
                    );
                }

                context.use_var(v)
                
                /*
                let data = module.declare_data(
                    "my_str",
                    Linkage::Export,
                    true,
                    true
                );

                let data_ctx = DataContext::new();
                //data_ctx.define(s.);

                let data = module.declare_anonymous_data(true, true);
                */
            },
            _ => panic!("unexpected HIR object at backend"),
        }
    }

    /// Compiles HIR from a FlycatcherFrontend into an object file.
    pub fn compile(&mut self, frontend: FlycatcherFrontend) -> bool {
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

        let mut sig_malloc = module.make_signature();
        sig_malloc.params.push(AbiParam::new(Type::triple_pointer_type(&self.target)));
        sig_malloc.returns.push(AbiParam::new(Type::triple_pointer_type(&self.target)));
        let func_malloc = module
            .declare_function("malloc", Linkage::Import, &sig_malloc)
            .unwrap();

        let mut sig_printf = module.make_signature();
        sig_printf.params.push(AbiParam::new(Type::triple_pointer_type(&self.target)));
        sig_printf.returns.push(AbiParam::new(Type::triple_pointer_type(&self.target)));
        let func_printf = module
            .declare_function("printf", Linkage::Import, &sig_printf)
            .unwrap();
        
        self.malloc = Some(func_malloc);

        let sig_main = module.make_signature();
        let func_main = module
            .declare_function("WinMain", Linkage::Export, &sig_main)
            .unwrap();

        ctx.func.signature = sig_main;
        ctx.func.name = ExternalName::user(0, func_main.as_u32());

        {
            let mut bcx = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            
            // Declare variables from the frontend
            for item in &frontend.symbols {
                let v = Variable::new(self.variables.len());
                
                let fctype = match item.1 {
                    VariableType::Declared(t) => t,
                    VariableType::Defined(t, ..) => t
                };

                bcx.declare_var(v, self.convert_fctype(*fctype));
                self.variables.push(item.0.to_string());
            }

            let block0 = bcx.create_block();
            bcx.switch_to_block(block0);

            for item in frontend.hir {
                match item.item {
                    Hir::Set(n, b) => {
                        let name = match n.item {
                            Hir::Named(v) => v,
                            _ => panic!("Unsupported variable name. (this shouldn't occur)")
                        };

                        let i = self.variables.iter().position(|x| x == &name).unwrap();
                        let v = Variable::with_u32(i as u32);

                        let val = self.convert_expression(*b.clone(), &mut bcx, &mut module);
                        bcx.def_var(v, val);

                        let t = b.item.get_type(&frontend.symbols);
                        match t {
                            FlycatcherType::NullString => {
                                let printf = module.declare_func_in_func(func_printf, &mut bcx.func);
                                let addr = bcx.use_var(v);
                                bcx.ins().call(printf, &[addr]);
                            },
                            _ => {}
                        }
                    },
                    _ => panic!("Unsupported HIR object at backend, during function init.")
                }
            }

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