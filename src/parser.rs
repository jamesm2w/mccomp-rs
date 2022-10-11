use std::{collections::VecDeque, fmt::Display};

use crate::lexer::{Token, TokenType};

use self::ast::{
    ArgsAST, ArgsListAST, ArgsListPrimeAST, BlockAST, CompareAST, ComparePrimeAST, DeclAST,
    DeclListAST, DeclListPrimeAST, ElseStmtAST, ExprAST, ExprStmtAST, ExternAST, ExternListAST,
    FunDecl, IfStmtAST, LiteralAST, LocalDeclAST, LocalDeclsAST, LogicalAST, LogicalPrimeAST,
    MathAST, MathPrimeAST, ParamAST, ParamListAST, ParamListPrimeAST, ParamsAST, ProgramAST,
    ReturnStmtAST, StmtAST, StmtListAST, TypeSpecAST, UnaryAST, VarTypeAST, WhileStmtAST,
};

use self::display::ASTNode;

pub mod ast;
pub mod display;

#[derive(Debug)]
pub struct Parser {
    tree: Box<ProgramAST>, // tree struct in here
}

impl Parser {
    pub fn parse(buffer: Vec<Token>) -> Self {
        let mut token_buffer = VecDeque::from(buffer);
        Parser {
            tree: parse_program(&mut token_buffer),
        }
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tree.print(String::from(""), String::from("")))
    }
}

fn get_next_token(token_buffer: &mut VecDeque<Token>) -> Token {
    token_buffer.pop_front().unwrap_or(Token::default())
}

fn peek_next_token(token_buffer: &mut VecDeque<Token>) -> &Token {
    token_buffer.get(0).unwrap()
}

// Program ::= ExternList DeclList
//           | DeclList
pub fn parse_program(token_buffer: &mut VecDeque<Token>) -> Box<ProgramAST> {
    let token = peek_next_token(token_buffer);

    let extern_list = if token.token_type == TokenType::Extern {
        parse_externlist(token_buffer)
    } else {
        Box::new(None)
    };

    let token = peek_next_token(token_buffer);
    let decl_list = match token.token_type {
        TokenType::Void | TokenType::Bool | TokenType::Float | TokenType::Integer => {
            parse_decllist(token_buffer)
        }
        _ => panic!("Syntax Error: Expected void|bool|float|int got {:?}", token),
    };

    Box::new(ProgramAST {
        decl_list,
        extern_list,
    })
}

pub fn parse_externlist(token_buffer: &mut VecDeque<Token>) -> Box<Option<ExternListAST>> {
    if peek_next_token(token_buffer).token_type == TokenType::Extern {
        let extern_item = parse_extern(token_buffer);

        let extern_list = parse_externlist(token_buffer);

        Box::new(Some(ExternListAST {
            extern_item,
            extern_list,
        }))
    } else {
        Box::new(None)
    }
}

pub fn parse_extern(token_buffer: &mut VecDeque<Token>) -> Box<ExternAST> {
    if get_next_token(token_buffer).token_type != TokenType::Extern {
        panic!("Syntax Error: Expected 'extern'");
    }

    let type_spec = parse_type_spec(token_buffer);
    let identifier = get_next_token(token_buffer);
    if identifier.token_type != TokenType::Identity {
        panic!("Expected identifier not {:?}", identifier.token_type);
    }
    let identifier = identifier.lexeme.clone();

    let lb = get_next_token(token_buffer);
    if lb.token_type != TokenType::LParen {
        panic!("Syntax Error: Expected '(' got {:?}", lb)
    }

    let params = parse_params(token_buffer);

    let rb = get_next_token(token_buffer);
    if rb.token_type != TokenType::RParen {
        panic!("Syntax Error: Expected ')' got {:?}", rb)
    }

    let sc = get_next_token(token_buffer);
    if sc.token_type != TokenType::Scolon {
        panic!("Syntax Error: Expected ';' got {:?}", sc);
    }

    Box::new(ExternAST {
        type_spec,
        identifier,
        params,
    })
}

pub fn parse_type_spec(token_buffer: &mut VecDeque<Token>) -> TypeSpecAST {
    match peek_next_token(token_buffer).token_type {
        TokenType::Void => {
            get_next_token(token_buffer);
            TypeSpecAST::Void
        }
        TokenType::Bool | TokenType::Float | TokenType::Integer => {
            TypeSpecAST::VarType(parse_var_type(token_buffer))
        }
        _ => panic!("Expected void|bool|float|int"),
    }
}

