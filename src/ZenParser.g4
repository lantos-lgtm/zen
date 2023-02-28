parser grammar ZenParser;
options { tokenVocab=ZenLexer; }





assignment: 
    IDENTIFIER COLON expr
    | ELLIPSIS IDENTIFIER
    ;

assignments: assignment (COMMA? assignment)*;

acccessor: IDENTIFIER DOT expr;

block: expr*;

call:           IDENTIFIER L_PAREN (assignments | LITERAL ) R_PAREN;
callWithBlock:  call L_CURLY block R_CURLY;

callStmt:
    call
    | callWithBlock
    | callGeneric
    ;

typeDef: 
    IDENTIFIER L_CURLY (assignments | LITERAL) R_CURLY
    | L_CURLY (assignments | LITERAL) R_CURLY
    ;

expr:
    assignments
    | typeDef
    | callStmt
    ;


program: block EOF;


