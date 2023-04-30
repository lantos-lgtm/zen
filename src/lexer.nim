import strutils, sequtils, json

import token

type
  Lexer* = object
    input*: string
    pos*: int
    finished*: bool

proc newLexer*(input: string): Lexer =
  Lexer(input: input, pos: 0, finished: false)

proc nextChar(lex: Lexer): Option[char] =
  if lex.pos < len(lex.input):
    some(lex.input[lex.pos])
  else:
    none(char)

proc startsWith(lex: Lexer, s: string): bool =
  lex.input.startsWith(s, lex.pos)

proc readWhile(lex: var Lexer, test: proc(ch: char): bool): string =
  var result = ""
  while true:
    let ch = lex.nextChar()
    if ch.isNone or not test(ch.get()):
      break
    result.add(ch.get())
    lex.pos += 1
  result

proc readNumber(lex: var Lexer): Token =
  let s = lex.readWhile(proc(ch: char): bool =
    ch.isDigit() or ch in {'.'} or ch in ['e', 'x', 'o', 'b', '+', '-'])
  Token(kind: NumberLiteral, value: s)

proc readString(lex: var Lexer): Token =
  let s = lex.readWhile(proc(ch: char): bool = ch notin {'"', '\n'})
  lex.pos += 1
  Token(kind: StringLiteral, value: s)

proc readChar(lex: var Lexer): Token =
  let s = lex.readWhile(proc(ch: char): bool = ch notin {'\'', '\n'})
  Token(kind: CharLiteral, value: s[0])

proc readIdentifier(lex: var Lexer): Token =
  if lex.startsWith("true"):
    lex.pos += 4
    return Token(kind: BoolLiteral, value: true)
  if lex.startsWith("false"):
    lex.pos += 5
    return Token(kind: BoolLiteral, value: false)

  let s = lex.readWhile(proc(ch: char): bool = ch.isAlphaNumeric() or ch == '_')
  Token(kind: Identifier, value: s)

proc readWhitespace(lex: var Lexer): Token =
  let newline = lex.readWhile(proc(ch: char): bool = ch in {'\r', '\n', '\uA0'})
  if newline.len > 0:
    return Token(kind: NewLine, value: newline.len)

  let whiteSpace = lex.readWhile(proc(ch: char): bool = ch.isSpace())
  if whiteSpace.len > 0:
    return Token(kind: WhiteSpace, value: whiteSpace.len)

  Token(kind: WhiteSpace, value: 0)

proc readComment(lex: var Lexer): Token =
  lex.pos += 2
  let comment = lex.readWhile(proc(ch: char): bool = ch != '\n')
  Token(kind: Comment, value: comment)

iterator next*(lex: var Lexer): Token =
  let white_space = lex.readWhitespace()
  if white_space.kind != WhiteSpace or white_space.value != 0:
    yield white_space

  while true:
    case lex.nextChar()
    of None:
      if not lex.finished:
        lex.finished = true
        yield Token(kind: EndOfFile)
      else:
        break
    of some(ch):
      case ch
      of '{':
        lex.pos += 1
        yield Token(kind: CurlyBraceOpen)
      of '}':
        lex.pos += 1
        yield Token(kind: CurlyBraceClose)
      of '(':
        lex.pos += 1
        yield
        yield Token(kind: ParenOpen)
      of ')':
        lex.pos += 1
        yield Token(kind: ParenClose)
      of ':':
        lex.pos += 1
        yield Token(kind: Colon)
      of ',':
        lex.pos += 1
        yield Token(kind: Comma)
      of '"':
        lex.pos += 1
        yield lex.readString()
      of '\'':
        lex.pos += 1
        yield lex.readChar()
      of '/':
        if lex.startsWith("//"):
          yield lex.readComment()
        else:
          lex.pos += 1
          yield Token(kind: Divide)
      of '.':
        if lex.startsWith("..."):
          lex.pos += 3
          yield Token(kind: Ellipse)
        else:
          lex.pos += 1
          yield Token(kind: Dot)
      of '&':
        if lex.startsWith("&&"):
          lex.pos += 2
          yield Token(kind: And)
        else:
          lex.pos += 1
          yield Token(kind: BitwiseAnd)
      of '|':
        if lex.startsWith("||"):
          lex.pos += 2
          yield Token(kind: Or)
        else:
          lex.pos += 1
          yield Token(kind: BitwiseOr)
      of '^':
        lex.pos += 1
        yield Token(kind: BitwiseXor)
      of '~':
        lex.pos += 1
        yield Token(kind: BitwiseNot)
      of '+':
        lex.pos += 1
        yield Token(kind: Plus)
      of '-':
        lex.pos += 1
        yield Token(kind: Minus)
      of '*':
        lex.pos += 1
        yield Token(kind: Multiply)
      of '%':
        lex.pos += 1
        yield Token(kind: Modulo)
      of '!':
        if lex.startsWith("!="):
          lex.pos += 2
          yield Token(kind: NotEqual)
        else:
          lex.pos += 1
          yield Token(kind: Not)
      of '>':
        if lex.startsWith(">="):
          lex.pos += 2
          yield Token(kind: GreaterThanOrEqual)
        else:
          lex.pos += 1
          yield Token(kind: GreaterThan)
      of '<':
        if lex.startsWith("<="):
          lex.pos += 2
          yield Token(kind: LessThanOrEqual)
        else:
          lex.pos += 1
          yield Token(kind: LessThan)
      of _:
        if ch.isDigit():
          yield lex.readNumber()
        elif ch.isAlpha():
          yield lex.readIdentifier()
        else:
          raise newException(ValueError, "Unexpected character: " & $ch)