pub fn parse_var_type(token_buffer: &mut VecDeque<Token>) -> VarTypeAST {
    match peek_next_token(token_buffer).token_type {
        TokenType::Bool => {
            get_next_token(token_buffer);
            VarTypeAST::Bool
        }
        TokenType::Float => {
            get_next_token(token_buffer);
            VarTypeAST::Float
        }
        TokenType::Integer => {
            get_next_token(token_buffer);
            VarTypeAST::Int
        }
        _ => panic!("Expected a bool float or int here"),
    }
}

pub fn parse_decllist(token_buffer: &mut VecDeque<Token>) -> Box<DeclListAST> {
    let next = peek_next_token(token_buffer);
    match next.token_type {
        TokenType::Void | TokenType::Bool | TokenType::Float | TokenType::Integer => {
            let decl = parse_decl(token_buffer);
            let decl_list_prime = parse_decllist_prime(token_buffer);

            Box::new(DeclListAST {
                decl,
                decl_list_prime,
            })
        }
        _ => panic!("Expected void|bool|float|int in declaration list"),
    }
}

pub fn parse_decl(token_buffer: &mut VecDeque<Token>) -> Box<DeclAST> {
    let type_spec = parse_type_spec(token_buffer);
    match type_spec {
        TypeSpecAST::Void => {
            let identifier = get_next_token(token_buffer);
            if identifier.token_type != TokenType::Identity {
                panic!("Expected identifier not {:?}", identifier.token_type)
            }
            let identifier = identifier.lexeme.clone();

            let fun_decl = parse_fun_decl(token_buffer);

            Box::new(DeclAST::Function {
                type_spec,
                identifier,
                fun_decl,
            })
        }
        TypeSpecAST::VarType(ref var_type) => {
            let identifier = get_next_token(token_buffer);
            if identifier.token_type != TokenType::Identity {
                panic!("Expected identifier not {:?}", identifier.token_type)
            }
            let identifier = identifier.lexeme.clone();

            match peek_next_token(token_buffer).token_type {
                TokenType::LParen => {
                    let fun_decl = parse_fun_decl(token_buffer);

                    Box::new(DeclAST::Function {
                        type_spec,
                        identifier,
                        fun_decl,
                    })
                }
                TokenType::Scolon => {
                    get_next_token(token_buffer);
                    Box::new(DeclAST::Variable {
                        var_type: *var_type,
                        identifier,
                    })
                }
                _ => panic!("Expected '(' or ';' "),
            }
        }
    }
}

pub fn parse_decllist_prime(token_buffer: &mut VecDeque<Token>) -> Box<Option<DeclListPrimeAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Void | TokenType::Bool | TokenType::Float | TokenType::Integer => {
            let decl = parse_decl(token_buffer);
            let decl_list_prime = parse_decllist_prime(token_buffer);

            Box::new(Some(DeclListPrimeAST {
                decl,
                decl_list_prime,
            }))
        }
        TokenType::EOF => Box::new(None),
        _ => panic!(
            "Expected void|bool|float|int or EOF next token was {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_fun_decl(token_buffer: &mut VecDeque<Token>) -> Box<FunDecl> {
    let lb = get_next_token(token_buffer);
    if lb.token_type != TokenType::LParen {
        panic!("Expected '('");
    }

    let params = parse_params(token_buffer);

    let rb = get_next_token(token_buffer);
    if rb.token_type != TokenType::RParen {
        panic!("Expected ')'");
    }

    let block = parse_block(token_buffer);

    Box::new(FunDecl {
        param_list: params,
        code_block: block,
    })
}

pub fn parse_params(token_buffer: &mut VecDeque<Token>) -> Box<ParamsAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::RParen => Box::new(ParamsAST { param_list: None }),

        TokenType::Void => {
            get_next_token(token_buffer);
            Box::new(ParamsAST { param_list: None })
        }

        TokenType::Bool | TokenType::Float | TokenType::Integer => {
            let param_list = Some(parse_paramlist(token_buffer));
            Box::new(ParamsAST { param_list })
        }
        _ => panic!("Expected '(', void or bool|float|int"),
    }
}

