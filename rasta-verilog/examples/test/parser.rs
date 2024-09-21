use std::sync::Mutex;

use rasta::*;
use pest::{
    iterators::Pair,
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cara.pest"]
pub struct CaraParser;

static FILE: Mutex<String> = Mutex::new(String::new());

pub fn parse(code: String,file: String) -> CompUnit {
    *FILE.lock().unwrap() = file;
    let result = CaraParser::parse(Rule::comp_unit, &code);

    if let Err(err) = result {
        panic!("{}", err);
    }

    to_ast(result.unwrap().next().unwrap())
}

fn get_span(span: pest::Span<'_>) -> Span {
    let code = span.lines().next().unwrap();
    Span::new(span.start_pos().line_col(), span.end_pos().line_col(), code.into(),FILE.lock().unwrap().clone())
}

fn to_ast(rules: Pair<Rule>) -> CompUnit {
    let mut items = Vec::new();
    let span = rules.as_span().clone();

    for line in rules.into_inner() {
        match line.as_rule() {
            Rule::const_decl => items.push(GlobalItem::ConstDecl(parse_const_decl(line))),
            Rule::inline_asm => items.push(GlobalItem::InlineAsm(parse_inline_asm(line))),
            Rule::soi | Rule::eoi => {}
            _ => unimplemented!(),
        }
    }

    CompUnit {
        global_items: items,
        span: get_span(span),
    }
}

fn parse_const_decl(rules: Pair<Rule>) -> ConstDecl {
    let mut rules_iter = rules.clone().into_inner();

    let attributes = rules_iter.next().unwrap();

    let (id, attr) = if let Rule::attributes = attributes.as_rule() {
        (
            rules_iter.next().unwrap(),
            Some(parse_attributes(attributes)),
        )
    } else {
        (attributes, None)
    };
    let id = parse_ident(id);

    let init = parse_const_init_val(rules_iter.next().unwrap());

    ConstDecl {
        span: get_span(rules.as_span().clone()),
        id,
        attr,
        init,
    }
}

fn parse_const_init_val(rules: Pair<Rule>) -> ConstInitVal {
    let mut rules_iter = rules.clone().into_inner();

    let init_val = rules_iter.next().unwrap();

    match init_val.as_rule() {
        Rule::const_exp => ConstInitVal::Exp(parse_const_expr(init_val)),
        Rule::func_def => ConstInitVal::Function(parse_function_def(init_val)),
        _ => unimplemented!(),
    }
}

fn parse_const_expr(rules: Pair<Rule>) -> ConstExp {
    ConstExp {
        exp: parse_expr(rules),
    }
}

fn parse_attributes(rules: Pair<Rule>) -> Attributes {
    Attributes {
        span: get_span(rules.as_span().clone()),
        attrs: rules
            .into_inner()
            .map(|attr| attr.as_str().into())
            .collect(),
    }
}

fn parse_ident(rules: Pair<Rule>) -> String {
    rules.as_str().to_string()
}

fn parse_deref(rules: Pair<Rule>) -> Deref {
    let mut primary_iter = rules.clone().into_inner();

    let tmp = primary_iter.next().unwrap();

    match tmp.as_rule() {
        Rule::lval => {
            let lval = parse_lval(tmp);
            if let Some(exp) = primary_iter.next() {
                let exp = parse_expr(exp);
                Deref::DerefPtr(lval, exp, get_span(rules.as_span()))
            } else {
                Deref::DerefId(lval, get_span(rules.as_span()))
            }
        }
        Rule::exp => {
            let exp = parse_expr(tmp);
            if let Some(exp2) = primary_iter.next() {
                let exp2 = parse_expr(exp2);
                Deref::DerefPtrExp(exp, exp2, get_span(rules.as_span()))
            } else {
                Deref::DerefExp(exp, get_span(rules.as_span()))
            }
        }
        _ => unreachable!(),
    }
}

fn parse_values(rules: Pair<Rule>) -> Exp {
    match rules.as_rule() {
        Rule::exp => parse_expr(rules),
        Rule::string => Exp::Str(parse_string(rules.clone()), get_span(rules.as_span())),
        Rule::array_def => parse_array_def(rules),
        _ => unimplemented!(),
    }
}

fn parse_array_def(rules: Pair<Rule>) -> Exp {
    let mut rules_iter = rules.clone().into_inner();

    if let Some(exp) = rules_iter.next() {
        let value = parse_values(exp);
        if let Some(tmp) = rules_iter.next() {
            match tmp.as_rule() {
                Rule::const_exp => {
                    let num = parse_const_expr(tmp);
                    Exp::Array(Box::new(Array::Template(
                        value,
                        num,
                        get_span(rules.as_span()),
                    )))
                }
                _ => {
                    let mut values = vec![value, parse_values(tmp)];
                    while let Some(tmp) = rules_iter.next() {
                        values.push(parse_values(tmp));
                    }
                    Exp::Array(Box::new(Array::List(values, get_span(rules.as_span()))))
                }
            }
        } else {
            Exp::Array(Box::new(Array::List(
                vec![value],
                get_span(rules.as_span()),
            )))
        }
    } else {
        Exp::Array(Box::new(Array::List(Vec::new(), get_span(rules.as_span()))))
    }
}

fn parse_expr(rules: Pair<Rule>) -> Exp {
    let pratt = PrattParser::new()
        .op(Op::infix(Rule::eq, Assoc::Left) | Op::infix(Rule::neq, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left)
            | Op::infix(Rule::div, Assoc::Left)
            | Op::infix(Rule::r#mod, Assoc::Left))
        .op(Op::prefix(Rule::neg) | Op::prefix(Rule::pos));

    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::exp => parse_expr(primary),
            Rule::number => Exp::Number(Number {
                num: primary.as_str().parse().unwrap(),
                span: get_span(primary.as_span()),
            }),
            Rule::lval => Exp::LVal(Box::new(parse_lval(primary))),
            Rule::func_call => Exp::FuncCall({
                let mut primary_iter = primary.clone().into_inner();

                let ids = vec![parse_ident(primary_iter.next().unwrap())];

                let mut args = Vec::new();

                while let Some(arg) = primary_iter.next() {
                    args.push(parse_values(arg));
                }

                FuncCall {
                    ids,
                    args,
                    span: get_span(primary.as_span()),
                }
            }),
            Rule::get_addr => {
                let mut primary_iter = primary.clone().into_inner();

                let lval = parse_lval(primary_iter.next().unwrap());

                Exp::GetAddr(Box::new(GetAddr {
                    lval,
                    span: get_span(primary.as_span()),
                }))
            }
            Rule::deref => Exp::Deref(Box::new(parse_deref(primary.clone()))),

            _ => panic!("Unkown primary {}!", primary),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => Exp::Unary(UnaryOp::Negative, Box::new(rhs), get_span(op.as_span())),
            Rule::pos => Exp::Unary(UnaryOp::Positive, Box::new(rhs), get_span(op.as_span())),
            _ => unimplemented!(),
        })
        .map_postfix(|_lhs, _op| unimplemented!())
        .map_infix(|lhs, op, rhs| {
            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);

            Exp::Binary(
                lhs,
                match op.as_rule() {
                    Rule::eq => BinaryOp::Eq,
                    Rule::neq => BinaryOp::Neq,
                    Rule::add => BinaryOp::Add,
                    Rule::sub => BinaryOp::Sub,
                    Rule::mul => BinaryOp::Mul,
                    Rule::div => BinaryOp::Div,
                    Rule::r#mod => BinaryOp::Mod,
                    _ => unimplemented!(),
                },
                rhs,
                get_span(op.as_span()),
            )
        })
        .parse(rules.into_inner())
}

