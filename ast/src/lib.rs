pub mod meta;
pub mod opcode;

pub use meta::AstMeta;
pub use opcode::Opcode;

/// The AST items that may be in an AST tree generated by Flycatcher's parser.
#[derive(Clone, Debug)]
pub enum Ast {
    /// An identifier literal, such as `my_identifier` or `__my4other2identifier`.
    IdentifierLiteral(String),

    /// Booleans may either be `true` or `false`.
    BooleanLiteral(bool),

    /// A string literal such as "Hello, world!".
    StringLiteral(String),

    /// Integer literals cannot be negative at the parsing phase, since the operator to make them
    /// negative isn't functional until the compilation phase.
    IntegerLiteral(u64),

    /// A floating point number literal, like `42.0` or `4.2e1`.
    FloatLiteral(f64),

    /// An array literal, using the `[]` syntax:
    ///
    /// ```flycatcher
    /// [1, 2, 3]
    /// ```
    ArrayLiteral(Vec<AstMeta>),

    /// A unary expression, (or a prefix expression) such as `-20`.
    UnaryExpr(Opcode, Box<AstMeta>),

    /// A binary expression, such as `2 + 2`.
    BinaryExpr(Opcode, Box<AstMeta>, Box<AstMeta>),

    /// A Subscript expression, such as `my_iden[subscript]`.  The first object is the subject of the
    /// expression, the second object is the value inside of the `[]`, if any.
    SubscriptExpr(Box<AstMeta>, Option<Box<AstMeta>>),

    /// A function call statement, using the call operator:
    ///
    /// ```flycatcher
    /// my_function();
    /// ```
    CallExpr(Box<AstMeta>, Vec<AstMeta>),

    /// A template expression:
    ///
    /// ```flycatcher
    /// my_template<Type1, Type2>
    /// ```
    TemplateExpr(Box<AstMeta>, Vec<AstMeta>),

    /// An `if` statement with any amount of branches.
    IfStmnt {
        /// The expression that the `if` statement evaluates.
        expr: Box<AstMeta>,

        /// The code block in the `if` statement.  It is wrapped in `{}`.
        block: Vec<AstMeta>,

        /// The list of branches in the `if` statement, such as:
        ///
        /// ```flycatcher
        /// if expr {
        /// } else if expr { // branch
        /// } else { // branch
        /// }
        /// ```
        branches: Vec<AstMeta>,
    },

    WhileStmnt {
        /// The expression to evaluate.
        expr: Box<AstMeta>,

        /// A list of child statements in the block after the expression.
        block: Vec<AstMeta>,
    },

    FunctionConstruct {
        /// The construct's name, minus the `@` prefix.
        construct: String,

        /// The name of the function being declared.
        name: Box<AstMeta>,

        /// The type annotation of which the function should return.
        returns: Option<Box<AstMeta>>,

        /// A list of arguments declared in the function construct.  These are expected to be type
        /// declarations using the colon (`:`) operator.
        arguments: Vec<AstMeta>,

        /// The code block after the argument list/return annotation.
        block: Vec<AstMeta>,
    },

    /// A class construct declaration.
    ///
    /// ```flycatcher
    /// @construct my_construct {
    ///
    ///     // ...
    ///
    /// }
    /// ```
    ClassConstruct {
        /// The name of the construct used to initialize the class.
        construct: String,

        /// The name of the class.
        name: Box<AstMeta>,

        /// The code block after the class name, wrapped in `{}`.
        block: Vec<AstMeta>,
    },

    /// A variable construct declaration.
    VariableConstruct {
        /// The name of the construct used to declare the variable.
        construct: String,

        /// The name of the variable being declared.
        name: Box<AstMeta>,

        /// The value the variable is being constructed with.
        value: Box<AstMeta>,
    },

    /// Declares an external function so it may be used within Flycatcher code.
    DeclareStmnt {
        /// The name of the function being declared.
        name: Box<AstMeta>,

        /// A list of arguments being declared.
        arguments: Vec<AstMeta>,

        /// The type that the function returns, if any.
        returns: Option<Box<AstMeta>>,
    },

    /// A statement with `pub` access.
    PubStmnt(Box<AstMeta>),

    /// A statement with `priv` access.
    PrivStmnt(Box<AstMeta>),

    /// A return statement with an optional return value.
    ReturnStmnt(Option<Box<AstMeta>>),

    /// A return statement with an optional label to jump to.
    ContinueStmnt(Option<Box<AstMeta>>),

    /// A break statement with an optional label of a loop to end.
    BreakStmnt(Option<Box<AstMeta>>),

    /// A block statement with a list of child statements.
    Block(Vec<AstMeta>),
}