pub fn parse_paramlist(token_buffer: &mut VecDeque<Token>) -> Box<ParamListAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Bool | TokenType::Float | TokenType::Integer => {
            let param = parse_param(token_buffer);
            let param_list_prime = parse_paramlist_prime(token_buffer);
            Box::new(ParamListAST {
                param,
                param_list_prime,
            })
        }
        _ => panic!("Seriously expected bool|float|int here!"),
    }
}

pub fn parse_param(token_buffer: &mut VecDeque<Token>) -> Box<ParamAST> {
    let var_type = parse_var_type(token_buffer);

    let identifier = get_next_token(token_buffer);
    if identifier.token_type != TokenType::Identity {
        panic!("Expected an identifier not {:?}", identifier.token_type)
    }
    let identifier = identifier.lexeme.clone();

    Box::new(ParamAST {
        var_type,
        identifier,
    })
}

pub fn parse_paramlist_prime(token_buffer: &mut VecDeque<Token>) -> Box<Option<ParamListPrimeAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Comma => {
            get_next_token(token_buffer);
            let param = parse_param(token_buffer);
            let param_list_prime = parse_paramlist_prime(token_buffer);
            Box::new(Some(ParamListPrimeAST {
                param,
                param_list_prime,
            }))
        }
        TokenType::RParen => Box::new(None),
        _ => panic!("Expected ',' or ')'"),
    }
}

pub fn parse_block(token_buffer: &mut VecDeque<Token>) -> Box<BlockAST> {
    // while token_buffer.len() > 1 { // just parse to EOF for testing
    //     token_buffer.pop_front();
    // }
    // Box::new(BlockAST { local_decls: Box::new(None), stmt_list: Box::new(None) })

    let lbrace = get_next_token(token_buffer);
    if lbrace.token_type != TokenType::LBrace {
        panic!("Syntax Error: Expected '{{' got {:?}", lbrace);
    }

    let local_decls = parse_local_decls(token_buffer);

    let stmt_list = parse_stmt_list(token_buffer);

    let rbrace = get_next_token(token_buffer);
    if rbrace.token_type != TokenType::RBrace {
        panic!("Syntax Error: Expected '}}' got {:?}", rbrace);
    }

    Box::new(BlockAST {
        local_decls,
        stmt_list,
    })
}

pub fn parse_local_decls(token_buffer: &mut VecDeque<Token>) -> Box<Option<LocalDeclsAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Bool | TokenType::Float | TokenType::Integer => {
            let local_decl = parse_local_decl(token_buffer);
            let local_decls = parse_local_decls(token_buffer);

            Box::new(Some(LocalDeclsAST {
                local_decl,
                local_decls,
            }))
        }
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus
        | TokenType::Return
        | TokenType::IfStmt
        | TokenType::While
        | TokenType::Scolon
        | TokenType::RBrace
        | TokenType::LBrace => Box::new(None),
        _ => panic!(
            "Syntax Error: Expected bool|float|int or another statement got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_local_decl(token_buffer: &mut VecDeque<Token>) -> Box<LocalDeclAST> {
    let var_type = parse_var_type(token_buffer);
    let identifier = get_next_token(token_buffer);
    if identifier.token_type != TokenType::Identity {
        panic!("Expected identifier got {:?}", identifier)
    }
    let identifier = identifier.lexeme;

    let sc = get_next_token(token_buffer);
    if sc.token_type != TokenType::Scolon {
        panic!("Expected ';' got {:?}", sc);
    }

    Box::new(LocalDeclAST {
        var_type,
        identifier,
    })
}

pub fn parse_stmt_list(token_buffer: &mut VecDeque<Token>) -> Box<Option<StmtListAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::RBrace => Box::new(None),
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus
        | TokenType::Return
        | TokenType::IfStmt
        | TokenType::While
        | TokenType::Scolon
        | TokenType::LBrace => {
            let stmt = parse_stmt(token_buffer);
            let stmt_list = parse_stmt_list(token_buffer);

            Box::new(Some(StmtListAST { stmt, stmt_list }))
        }
        _ => panic!(
            "Expected '}}' or another statement. Got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_stmt(token_buffer: &mut VecDeque<Token>) -> Box<StmtAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus
        | TokenType::Scolon => {
            let expr_stmt = parse_expr_stmt(token_buffer);

            Box::new(StmtAST::ExprStmt(expr_stmt))
        }
        TokenType::Return => Box::new(StmtAST::ReturnStmt(parse_return_stmt(token_buffer))),
        TokenType::IfStmt => Box::new(StmtAST::IfStmt(parse_if_stmt(token_buffer))),
        TokenType::While => Box::new(StmtAST::WhileStmt(parse_while_stmt(token_buffer))),
        TokenType::LBrace => Box::new(StmtAST::Block(*parse_block(token_buffer))),
        _ => panic!("Expected statement got {:?}", peek_next_token(token_buffer)),
    }
}

