use std::fmt::Display;

use super::ast::*;

pub trait ASTNode {
    fn print(&self, prefix: String, _children_prefix: String) -> String {
        format!("{}ASTNode {{}}\n", prefix)
    }
}

impl ASTNode for ProgramAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Program\n", prefix);

        if self.extern_list.is_none() {
            out.push_str("├── None\n");
        }

        if let Some(extern_list) = &*self.extern_list {
            let fmt = extern_list.print(
                children_prefix.clone() + "├── ",
                children_prefix.clone() + "│   ",
            );
            out.push_str(fmt.as_str());
        }

        out.push_str(
            self.decl_list
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );
        out
    }
}

// Externs //
impl ASTNode for ExternListAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ExternList\n", prefix);
        let has_next = self.extern_list.is_some();

        out.push_str(
            self.extern_item
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " }
                )
                .as_str(),
        );

        if let Some(extern_list) = &*self.extern_list {
            let fmt = extern_list.print(
                children_prefix.clone() + "└── ",
                children_prefix.clone() + "    ",
            );
            out.push_str(fmt.as_str());
        }

        out
    }
}

impl ASTNode for ExternAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!(
            "{}Extern Type[{}] Identifier[{}]\n",
            prefix, self.type_spec, self.identifier
        );

        out.push_str(
            self.params
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );

        out
    }
}

// Declarations //
impl ASTNode for DeclListAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}DeclList\n", prefix);
        let has_next = self.decl_list_prime.is_some();

        out.push_str(
            self.decl
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " }
                )
                .as_str(),
        );

        if let Some(decl_list_prime) = &*self.decl_list_prime {
            let fmt =
                decl_list_prime.print(children_prefix.clone() + "└── ", children_prefix + "    ");
            out.push_str(fmt.as_str());
        }

        out
    }
}

impl ASTNode for DeclListPrimeAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}DeclListPrime\n", prefix);
        let has_next = self.decl_list_prime.is_some();

        out.push_str(
            self.decl
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " }
                )
                .as_str(),
        );

        if let Some(decl_list_prime) = &*self.decl_list_prime {
            let fmt =
                decl_list_prime.print(children_prefix.clone() + "└── ", children_prefix + "    ");
            out.push_str(fmt.as_str());
        }

        out
    }
}

impl ASTNode for DeclAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Decl", prefix);

        match &self {
            DeclAST::Variable {
                var_type,
                identifier,
            } => {
                out.push_str(format!(" (Variable) Type[{}] Identifier[{}]\n", var_type, identifier).as_str());
                out
            }
            DeclAST::Function {
                type_spec,
                identifier,
                fun_decl,
            } => {
                out.push_str(format!(" (Function) Type[{}] Identifier[{}]\n", type_spec, identifier).as_str());
                out.push_str(
                    fun_decl
                        .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                        .as_str(),
                );
                out
            }
        }
    }
}

// Types //
impl Display for TypeSpecAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TypeSpecAST::Void => write!(f, "Type::Void"),
            TypeSpecAST::VarType(vt) => write!(f, "{}", vt),
        }
    }
}

impl ASTNode for TypeSpecAST {
    fn print(&self, _prefix: String, _children_prefix: String) -> String {
        format!("{}", self)
    }
}

impl Display for VarTypeAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Type::{}",
            match &self {
                VarTypeAST::Bool => "Bool",
                VarTypeAST::Float => "Float",
                VarTypeAST::Int => "Int",
            }
        )
    }
}

impl ASTNode for VarTypeAST {
    fn print(&self, _prefix: String, _children_prefix: String) -> String {
        format!("{}", self)
    }
}

// Function //
impl ASTNode for FunDecl {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}FunDecl\n", prefix);

        out.push_str(
            self.param_list
                .print(
                    children_prefix.clone() + "├── ",
                    children_prefix.clone() + "│   ",
                )
                .as_str(),
        );

        out.push_str(
            self.code_block
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );

        out
    }
}

// Params //
impl ASTNode for ParamsAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        if let Some(params) = &self.param_list {
            let mut out = format!("{}Params\n", prefix);
            out.push_str(
                params
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
            out
        } else {
            format!("{}Params: None\n", prefix)
        }
    }
}

