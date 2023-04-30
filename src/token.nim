import json

type
  TokenKind = enum
    # Literals
    StringLiteral,
    NumberLiteral,
    CharLiteral,
    BoolLiteral,
    Identifier,

    # Binary
    Colon,
    Dot,

    # Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    # Logical
    And,
    Or,

    # Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,

    # Comparison
    Equality,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    # Unary
    Not,
    Ellipse,

    # Group
    CurlyBraceOpen,
    CurlyBraceClose,
    ParenOpen,
    ParenClose,

    # Markup
    Comma,
    Comment,
    WhiteSpace,
    NewLine,
    EndOfFile

  Token = object
    case kind: TokenKind
    of StringLiteral, NumberLiteral, Identifier, Comment:
      value: string
    of CharLiteral:
      value: char
    of BoolLiteral:
      value: bool
    of WhiteSpace, NewLine:
      value: int
    else: 
      discard

proc `$`*(t: Token): string =
  case t.kind
  of StringLiteral, NumberLiteral, Identifier, Comment:
    $t.kind & "(" & t.value & ")"
  of CharLiteral:
    $t.kind & "(" & $t.value & ")"
  of BoolLiteral:
    $t.kind & "(" & $t.value & ")"
  of WhiteSpace, NewLine:
    $t.kind & "(" & $t.value & ")"
  else:
    $t.kind

proc to(json: JsonNode, t: Token) =
  case t.kind
  of StringLiteral, NumberLiteral, Identifier, Comment:
    json["value"] = %t.value
  of CharLiteral:
    json["value"] = %$t.value
  of BoolLiteral:
    json["value"] = %t.value
  of WhiteSpace, NewLine:
    json["value"] = %t.value
  else:
    discard
  json["kind"] = %$t.kind

proc from(json: JsonNode): Token =
  result.kind = TokenKind(json["kind"].getStr)
  case result.kind
  of StringLiteral, NumberLiteral, Identifier, Comment:
    result.value = json["value"].getStr
  of CharLiteral:
    result.value = json["value"].getStr[0]
  of BoolLiteral:
    result.value = json["value"].getBool
  of WhiteSpace, NewLine:
    result.value = json["value"].getInt
  else:
    discard
