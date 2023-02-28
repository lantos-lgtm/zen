lexer grammar ZenLexer;

// Punctuation

L_CURLY             : '{';
R_CURLY             : '}';
L_PAREN             : '(';
R_PAREN             : ')';
COMMA               : ',';
DOT                 : '.';
ELLIPSIS            : '...';
COLON               : ':';
WHITE_SPACE         :[ \t\n\r\f]+ -> skip ;

IDENTIFIER          : [a-zA-Z_][a-zA-Z0-9_]*;


// Number literals
DIGIT                  : [0-9];
INT_LITERAL            : DIGIT+;

// HEX_DIGIT              : [0-9a-fA-F];
// HEX_LITERAL            : '0' [xX] HEX_DIGIT+;
// OCTAL_DIGIT            : [0-7];
// OCTAL_LITERAL          : '0' OCTAL_DIGIT+;
// BINARY_DIGIT           : [01];
// BINARY_LITERAL         : '0' [bB] BINARY_DIGIT+;

// INTEGERS               : 
//                         HEX_LITERAL
//                         | OCTAL_LITERAL
//                         | BINARY_LITERAL
//                         | INT_LITERAL;

FLOAT_LITERAL          : DIGIT+ '.' DIGIT*
                        | '.' DIGIT+;
                        
STRING_LITERAL         : '"' CHAR* '"';
CHAR                   : ~["\n\r];

LITERAL: 
    INT_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | IDENTIFIER
    ;


EXCLAMATION            : '!';


// Logical
LOGICAL_OR             : '||';
LOGICAL_AND            : '&&';

// Relation operators
EQUALS                 : '==';
NOT_EQUALS             : '!=';
LESS                   : '<';
LESS_OR_EQUALS         : '<=';
GREATER                : '>';
GREATER_OR_EQUALS      : '>=';

// Arithmetic operators

OR                     : '|';
DIV                    : '/';
MOD                    : '%';
LSHIFT                 : '<<';
RSHIFT                 : '>>';
BIT_CLEAR              : '&^';

// Mixed operators

PLUS                   : '+';
MINUS                  : '-';
CARET                  : '^';
STAR                   : '*';
AMPERSAND              : '&';