impl ASTNode for ParamListAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ParamList\n", prefix);
        let has_next = self.param_list_prime.is_some();

        out.push_str(
            self.param
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " }
                )
                .as_str(),
        );

        if let Some(param_list_prime) = &*self.param_list_prime {
            out.push_str(
                param_list_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for ParamListPrimeAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ParamListPrime\n", prefix);
        let has_next = self.param_list_prime.is_some();

        out.push_str(
            self.param
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " }
                )
                .as_str(),
        );

        if let Some(param_list_prime) = &*self.param_list_prime {
            out.push_str(
                param_list_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for ParamAST {
    fn print(&self, prefix: String, _children_prefix: String) -> String {
        format!("{}Param {} {}\n", prefix, self.var_type, self.identifier)
    }
}

// Block //
impl ASTNode for BlockAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Block\n", prefix);

        let has_next = self.stmt_list.is_some();

        if let Some(local_decls) = &*self.local_decls {
            out.push_str(
                local_decls
                    .print(
                        children_prefix.clone()
                            + if has_next { "├── " } else { "└── " },
                        children_prefix.clone() + if has_next { "│   " } else { "    " },
                    )
                    .as_str(),
            );
        }

        if let Some(stmt_list) = &*self.stmt_list {
            out.push_str(
                stmt_list
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for LocalDeclsAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}LocalDecls\n", prefix);
        let has_next = self.local_decls.is_some();

        out.push_str(
            self.local_decl
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(local_decls) = &*self.local_decls {
            out.push_str(
                local_decls
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for LocalDeclAST {
    fn print(&self, prefix: String, _children_prefix: String) -> String {
        format!(
            "{}LocalDeclAST Type[{}] Identifier[{}]\n",
            prefix, self.var_type, self.identifier
        )
    }
}

impl ASTNode for StmtListAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}StmtList\n", prefix);
        let has_next = self.stmt_list.is_some();

        out.push_str(
            self.stmt
                .print(
                    children_prefix.clone() + if has_next { "├── " } else { "└── " },
                    children_prefix.clone() + if has_next { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(stmt_list) = &*self.stmt_list {
            out.push_str(
                stmt_list
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            )
        }

        out
    }
}

impl ASTNode for StmtAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        match &self {
            StmtAST::Block(block) => block.print(prefix, children_prefix),
            StmtAST::ExprStmt(expr) => match expr {
                Some(expr_stmt) => expr_stmt.print(prefix, children_prefix),
                None => format!("{}Stmt ExprStmt [None]", prefix),
            },
            StmtAST::IfStmt(if_stmt) => if_stmt.print(prefix, children_prefix),
            StmtAST::WhileStmt(while_stmt) => while_stmt.print(prefix, children_prefix),
            StmtAST::ReturnStmt(return_stmt) => return_stmt.print(prefix, children_prefix),
        }
    }
}

impl ASTNode for ExprStmtAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ExprStmt\n", prefix);
        out.push_str(
            self.expr
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );
        out
    }
}

impl ASTNode for WhileStmtAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}WhileStmt\n", prefix);
        out.push_str(
            self.expr
                .print(
                    children_prefix.clone() + "├── ",
                    children_prefix.clone() + "│   ",
                )
                .as_str(),
        );

        out.push_str(
            self.stmt
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );

        out
    }
}