pub fn parse_expr_stmt(token_buffer: &mut VecDeque<Token>) -> Option<ExprStmtAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Scolon => None,
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => {
            let expr = parse_expr(token_buffer);
            let sc = get_next_token(token_buffer);
            if sc.token_type != TokenType::Scolon {
                panic!("Expected ';' got {:?}", sc)
            }

            Some(ExprStmtAST { expr })
        }
        _ => panic!(
            "Expected ';' or expression got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_while_stmt(token_buffer: &mut VecDeque<Token>) -> WhileStmtAST {
    let while_word = get_next_token(token_buffer);
    if while_word.token_type != TokenType::While {
        panic!("Expected 'while' got {:?}", while_word)
    }

    let lb = get_next_token(token_buffer);
    if lb.token_type != TokenType::LParen {
        panic!("Expected '(' got {:?}", lb);
    }

    let expr = parse_expr(token_buffer);

    let rb = get_next_token(token_buffer);
    if rb.token_type != TokenType::RParen {
        panic!("Expected ')' got {:?}", rb);
    }

    let block = parse_stmt(token_buffer);

    WhileStmtAST { expr, stmt: block }
}

pub fn parse_if_stmt(token_buffer: &mut VecDeque<Token>) -> IfStmtAST {
    let if_word = get_next_token(token_buffer);
    if if_word.token_type != TokenType::IfStmt {
        panic!("Expected 'if' got {:?}", if_word);
    }
    let lb = get_next_token(token_buffer);
    if lb.token_type != TokenType::LParen {
        panic!("Expected '(' got {:?}", lb);
    }

    let expr = parse_expr(token_buffer);

    let rb = get_next_token(token_buffer);
    if rb.token_type != TokenType::RParen {
        panic!("Expected ')' got {:?}", rb);
    }

    let block = parse_block(token_buffer);

    let else_stmt = parse_else_stmt(token_buffer);

    IfStmtAST {
        expr,
        block,
        else_stmt,
    }
}

pub fn parse_else_stmt(token_buffer: &mut VecDeque<Token>) -> Box<Option<ElseStmtAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Else => {
            get_next_token(token_buffer);
            let block = parse_block(token_buffer);
            Box::new(Some(ElseStmtAST { block }))
        }
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus
        | TokenType::Return
        | TokenType::IfStmt
        | TokenType::While
        | TokenType::Scolon
        | TokenType::RBrace
        | TokenType::LBrace => Box::new(None),
        _ => panic!(
            "Expected 'else' or another statement got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_return_stmt(token_buffer: &mut VecDeque<Token>) -> ReturnStmtAST {
    let return_word = get_next_token(token_buffer);
    if return_word.token_type != TokenType::Return {
        panic!("Expected return got {:?}", return_word)
    }

    let expr_stmt = parse_expr_stmt(token_buffer);

    ReturnStmtAST { expr_stmt }
}