fn parse_function_def(rules: Pair<Rule>) -> FuncDef {
    let mut rules_iter = rules.clone().into_inner();

    let (return_type, params) = {
        let tmp_k = rules_iter.next().unwrap();

        let mut params = Vec::new();

        match tmp_k.as_rule() {
            Rule::param => {
                let mut tmp_k_iter = tmp_k.clone().into_inner();
                let id = tmp_k_iter.next().unwrap().as_str().to_string();

                let ty = parse_vtype(tmp_k_iter.next().unwrap());
                params.push(Param {
                    ty,
                    id,
                    span: get_span(tmp_k.as_span().clone()),
                });

                let rule = loop {
                    let tmp_k = rules_iter.next().unwrap();
                    match tmp_k.as_rule() {
                        Rule::param => {
                            let mut tmp_k_iter = tmp_k.clone().into_inner();
                            let id = tmp_k_iter.next().unwrap().as_str().to_string();
                            let ty = parse_vtype(tmp_k_iter.next().unwrap());
                            params.push(Param {
                                ty,
                                id,
                                span: get_span(tmp_k.as_span()),
                            });
                        }

                        _ => break tmp_k,
                    }
                };
                (parse_vtype(rule), params)
            }
            _ => (parse_vtype(tmp_k), params),
        }
    };

    FuncDef {
        span: get_span(rules.as_span().clone()),
        params,
        func_type: return_type,
        block: parse_block(rules_iter.next().unwrap()),
    }
}