impl ASTNode for IfStmtAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}IfStmt\n", prefix);
        let is_else = self.else_stmt.is_some();

        out.push_str(
            self.expr
                .print(
                    children_prefix.clone() + "├── ",
                    children_prefix.clone() + "│   ",
                )
                .as_str(),
        );

        out.push_str(
            self.block
                .print(
                    children_prefix.clone() + if is_else { "├── " } else { "└── " },
                    children_prefix.clone() + if is_else { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(else_stmt) = &*self.else_stmt {
            out.push_str(
                else_stmt
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for ElseStmtAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ElseStmt\n", prefix);
        out.push_str(
            self.block
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );

        out
    }
}

impl ASTNode for ReturnStmtAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ReturnStmt\n", prefix);

        if let Some(expr_stmt) = &self.expr_stmt {
            out.push_str(
                expr_stmt
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for ExprAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Expr", prefix);
        match &self {
            ExprAST::Assignment { identifier, expr } => {
                out.push_str(format!(" Identifier[{}]\n", identifier).as_str());
                out.push_str(
                    expr.print(children_prefix.clone() + "└── ", children_prefix + "    ")
                        .as_str(),
                );
            }
            ExprAST::Logical(logical) => {
                out.push_str("\n");
                out.push_str(
                    logical
                        .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                        .as_str(),
                );
            }
        };

        out
    }
}

impl ASTNode for LogicalAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Logical\n", prefix);
        let not_last = self.logical_prime.is_some();

        out.push_str(
            self.compare
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(logical_prime) = &*self.logical_prime {
            out.push_str(
                logical_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LogicalOp::{}",
            match &self {
                LogicalOp::And => "And",
                LogicalOp::Or => "Or",
            }
        )
    }
}

impl ASTNode for LogicalPrimeAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}LogicalPrime Operator[{}]\n", prefix, self.op);
        let not_last = self.logical_prime.is_some();
        out.push_str(
            self.compare
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(logical_prime) = &*self.logical_prime {
            out.push_str(
                logical_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for CompareAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Compare\n", prefix);
        let not_last = self.compare_prime.is_some();

        out.push_str(
            self.math
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(compare_prime) = &*self.compare_prime {
            out.push_str(
                compare_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl Display for CompareOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompareOp::{}",
            match &self {
                CompareOp::Equal => "EqualEqual",
                CompareOp::NotEqual => "NotEqual",
                CompareOp::Gt => "GreaterThan",
                CompareOp::GtEq => "GtEqual",
                CompareOp::Lt => "LessThan",
                CompareOp::LtEq => "LtEqual",
            }
        )
    }
}

impl ASTNode for ComparePrimeAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ComparePrime Operator[{}]\n", prefix, self.op);
        let not_last = self.compare_prime.is_some();

        out.push_str(
            self.math
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(compare_prime) = &*self.compare_prime {
            out.push_str(
                compare_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for MathAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Math\n", prefix);
        let not_last = self.math_prime.is_some();

        out.push_str(
            self.unary
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(math_prime) = &*self.math_prime {
            out.push_str(
                math_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl Display for MathOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MathOp::{}",
            match &self {
                MathOp::Plus => "Plus",
                MathOp::Minus => "Minus",
                MathOp::Mult => "Mult",
                MathOp::Div => "Div",
                MathOp::Mod => "Mod",
            }
        )
    }
}

impl ASTNode for MathPrimeAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}MathPrime Operator[{}]\n", prefix, self.op);
        let not_last = self.math_prime.is_some();

        out.push_str(
            self.unary
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(math_prime) = &*self.math_prime {
            out.push_str(
                math_prime
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for UnaryAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Unary Operator[{}]\n", prefix, self.op);
        out.push_str(
            self.literals
                .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                .as_str(),
        );

        out
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnaryOp::{}",
            match &self {
                UnaryOp::Not => "Not",
                UnaryOp::None => "None",
                UnaryOp::Minus => "Negative",
            }
        )
    }
}

impl ASTNode for LiteralAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}Literal ", prefix);

        match &self {
            LiteralAST::BoolLiteral(bool_lit) => {
                out.push_str(format!(" Bool[{}]\n", bool_lit).as_str())
            }
            LiteralAST::FloatLiteral(float_lit) => {
                out.push_str(format!(" Float[{}]\n", float_lit).as_str())
            }
            LiteralAST::IntegerLiteral(int_lit) => {
                out.push_str(format!(" Integer[{}]\n", int_lit).as_str())
            }
            LiteralAST::Expr(expr) => {
                out.push_str("\n");
                out.push_str(
                    expr.print(children_prefix.clone() + "└── ", children_prefix + "    ")
                        .as_str(),
                )
            }
            LiteralAST::Identity(identity) => {
                out.push_str(format!("Identity[{}]\n", identity).as_str())
            }
            LiteralAST::FunctionCall { identifier, args } => {
                out.push_str(format!("FunctionCall Identity[{}]\n", identifier).as_str());
                out.push_str(
                    args.print(children_prefix.clone() + "└── ", children_prefix + "    ")
                        .as_str(),
                )
            }
        }

        out
    }
}

impl ASTNode for ArgsAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        match &*self.args_list {
            Some(args) => {
                let mut out = format!("{}Args\n", prefix);

                out.push_str(
                    args.print(children_prefix.clone() + "└── ", children_prefix + "    ")
                        .as_str(),
                );

                out
            }
            None => format!("{}Args: None\n", prefix),
        }
    }
}

impl ASTNode for ArgsListAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ArgsList\n", prefix);
        let not_last = self.args_list.is_some();

        out.push_str(
            self.expr
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(args_list) = &*self.args_list {
            out.push_str(
                args_list
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}

impl ASTNode for ArgsListPrimeAST {
    fn print(&self, prefix: String, children_prefix: String) -> String {
        let mut out = format!("{}ArgsListPrime\n", prefix);
        let not_last = self.args_list_prime.is_some();

        out.push_str(
            self.expr
                .print(
                    children_prefix.clone() + if not_last { "├── " } else { "└── " },
                    children_prefix.clone() + if not_last { "│   " } else { "    " },
                )
                .as_str(),
        );

        if let Some(args_list) = &*self.args_list_prime {
            out.push_str(
                args_list
                    .print(children_prefix.clone() + "└── ", children_prefix + "    ")
                    .as_str(),
            );
        }

        out
    }
}
