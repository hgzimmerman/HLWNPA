use ast::abstract_syntax_tree::Ast;
use ast::type_info::TypeInfo;
use std::collections::HashMap;
use ast::s_expression::SExpression;

pub enum TypeError {
    TypeMismatch,
    UnsupportedOperation
}

pub type TypeResult = Result<TypeInfo, TypeError>;
type TypeStore = HashMap<String, TypeInfo>;

impl Ast {
    fn check_types( &self, type_store: &mut TypeStore ) -> Result<TypeInfo, TypeError> {
        match *self {
            Ast::SExpr(ref sexpr) => {
                match *sexpr {
                    SExpression::Add(ref lhs, ref rhs) => {
                        TypeInfo::from(lhs.check_types(type_store)?) + TypeInfo::from(rhs.check_types(type_store)?)
                    }
                    SExpression::Subtract(ref lhs, ref rhs) => {
                        TypeInfo::from(lhs.check_types(type_store)?) + TypeInfo::from(rhs.check_types(type_store)?)
                    }
                    _ => unimplemented!()
                }

            }
            _ => unimplemented!()
        }

    }
}