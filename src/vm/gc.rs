use std::{borrow::Borrow, cell::RefCell};

use crate::{
    compiler::values::{FunctionObj, NativeFn, Obj, StrObj, Structs, StructsInstance, Value},
    vm::{DEBUG, VM},
};

#[allow(dead_code)]
impl VM {
    pub fn collect_garbage(&self) {
        if DEBUG {
            println!("-- Collecting Garbage");
        }

        let mut worklist = Vec::new();
        self.mark_root(worklist.as_mut());

        loop {
            match worklist.pop() {
                Some(obj) => self.blacken_obj(obj),
                None => break,
            }
        }

        self.sweep();

        if DEBUG {
            println!("-- Finished Collecting Garbage");
        }
    }

    fn mark_root<'a>(&'a self, worklist: &mut Vec<&'a Obj>) {
        for val in &self.stack {
            self.mark_obj(val, worklist);
        }

        self.mark_table(worklist);
    }

    fn mark_table<'a>(&'a self, worklist: &mut Vec<&'a Obj>) {
        for (_key, val) in &self.globals {
            self.mark_obj(val, worklist);
        }
    }

    fn mark_obj<'a>(&self, val: &'a Value, worklist: &mut Vec<&'a Obj>) {
        match val {
            Value::Obj(obj) => {
                // The is_marked check maybe unnecessary due to the fact that when I blacken, I
                // don't have any possiblity of cycles. Though I may save 1-2 iterations of cycle
                // for when I'm calling a function in the stack constantly. I don't need to blacken
                // it and check for it since I already know it is marked.
                match obj {
                    Obj::String(str) => {
                        let str_obj: &RefCell<StrObj> = str.borrow();

                        if DEBUG {
                            println!("mark {}", str_obj.borrow());
                        }
                        let mut str_obj = str_obj.borrow_mut();
                        str_obj.is_marked = if str_obj.is_marked { return } else { true };
                    }
                    Obj::Function(func) => {
                        let func_obj: &RefCell<FunctionObj> = func.borrow();
                        if DEBUG {
                            println!("mark {}", func_obj.borrow());
                        }
                        let mut func_obj = func_obj.borrow_mut();
                        func_obj.is_marked = if func_obj.is_marked { return } else { true };
                    }
                    Obj::NativeFn(nativ_func) => {
                        let native_func_obj: &RefCell<NativeFn> = nativ_func.borrow();
                        if DEBUG {
                            println!("mark {}", native_func_obj.borrow());
                        }
                        let mut native_func_obj = native_func_obj.borrow_mut();
                        native_func_obj.is_marked = if native_func_obj.is_marked {
                            return;
                        } else {
                            true
                        };
                    }
                    Obj::Structs(structs) => {
                        let struct_obj: &RefCell<Structs> = structs.borrow();
                        if DEBUG {
                            println!("mark {}", struct_obj.borrow());
                        }
                        let mut struct_obj = struct_obj.borrow_mut();
                        struct_obj.is_marked = if struct_obj.is_marked {
                            return;
                        } else {
                            true
                        };
                    }
                    Obj::Instance(instance) => {
                        let instance_obj: &RefCell<StructsInstance> = instance.borrow();
                        if DEBUG {
                            println!("mark {}", instance_obj.borrow());
                        }
                        let mut instance_obj = instance_obj.borrow_mut();
                        instance_obj.is_marked = if instance_obj.is_marked {
                            return;
                        } else {
                            true
                        };
                    }
                };

                worklist.push(obj);
            }
            _ => {}
        }
    }

    fn sweep(&self) {
        // If only there was a place for me to store my Obj, perhaps it might be faster, perhaps I
        // can have the opportunity to use this GC.
        // For now all I'll do is just log, if any, unmarked objs.
        // This implementation is still flawed too somehow.

        if DEBUG {
            println!("--- Unmarked Objs ---");
        }
        for val in &self.stack {
            self.print_unmarked_val(val)
        }
        for (_key, val) in &self.globals {
            self.print_unmarked_val(val)
        }
    }

    fn blacken_obj(&self, obj: &Obj) {
        match obj {
            Obj::String(obj) => {
                let str_obj: &RefCell<StrObj> = obj.borrow();

                if DEBUG {
                    println!("blacken {}", str_obj.borrow());
                }
            }
            Obj::NativeFn(obj) => {
                let native_fn: &RefCell<NativeFn> = obj.borrow();

                if DEBUG {
                    println!("blacken {}", native_fn.borrow());
                }
            }
            Obj::Function(obj) => {
                let fn_obj: &RefCell<FunctionObj> = obj.borrow();
                if DEBUG {
                    println!("blacken {}", fn_obj.borrow());
                }
            }
            Obj::Structs(obj) => {
                let structs_obj: &RefCell<Structs> = obj.borrow();
                if DEBUG {
                    println!("blacken {}", structs_obj.borrow());
                }
            }
            Obj::Instance(obj) => {
                let instance_obj: &RefCell<StructsInstance> = obj.borrow();
                if DEBUG {
                    println!("blacken {}", instance_obj.borrow());
                }
            }
        }
    }

    fn print_unmarked_val(&self, val: &Value) {
        match val {
            Value::Obj(obj) => match obj {
                Obj::String(obj) => {
                    let str_obj: &RefCell<StrObj> = obj.borrow();
                    let str_obj = str_obj.borrow();
                    if !str_obj.is_marked {
                        println!("{}", str_obj);
                    }
                }
                Obj::NativeFn(obj) => {
                    let native_fn: &RefCell<NativeFn> = obj.borrow();
                    let native_fn = native_fn.borrow();
                    if !native_fn.is_marked {
                        println!("{}", native_fn);
                    }
                }
                Obj::Function(obj) => {
                    let fn_obj: &RefCell<FunctionObj> = obj.borrow();
                    let fn_obj = fn_obj.borrow();
                    if !fn_obj.is_marked {
                        println!("{}", fn_obj);
                    }
                }
                Obj::Structs(obj) => {
                    let structs_obj: &RefCell<Structs> = obj.borrow();
                    let structs_obj = structs_obj.borrow();
                    if !structs_obj.is_marked {
                        println!("{}", structs_obj);
                    }
                }

                Obj::Instance(obj) => {
                    let instance_obj: &RefCell<StructsInstance> = obj.borrow();
                    let instance_obj = instance_obj.borrow();
                    if !instance_obj.is_marked {
                        println!("{}", instance_obj);
                    }
                }
            },
            _ => {}
        }
    }
}
