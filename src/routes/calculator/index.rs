use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use rocket::{
    http,
    serde::{json::Json, Serialize},
};
use std::ffi::{CStr, CString};
use std::mem;
use utoipa::ToSchema;

// 式を表現する列挙型
// Number: 数値を表す
// BinaryOp: 二項演算を表す（左辺、演算子、右辺）
#[derive(Debug)]
enum Expr {
    Number(f64),
    BinaryOp(Box<Expr>, char, Box<Expr>),
}

// パーサー構造体
// tokens: 入力文字列をトークン（文字）に分解したもの
// pos: 現在解析中のトークンの位置
struct Parser {
    tokens: Vec<char>,
    pos: usize,
}

impl Parser {
    // 新しいパーサーを作成する
    // 入力文字列から空白を除去し、文字のベクターに変換する
    fn new(input: &str) -> Self {
        Parser {
            tokens: input.chars().filter(|c| !c.is_whitespace()).collect(),
            pos: 0,
        }
    }

    // 式全体を解析する
    fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expr()
    }

    // 式を解析する（加算と減算を処理）
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_term()
    }

    // 項を解析する（加算と減算を処理）
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                '+' | '-' => {
                    let op = self.tokens[self.pos];
                    self.pos += 1;
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // 因子を解析する（乗算と除算を処理）
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                '*' | '/' => {
                    let op = self.tokens[self.pos];
                    self.pos += 1;
                    let right = self.parse_primary()?;
                    left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // 基本要素（数値または括弧で囲まれた式）を解析する
    fn parse_primary(&mut self) -> Result<Expr, String> {
        if self.pos >= self.tokens.len() {
            return Err("予期せぬ入力の終わりです".to_string());
        }

        match self.tokens[self.pos] {
            '(' => {
                // 括弧内の式を解析
                self.pos += 1;
                let expr = self.parse_expr()?;
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != ')' {
                    return Err("括弧が閉じられていません".to_string());
                }
                self.pos += 1;
                Ok(expr)
            }
            '0'..='9' | '.' => {
                // 数値を解析
                let mut num_str = String::new();
                while self.pos < self.tokens.len()
                    && (self.tokens[self.pos].is_ascii_digit() || self.tokens[self.pos] == '.')
                {
                    num_str.push(self.tokens[self.pos]);
                    self.pos += 1;
                }
                num_str
                    .parse::<f64>()
                    .map(Expr::Number)
                    .map_err(|e| e.to_string())
            }
            _ => Err(format!("予期せぬトークンです: {}", self.tokens[self.pos])),
        }
    }
}

// コード生成を行う構造体
struct CodeGen {
    context: LLVMContextRef,    // LLVM コンテキスト
    module: LLVMModuleRef,      // LLVM モジュール
    builder: LLVMBuilderRef,    // LLVM IRビルダー
    ee: LLVMExecutionEngineRef, // LLVM 実行エンジン
}

impl CodeGen {
    // 新しいCodeGen構造体を作成する
    fn new(module_name: &str) -> Self {
        unsafe {
            // LLVM コンテキストを作成
            let context = LLVMContextCreate();
            // モジュール名をC言語形式の文字列に変換
            let module_name_c = CString::new(module_name).unwrap();
            // LLVM モジュールを作成
            let module = LLVMModuleCreateWithNameInContext(module_name_c.as_ptr(), context);
            // LLVM IRビルダーを作成
            let builder = LLVMCreateBuilderInContext(context);

            let mut ee = mem::MaybeUninit::uninit();
            let mut error = mem::zeroed();

            // LLVM ターゲットを初期化
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();
            LLVM_InitializeNativeAsmParser();
            // JITコンパイラをリンク
            LLVMLinkInMCJIT();

            // 実行エンジンを作成
            if LLVMCreateExecutionEngineForModule(ee.as_mut_ptr(), module, &mut error) != 0 {
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();
                panic!("実行エンジンの作成に失敗しました: {}", error);
            }

            let ee = ee.assume_init();

            CodeGen {
                context,
                module,
                builder,
                ee,
            }
        }
    }

    // 式をLLVM IRにコンパイルする
    fn compile(&self, expr: &Expr) -> LLVMValueRef {
        unsafe {
            match expr {
                // 数値の場合、LLVM の定数を作成
                Expr::Number(n) => LLVMConstReal(LLVMDoubleTypeInContext(self.context), *n),
                // 二項演算の場合、左辺と右辺を再帰的にコンパイルし、適切な演算を適用
                Expr::BinaryOp(left, op, right) => {
                    let lhs = self.compile(left);
                    let rhs = self.compile(right);
                    let name = CString::new(match op {
                        '+' => "addtmp",
                        '-' => "subtmp",
                        '*' => "multmp",
                        '/' => "divtmp",
                        _ => panic!("不明な演算子です"),
                    })
                    .unwrap();
                    match op {
                        '+' => LLVMBuildFAdd(self.builder, lhs, rhs, name.as_ptr()),
                        '-' => LLVMBuildFSub(self.builder, lhs, rhs, name.as_ptr()),
                        '*' => LLVMBuildFMul(self.builder, lhs, rhs, name.as_ptr()),
                        '/' => LLVMBuildFDiv(self.builder, lhs, rhs, name.as_ptr()),
                        _ => panic!("不明な演算子です"),
                    }
                }
            }
        }
    }

    // メイン関数を作成する
    fn create_main_function(&self) -> LLVMValueRef {
        unsafe {
            // double型を表すLLVM型を取得
            let double_type = LLVMDoubleTypeInContext(self.context);
            // 引数なし、戻り値がdouble型の関数型を作成
            let function_type = LLVMFunctionType(double_type, std::ptr::null_mut(), 0, 0);
            // "main"という名前の関数をモジュールに追加
            let name = CString::new("main").unwrap();
            let function = LLVMAddFunction(self.module, name.as_ptr(), function_type);
            // 関数にエントリーブロックを追加
            let entry_name = CString::new("entry").unwrap();
            let entry = LLVMAppendBasicBlockInContext(self.context, function, entry_name.as_ptr());
            // ビルダーの挿入位置をエントリーブロックの終端に設定
            LLVMPositionBuilderAtEnd(self.builder, entry);
            function
        }
    }

    // コンパイルされた関数を実行する
    fn run(&self, function: LLVMValueRef) -> f64 {
        unsafe {
            // 関数を実行
            let result = LLVMRunFunction(self.ee, function, 0, std::ptr::null_mut());
            // 結果を浮動小数点数に変換
            let double_type = LLVMDoubleTypeInContext(self.context);
            LLVMGenericValueToFloat(double_type, result)
        }
    }
}

// CodeGen構造体のデストラクタ
// LLVMリソースを適切に解放する
impl Drop for CodeGen {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeExecutionEngine(self.ee);
            LLVMContextDispose(self.context);
        }
    }
}

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetCalculatorSuccessResult {
    success: bool,
    message: f64,
}

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetCalculatorErrorResult {
    success: bool,
    error: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get Calculator result.", body = [GetCalculatorSuccessResult]),
        (status = 400, description = "Bad request.", body = [GetCalculatorErrorResult])
    )
)]
#[get("/calculator?<expression>")]
pub fn calculator_index(
    expression: String,
) -> Result<
    (http::Status, Json<GetCalculatorSuccessResult>),
    (http::Status, Json<GetCalculatorErrorResult>),
> {
    // 入力された数式を取得
    let input = &expression;
    // パーサーを作成し、数式を解析
    let mut parser = Parser::new(input);
    let expr = match parser.parse() {
        Ok(expr) => expr,
        Err(err) => {
            return Err((
                http::Status::BadRequest,
                Json(GetCalculatorErrorResult {
                    success: false,
                    error: err, // エラーメッセージを設定
                }),
            ));
        }
    };

    // コード生成器を作成
    let codegen = CodeGen::new("calculator");
    // メイン関数を作成
    let main_function = codegen.create_main_function();
    // 式をコンパイル
    let result = codegen.compile(&expr);
    unsafe {
        // コンパイルした結果を関数の戻り値として設定
        LLVMBuildRet(codegen.builder, result);
    }
    // コンパイルした関数を実行
    let computed_result = codegen.run(main_function);

    Ok((
        http::Status::Ok,
        Json(GetCalculatorSuccessResult {
            success: true,
            message: computed_result,
        }),
    ))
}
