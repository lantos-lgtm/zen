import 
    json,
    tables,
    strformat

type
  Expr = ref object
    case kind: ExprKind
    of Atom:
      atomVal: Atom
    of Unary:
      unaryVal: Unary
    of Binary:
      binaryVal: Binary
    of Group:
      groupVal: Group
    of TypeDef:
      typeDefVal: TypeDef
    of FuncCall:
      funcCallVal: FuncCall
    of EndOfFile: 
      discard

  LiteralKind = enum
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
    CharLiteral,
    OctalLiteral,
    HexLiteral,
    BinaryLiteral,
    StringLiteral

  Literal = object
    case kind: LiteralKind
    of IntLiteral:
      intVal: int64
    of FloatLiteral:
      floatVal: float64
    of BoolLiteral:
      boolVal: bool
    of CharLiteral:
      charVal: char
    of OctalLiteral:
      octalVal: uint32
    of HexLiteral:
      hexVal: uint8
    of BinaryLiteral:
      binaryVal: uint32
    of StringLiteral:
      stringVal: string

  AtomKind = enum
    Identifier,
    Literal

  Atom = object
    case kind: AtomKind
    of Identifier:
      identifierVal: string
    of Literal:
      literalVal: Literal

  UnaryKind = enum
    SpreadExpr

  Unary = object
    kind: UnaryKind
    expr: Expr

  BinaryOp = enum
    Assignment,
    Accessor

  Binary = object
    op: BinaryOp
    left, right: Expr

  GroupOp = enum
    AssignmentBlock,
    StatementBlock,
    ParamBlock,
    AnonymousType

  Group = object
    op: GroupOp
    exprs: seq[Expr]

  TypeDef = object
    name: Expr
    fields: Expr

  FuncCall = object
    name: Expr
    args: Expr
    fields: Option[Expr]
    body: Option[Expr]

  ExprKind = enum
    Atom,
    Unary,
    Binary,
    Group,
    TypeDef,
    FuncCall,
    EndOfFile
