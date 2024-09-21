use std::sync::Mutex;

use super::*;

pub static TOP_MODULE: Mutex<Option<String>> = Mutex::new(None);

pub trait GenerateVerilog {
    type Out;
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error>;
}

impl GenerateVerilog for VType {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        assert_eq!(self.star,0);
        write!(code.borrow_mut(),"[{}:0]",match self.ty {
            VTypeEnum::I8 => 8,
            VTypeEnum::U64 => 64,
            VTypeEnum::Void => 0,
            _ => unimplemented!(),
        }-1).unwrap();
        Ok(())
    }
}

impl GenerateVerilog for CompUnit {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        for item in self.global_items.iter() {
            item.generate(code.clone())?;
        }
        Ok(())
    }
}

impl GenerateVerilog for GlobalItem {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        match self {
            GlobalItem::ConstDecl(decl) => decl.generate(code),
            _ => unimplemented!()
        }
    }
}

impl GenerateVerilog for ConstDecl {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        write!(code.borrow_mut(),"module {}",self.id).unwrap();
        self.init.generate(code.clone())?;
        writeln!(code.borrow_mut(),"endmodule").unwrap();

        if let Some(attr) = &self.attr {
            if attr.attrs[0] == "top" {
                if TOP_MODULE.lock().unwrap().is_some() {
                    panic!("Duplicated top modules! {}",attr.span);
                }
                *TOP_MODULE.lock().unwrap() = Some(self.id.clone());
            }
        }

        Ok(())
    }
}

impl GenerateVerilog for ConstInitVal {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        match self {
            Self::Function(func) => func.generate(code),
            _ => unimplemented!(),
        }
    }
}

impl GenerateVerilog for FuncDef {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        write!(code.borrow_mut(),"(").unwrap();

        for param in self.params.iter() {
            write!(code.borrow_mut(),"input ").unwrap();
            param.ty.generate(code.clone())?;
            write!(code.borrow_mut()," {},",param.id).unwrap();
        }

        write!(code.borrow_mut(),"output ").unwrap();
        self.func_type.generate(code.clone())?;
        writeln!(code.borrow_mut(),"  out);").unwrap();

        self.block.generate(code.clone())?;

        Ok(())
    }
}

impl GenerateVerilog for Block {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        for item in self.items.iter() {
            item.generate(code.clone())?;
        }
        Ok(())
    }
}

impl GenerateVerilog for BlockItem {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        match self {
            BlockItem::Stmt(stmt) => stmt.generate(code),
            _ => unimplemented!(),
        }
    }
}

impl GenerateVerilog for Stmt {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        match self {
            Self::Return(ret) => ret.generate(code),
            _ => unimplemented!(),
        }
    }
}

impl GenerateVerilog for Return {
    type Out = ();

    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        write!(code.borrow_mut(),"assign out = ").unwrap();
        self.exp.generate(code.clone())?;
        writeln!(code.borrow_mut(),";").unwrap();
        Ok(())
    }
}

impl GenerateVerilog for Exp {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        match self {
            Self::Unary(unary, exp, _span) => {
                write!(code.borrow_mut(),"{}",match unary {
                    UnaryOp::Not => "~",
                    UnaryOp::Negative => "-",
                    _ => "",
                }).unwrap();
                exp.generate(code)
            },
            Self::Exp(exp, _span) => exp.generate(code),
            Self::Number(number) => number.generate(code),
            Self::LVal(lval) => lval.generate(code),
            Self::Binary(lhs, op, rhs, _span) => {
                lhs.generate(code.clone())?;
                write!(code.borrow_mut()," {} ",match op {
                    BinaryOp::Eq => "==",
                    BinaryOp::Neq => "!=",
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Ge => ">=",
                    BinaryOp::Le => "<=",
                    BinaryOp::Gt => ">",
                    BinaryOp::Lt => "<",
                    BinaryOp::Mod => "%",
                }).unwrap();
                rhs.generate(code.clone())?;
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}

impl GenerateVerilog for Number {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        write!(code.borrow_mut(),"{}",self.num).unwrap();
        Ok(())
    }
}

impl GenerateVerilog for LVal {
    type Out = ();
    fn generate(&self, code: Rc<RefCell<String>>) -> Result<Self::Out, Error> {
        write!(code.borrow_mut(),"{}",self.ids.join(".")).unwrap();
        Ok(())
    }
}



