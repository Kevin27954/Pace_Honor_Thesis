use std::{borrow::Borrow, cell::RefCell};

use crate::{
    compiler::values::{FunctionObj, NativeFn, Obj, StrObj, Value},
    vm::VM,
};

impl VM {
    pub fn collect_garbage(&self) {
        println!("-- Collecting Garbage");

        let mut worklist = Vec::new();
        self.mark_root(worklist.as_mut());

        loop {
            match worklist.pop() {
                Some(obj) => self.blacken_obj(obj),
                None => break,
            }
        }

        self.sweep();

        println!("-- Finished Collecting Garbage");
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
                        println!("mark {}", str_obj.borrow());
                        let mut str_obj = str_obj.borrow_mut();
                        str_obj.is_marked = if str_obj.is_marked { return } else { true };
                    }
                    Obj::Function(func) => {
                        let func_obj: &RefCell<FunctionObj> = func.borrow();
                        println!("mark {}", func_obj.borrow());
                        let mut func_obj = func_obj.borrow_mut();
                        func_obj.is_marked = if func_obj.is_marked { return } else { true };
                    }
                    Obj::NativeFn(nativ_func) => {
                        let native_func_obj: &RefCell<NativeFn> = nativ_func.borrow();
                        println!("mark {}", native_func_obj.borrow());
                        let mut native_func_obj = native_func_obj.borrow_mut();
                        native_func_obj.is_marked = if native_func_obj.is_marked {
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

        println!("--- Unmarked Objs ---");
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
                println!("blacken {}", str_obj.borrow());
            }
            Obj::NativeFn(obj) => {
                let native_fn: &RefCell<NativeFn> = obj.borrow();
                println!("blacken {}", native_fn.borrow());
            }
            Obj::Function(obj) => {
                let fn_obj: &RefCell<FunctionObj> = obj.borrow();
                println!("blacken {}", fn_obj.borrow());
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
            },
            _ => {}
        }
    }
}