fn parse_block(rules: Pair<Rule>) -> Block {
    let mut rules_iter = rules.clone().into_inner();
    let mut item = Vec::new();
    while let Some(rule) = rules_iter.next() {
        match rule.as_rule() {
            Rule::stmt => item.push(BlockItem::Stmt(parse_stmt(rule))),
            Rule::decl => item.push(BlockItem::Decl(parse_decl(rule))),
            _ => unimplemented!(),
        }
    }
    Block {
        span: get_span(rules.as_span().clone()),
        items: item,
    }
}

fn parse_decl(rules: Pair<Rule>) -> Decl {
    let mut rules_iter = rules.clone().into_inner();

    let decl = rules_iter.next().unwrap();

    match decl.as_rule() {
        Rule::const_decl => Decl::Const(parse_const_decl(decl)),
        Rule::var_decl => Decl::Var(parse_var_decl(decl)),
        _ => unimplemented!(),
    }
}

fn parse_var_decl(rules: Pair<Rule>) -> VarDecl {
    let mut rules_iter = rules.clone().into_inner();

    let id = rules_iter.next().unwrap().as_str().to_string();
    let ty = parse_vtype(rules_iter.next().unwrap());
    let init = parse_init_val(rules_iter.next().unwrap());

    VarDecl {
        span: get_span(rules.as_span().clone()),
        id: id.as_str().to_string(),
        ty,
        init,
    }
}

fn parse_init_val(rules: Pair<Rule>) -> InitVal {
    let mut rules_iter = rules.clone().into_inner();

    let init_val = rules_iter.next().unwrap();

    InitVal {
        exp: parse_values(init_val),
    }
}

fn parse_stmt(rules: Pair<Rule>) -> Stmt {
    let mut rules_iter = rules.clone().into_inner();

    let stmt = rules_iter.next().unwrap();

    match stmt.as_rule() {
        Rule::r#return => Stmt::Return(parse_return(stmt)),
        Rule::assign => Stmt::Assign(parse_assign(stmt)),
        Rule::block => Stmt::Block(parse_block(stmt)),
        Rule::r#if => Stmt::If(parse_if(stmt)),
        Rule::r#while => Stmt::While(parse_while(stmt)),
        Rule::inline_asm => Stmt::InlineAsm(parse_inline_asm(stmt)),
        Rule::terminator => Stmt::Terminator(parse_terminator(stmt)),
        Rule::r#for => Stmt::For(parse_for(stmt)),
        Rule::exp => Stmt::Exp(Some(parse_expr(stmt))),
        _ => unimplemented!(),
    }
}

fn parse_inline_asm(rules: Pair<Rule>) -> InlineAsm {
    let mut rules_iter = rules.clone().into_inner();

    let code = parse_string(rules_iter.next().unwrap());

    let constraints = if let Some(constraint_r) = rules_iter.next() {
        let mut constraint = constraint_r.clone().into_inner();
        let id = parse_ident(constraint.next().unwrap());
        let any = constraint.next().unwrap();
        let constraint = match any.as_rule() {
            Rule::exp => AsmConstraint::In(id, parse_expr(any), get_span(constraint_r.as_span())),
            _ => unimplemented!(),
        };

        let mut constraints = vec![constraint];

        while let Some(constraint_r) = rules_iter.next() {
            let mut constraint = constraint_r.clone().into_inner();
            let id = parse_ident(constraint.next().unwrap());
            let any = constraint.next().unwrap();
            let constraint = match any.as_rule() {
                Rule::exp => {
                    AsmConstraint::In(id, parse_expr(any), get_span(constraint_r.as_span()))
                }
                _ => unimplemented!(),
            };
            constraints.push(constraint);
        }

        constraints
    } else {
        Vec::new()
    };

    InlineAsm {
        asm: code,
        constraints,
        span: get_span(rules.as_span()),
    }
}

