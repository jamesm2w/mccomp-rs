use std::fmt::Debug;

#[derive(Debug)]
pub struct ProgramAST {
    pub extern_list: Box<Option<ExternListAST>>,
    pub decl_list: Box<DeclListAST>
}

#[derive(Debug)]
pub struct ExternListAST {
    pub extern_item: Box<ExternAST>,
    pub extern_list: Box<Option<ExternListAST>>
}

#[derive(Debug)]
pub struct ExternAST {
    pub type_spec: TypeSpecAST,
    pub identifier: String,
    pub params: Box<ParamsAST>
}

#[derive(Debug)]
pub struct DeclListAST {
    pub decl: Box<DeclAST>,
    pub decl_list_prime: Box<Option<DeclListPrimeAST>>
}

#[derive(Debug)]
pub struct DeclListPrimeAST {
    pub decl: Box<DeclAST>,
    pub decl_list_prime: Box<Option<DeclListPrimeAST>>
}

// pub struct DeclAST {
//     pub var_type: VarTypeAST,
//     pub identifier: String,
//     pub function_decl: Box<Option<FunDecl>>
// }

#[derive(Debug)]
pub enum DeclAST {
    Variable{ var_type: VarTypeAST, identifier: String },
    Function{ type_spec: TypeSpecAST, identifier: String, fun_decl: Box<FunDecl> }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeSpecAST { Void, VarType(VarTypeAST) }

#[derive(Debug, Clone, Copy)]
pub enum VarTypeAST { Int, Float, Bool }

#[derive(Debug)]
pub struct FunDecl {
    pub param_list: Box<ParamsAST>,
    pub code_block: Box<BlockAST>
}

#[derive(Debug)]
pub struct ParamsAST {
    pub param_list: Option<Box<ParamListAST>>
}

#[derive(Debug)]
pub struct ParamListAST {
    pub param: Box<ParamAST>,
    pub param_list_prime: Box<Option<ParamListPrimeAST>>
}

#[derive(Debug)]
pub struct ParamListPrimeAST {
    pub param: Box<ParamAST>,
    pub param_list_prime: Box<Option<ParamListPrimeAST>>
}

#[derive(Debug)]
pub struct ParamAST {
    pub var_type: VarTypeAST,
    pub identifier: String
}

#[derive(Debug)]
pub struct BlockAST {
    pub local_decls: Box<Option<LocalDeclsAST>>,
    pub stmt_list: Box<Option<StmtListAST>>
}

#[derive(Debug)]
pub struct LocalDeclsAST {
    pub local_decl: Box<LocalDeclAST>,
    pub local_decls: Box<Option<LocalDeclsAST>>
}

#[derive(Debug)]
pub struct LocalDeclAST {
    pub var_type: VarTypeAST,
    pub identifier: String
}

#[derive(Debug)]
pub struct StmtListAST {
    pub stmt: Box<StmtAST>,
    pub stmt_list: Box<Option<StmtListAST>>
}

#[derive(Debug)]
pub enum StmtAST {
    ExprStmt(Option<ExprStmtAST>),
    Block(BlockAST),
    IfStmt(IfStmtAST),
    WhileStmt(WhileStmtAST),
    ReturnStmt(ReturnStmtAST)
}

#[derive(Debug)]
pub struct ExprStmtAST {
    pub expr: Box<ExprAST>
}

#[derive(Debug)]
pub struct WhileStmtAST {
    pub expr: Box<ExprAST>,
    pub stmt: Box<StmtAST>
}

#[derive(Debug)]
pub struct IfStmtAST {
    pub expr: Box<ExprAST>,
    pub block: Box<BlockAST>,
    pub else_stmt: Box<Option<ElseStmtAST>>
}

#[derive(Debug)]
pub struct ElseStmtAST {
    pub block: Box<BlockAST>
}

#[derive(Debug)]
pub struct ReturnStmtAST {
    pub expr_stmt: Option<ExprStmtAST>
}

#[derive(Debug)]
pub enum ExprAST {
    Assignment{ identifier: String, expr: Box<ExprAST> }, Logical(LogicalAST)
}

#[derive(Debug)]
pub struct LogicalAST {
    pub compare: Box<CompareAST>,
    pub logical_prime: Box<Option<LogicalPrimeAST>>
}

#[derive(Debug)]
pub enum LogicalOp { Or, And }

#[derive(Debug)]
pub struct LogicalPrimeAST {
    pub op: LogicalOp,
    pub compare: Box<CompareAST>,
    pub logical_prime: Box<Option<LogicalPrimeAST>>
}

#[derive(Debug)]
pub struct CompareAST {
    pub math: Box<MathAST>,
    pub compare_prime: Box<Option<ComparePrimeAST>>
}

#[derive(Debug)]
pub enum CompareOp { Equal, NotEqual, LtEq, Lt, GtEq, Gt }

#[derive(Debug)]
pub struct ComparePrimeAST {
    pub op: CompareOp,
    pub math: Box<MathAST>,
    pub compare_prime: Box<Option<ComparePrimeAST>>
}

#[derive(Debug)]
pub struct MathAST {
    pub unary: Box<UnaryAST>,
    pub math_prime: Box<Option<MathPrimeAST>>
}

#[derive(Debug)]
pub enum MathOp { Plus, Minus, Mult, Div, Mod }

#[derive(Debug)]
pub struct MathPrimeAST {
    pub op: MathOp,
    pub unary: Box<UnaryAST>,
    pub math_prime: Box<Option<MathPrimeAST>>
}

#[derive(Debug)]
pub enum UnaryOp { Minus, Not, None }

#[derive(Debug)]
pub struct UnaryAST {
    pub op: UnaryOp,
    pub literals: Box<LiteralAST>
}

#[derive(Debug)]
pub enum LiteralAST {
    Expr(ExprAST), FunctionCall{identifier: String, args: Box<ArgsAST> }, Identity(String), IntegerLiteral(String), FloatLiteral(String), BoolLiteral(String) 
}

#[derive(Debug)]
pub struct ArgsAST {
    pub args_list: Box<Option<ArgsListAST>>
}

#[derive(Debug)]
pub struct ArgsListAST {
    pub expr: Box<ExprAST>,
    pub args_list: Box<Option<ArgsListPrimeAST>>
}

#[derive(Debug)]
pub struct ArgsListPrimeAST {
    pub expr: Box<ExprAST>,
    pub args_list_prime: Box<Option<ArgsListPrimeAST>>
}