pub fn parse_expr(token_buffer: &mut VecDeque<Token>) -> Box<ExprAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => Box::new(ExprAST::Logical(parse_logical(token_buffer))),
        TokenType::Identity => {
            if token_buffer.get(1).is_some()
                && token_buffer.get(1).unwrap().token_type == TokenType::Assign
            {
                // .contains(|tok| tok.token_type == TokenType::Assign) {

                let identifier = get_next_token(token_buffer).lexeme;
                get_next_token(token_buffer); // Consume the "=" as well
                let expr = parse_expr(token_buffer);

                Box::new(ExprAST::Assignment { identifier, expr })
            } else {
                Box::new(ExprAST::Logical(parse_logical(token_buffer)))
            }
        }
        _ => panic!(
            "Expected identifier or expression got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_logical(token_buffer: &mut VecDeque<Token>) -> LogicalAST {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => {
            let compare = parse_compare(token_buffer);
            let logical_prime = parse_logical_prime(token_buffer);

            LogicalAST {
                compare,
                logical_prime,
            }
        }
        _ => panic!(
            "Expected expression got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_logical_prime(token_buffer: &mut VecDeque<Token>) -> Box<Option<LogicalPrimeAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Comma | TokenType::RParen | TokenType::Scolon => Box::new(None),
        TokenType::LOr => {
            get_next_token(token_buffer);
            let compare = parse_compare(token_buffer);
            let logical_prime = parse_logical_prime(token_buffer);
            Box::new(Some(LogicalPrimeAST {
                op: ast::LogicalOp::Or,
                compare,
                logical_prime,
            }))
        }
        TokenType::LAnd => {
            get_next_token(token_buffer);
            let compare = parse_compare(token_buffer);
            let logical_prime = parse_logical_prime(token_buffer);
            Box::new(Some(LogicalPrimeAST {
                op: ast::LogicalOp::And,
                compare,
                logical_prime,
            }))
        }
        _ => panic!(
            "Expected end of expr or || or && got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_compare(token_buffer: &mut VecDeque<Token>) -> Box<CompareAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => {
            let math = parse_math(token_buffer);
            let compare_prime = parse_compare_prime(token_buffer);

            Box::new(CompareAST {
                math,
                compare_prime,
            })
        }
        _ => panic!(
            "Expected expression start got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_compare_prime(token_buffer: &mut VecDeque<Token>) -> Box<Option<ComparePrimeAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Comma
        | TokenType::RParen
        | TokenType::LOr
        | TokenType::LAnd
        | TokenType::Scolon => Box::new(None),
        op @ (TokenType::Gt
        | TokenType::Ge
        | TokenType::Lt
        | TokenType::Le
        | TokenType::Ne
        | TokenType::Eq) => {
            get_next_token(token_buffer);
            let math = parse_math(token_buffer);
            let compare_prime = parse_compare_prime(token_buffer);

            Box::new(Some(ComparePrimeAST {
                op: match op {
                    TokenType::Gt => ast::CompareOp::Gt,
                    TokenType::Ge => ast::CompareOp::GtEq,
                    TokenType::Lt => ast::CompareOp::Lt,
                    TokenType::Le => ast::CompareOp::LtEq,
                    TokenType::Ne => ast::CompareOp::NotEqual,
                    TokenType::Eq => ast::CompareOp::Equal,
                    _ => unimplemented!(),
                },
                math,
                compare_prime,
            }))
        }
        _ => panic!(
            "expected end of expr or operator (<, >, <=, >=, !=, ==) got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_math(token_buffer: &mut VecDeque<Token>) -> Box<MathAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => {
            let unary = parse_unary(token_buffer);
            let math_prime = parse_math_prime(token_buffer);

            Box::new(MathAST { unary, math_prime })
        }
        _ => panic!(
            "Expected expression got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_math_prime(token_buffer: &mut VecDeque<Token>) -> Box<Option<MathPrimeAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Comma
        | TokenType::RParen
        | TokenType::LOr
        | TokenType::LAnd
        | TokenType::Scolon
        | TokenType::Gt
        | TokenType::Ge
        | TokenType::Lt
        | TokenType::Le
        | TokenType::Ne
        | TokenType::Eq => Box::new(None),
        op @ (TokenType::Minus
        | TokenType::Mod
        | TokenType::Div
        | TokenType::Mult
        | TokenType::Plus) => {
            get_next_token(token_buffer);
            let unary = parse_unary(token_buffer);
            let math_prime = parse_math_prime(token_buffer);

            Box::new(Some(MathPrimeAST {
                op: match op {
                    TokenType::Minus => ast::MathOp::Minus,
                    TokenType::Mod => ast::MathOp::Mod,
                    TokenType::Div => ast::MathOp::Div,
                    TokenType::Mult => ast::MathOp::Mult,
                    TokenType::Plus => ast::MathOp::Plus,
                    _ => unimplemented!("Not sure how you got here"),
                },
                unary,
                math_prime,
            }))
        }
        _ => panic!(
            "Expected expr continuation or an operator got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_unary(token_buffer: &mut VecDeque<Token>) -> Box<UnaryAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Not => {
            get_next_token(token_buffer);
            let literals = parse_literals(token_buffer);
            Box::new(UnaryAST {
                op: ast::UnaryOp::Not,
                literals,
            })
        }
        TokenType::Minus => {
            get_next_token(token_buffer);
            let literals = parse_literals(token_buffer);
            Box::new(UnaryAST {
                op: ast::UnaryOp::Minus,
                literals,
            })
        }
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen => {
            let literals = parse_literals(token_buffer);
            Box::new(UnaryAST {
                op: ast::UnaryOp::None,
                literals,
            })
        }
        _ => panic!(
            "expected val or unary op got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_literals(token_buffer: &mut VecDeque<Token>) -> Box<LiteralAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit => {
            Box::new(LiteralAST::BoolLiteral(get_next_token(token_buffer).lexeme))
        }
        TokenType::FloatLit => Box::new(LiteralAST::FloatLiteral(
            get_next_token(token_buffer).lexeme,
        )),
        TokenType::IntLit => Box::new(LiteralAST::IntegerLiteral(
            get_next_token(token_buffer).lexeme,
        )),
        TokenType::LParen => {
            get_next_token(token_buffer);
            let expr = parse_expr(token_buffer);
            let rb = get_next_token(token_buffer);
            if rb.token_type != TokenType::RParen {
                panic!("Expected ')' got {:?}", rb)
            }

            Box::new(LiteralAST::Expr(*expr))
        }
        TokenType::Identity => {
            let identity = get_next_token(token_buffer);

            match peek_next_token(token_buffer).token_type {
                TokenType::LParen => {
                    get_next_token(token_buffer);
                    let args = parse_args(token_buffer);
                    let rb = get_next_token(token_buffer);
                    if rb.token_type != TokenType::RParen {
                        panic!("Expected ')' got {:?}", rb)
                    }

                    Box::new(LiteralAST::FunctionCall {
                        identifier: identity.lexeme,
                        args,
                    })
                }
                TokenType::Comma
                | TokenType::RBrace
                | TokenType::Minus
                | TokenType::Mod
                | TokenType::Div
                | TokenType::Mult
                | TokenType::Plus
                | TokenType::Gt
                | TokenType::Ge
                | TokenType::Lt
                | TokenType::Le
                | TokenType::Ne
                | TokenType::Eq
                | TokenType::LAnd
                | TokenType::LOr
                | TokenType::Scolon => Box::new(LiteralAST::Identity(identity.lexeme)),
                _ => panic!(
                    "Expected function call or continuing expr got {:?}",
                    peek_next_token(token_buffer)
                ),
            }
        }
        _ => panic!("unexpected {:?}", peek_next_token(token_buffer)),
    }
}

pub fn parse_args(token_buffer: &mut VecDeque<Token>) -> Box<ArgsAST> {
    match peek_next_token(token_buffer).token_type {
        TokenType::RParen => Box::new(ArgsAST {
            args_list: Box::new(None),
        }),
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => {
            let args_list = parse_args_list(token_buffer);
            Box::new(ArgsAST { args_list })
        }
        _ => panic!(
            "Expected expr or ')' got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_args_list(token_buffer: &mut VecDeque<Token>) -> Box<Option<ArgsListAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::BoolLit
        | TokenType::FloatLit
        | TokenType::IntLit
        | TokenType::Identity
        | TokenType::LParen
        | TokenType::Not
        | TokenType::Minus => {
            let expr = parse_expr(token_buffer);
            let args_list = parse_args_list_prime(token_buffer);

            Box::new(Some(ArgsListAST { expr, args_list }))
        }
        _ => panic!(
            "Expected expr or ')' got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}

pub fn parse_args_list_prime(token_buffer: &mut VecDeque<Token>) -> Box<Option<ArgsListPrimeAST>> {
    match peek_next_token(token_buffer).token_type {
        TokenType::Comma => {
            get_next_token(token_buffer);
            let expr = parse_expr(token_buffer);
            let args_list_prime = parse_args_list_prime(token_buffer);

            Box::new(Some(ArgsListPrimeAST {
                expr,
                args_list_prime,
            }))
        }
        TokenType::RParen => Box::new(None),
        _ => panic!(
            "Expected ',' expr or end of args got {:?}",
            peek_next_token(token_buffer)
        ),
    }
}
