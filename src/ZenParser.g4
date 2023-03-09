parser grammar ZenParser;
options { tokenVocab=ZenLexer; }



// asignments
// value1   : value0
// value2   : value0.value1
// value0   : Type( ... ) ...
// value3   : call( ... ) ...
// { }      : call( ... ) ...
// { }      : ...value0
// which is the same as 
// IDENTIFIER COLON expr

assignment  : IDENTIFIER COLON expr;

// `...`value0
// `...`Type( ... ) ...
// `...`Type
// `...`call( ... ) ...
// wich is the same as
// ELLIPSIS expr

spread      : ELLIPSIS expr;


// this looks wrong
// value0.value1
// value0.Type
// value0.Type( ... ) ...
// value0.call( ... ) ...
// call().value0
// Type
// which is the same as
// expr DOT IDENTIFIER
accessor   : expr DOT expr;

statement   : expr (COMMA? expr)*;

arguments   : L_PAREN statement R_PAREN;

body        : L_CURLY statement R_CURLY;

call        : IDENTIFIER arguments body?;

typeDef     : IDENTIFIER body;

expr:
    assignment // I don't think this works
    | Literal
    | spread
    | accessor
    | call
    | typeDef
    ;

program: statement EOF;