fn parse_string(rules: Pair<Rule>) -> String {
    let inner = rules.clone().into_inner().next().unwrap();
    String::from_utf8(escape_bytes::unescape(inner.as_str().to_string().as_bytes()).unwrap())
        .unwrap()
}

fn parse_while(rules: Pair<Rule>) -> While {
    let mut rules_iter = rules.clone().into_inner();

    let cond = parse_expr(rules_iter.next().unwrap());
    let then = parse_block(rules_iter.next().unwrap());

    While {
        cond,
        then,
        span: get_span(rules.as_span()),
    }
}

fn parse_if(rules: Pair<Rule>) -> If {
    let mut rules_iter = rules.clone().into_inner();

    let cond = rules_iter.next().unwrap();

    let cond = parse_expr(cond);

    let then = parse_block(rules_iter.next().unwrap());

    let else_then = if let Some(rule) = rules_iter.next() {
        Some(parse_block(rule))
    } else {
        None
    };

    If {
        cond,
        then,
        else_then,
        span: get_span(rules.as_span()),
    }
}

fn parse_terminator(rules: Pair<Rule>) -> Terminator {
    match rules.as_str() {
        "break;" => Terminator::Break(get_span(rules.as_span().clone())),
        "continue;" => Terminator::Continue(get_span(rules.as_span().clone())),
        _ => panic!("Unknown terminator {}!", rules.as_str()),
    }
}

fn parse_for(rules: Pair<Rule>) -> For {
    let mut rules_iter = rules.clone().into_inner();

    let var_name = rules_iter.next().unwrap().as_str().to_string();
    let start = parse_expr(rules_iter.next().unwrap());
    let end = parse_expr(rules_iter.next().unwrap());
    let step = parse_expr(rules_iter.next().unwrap());
    let block = parse_block(rules_iter.next().unwrap());

    For {
        span: get_span(rules.as_span().clone()),
        var: var_name,
        start,
        end,
        step,
        then: block,
    }
}

fn parse_assign(rules: Pair<Rule>) -> Assign {
    let mut rules_iter = rules.clone().into_inner();

    let tmp = rules_iter.next().unwrap();

    match tmp.as_rule() {
        Rule::deref => {
            let lhs = parse_deref(tmp);
            let rhs = parse_expr(rules_iter.next().unwrap());
            Assign::WritePtr(lhs, rhs, get_span(rules.as_span().clone()))
        }
        Rule::lval => {
            let id = parse_lval(tmp);
            let exp = parse_expr(rules_iter.next().unwrap());

            Assign::WriteVar(id, exp, get_span(rules.as_span().clone()))
        }
        _ => panic!("Unknown assignment {}!",tmp),
    }
}

fn parse_lval(rules: Pair<Rule>) -> LVal {
    let ident = rules.as_str().to_string();

    LVal {
        ids: vec![ident],
        span: get_span(rules.as_span().clone()),
        exp: None,
    }
}

fn parse_return(rules: Pair<Rule>) -> Return {
    let mut rules_iter = rules.clone().into_inner();

    let exp = rules_iter.next().unwrap();

    Return {
        span: get_span(rules.as_span().clone()),
        exp: parse_expr(exp),
    }
}

fn parse_vtype(rules: Pair<Rule>) -> VType {
    let mut rules_iter = rules.clone().into_inner();

    let vtype_enum = rules_iter.next().unwrap();

    let vty_enum = match vtype_enum.as_str() {
        "u64" => VTypeEnum::U64,
        "i8" => VTypeEnum::I8,
        "void" => VTypeEnum::Void,
        _ => panic!("Unkown type {}!", vtype_enum.as_str()),
    };

    if let Some(_) = rules_iter.next() {
        let mut star_cnt = 1usize;
        while let Some(_) = rules_iter.next() {
            star_cnt += 1;
        }
        VType {
            ty: vty_enum,
            star: star_cnt,
            span: get_span(rules.as_span().clone()),
        }
    } else {
        VType {
            ty: vty_enum,
            star: 0,
            span: get_span(rules.as_span().clone()),
        }
    }
}
