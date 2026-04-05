type token_type =
    | BANG of int   (** end of statement, contains priority *)
    | SEMICOLON  (** not operator *)
    | QUESTION_MARK  (** debug operator *)
    (** parens *)
    | LEFT_PAREN  (** '(' *)
    | RIGHT_PAREN  (** ')' *)
    (** scopes *)
    | LEFT_BRACE  (** '{' *)
    | RIGHT_BRACE  (** '}' *)
    (** arrays *)
    | LEFT_BRACKET  (** '[' *)
    | RIGHT_BRACKET  (** ']' *)
    (** lifetimes *)
    | LEFT_ANGULAR  (** '<' *)
    | RIGHT_ANGULAR  (** '>' *)
    (** booleans *)
    | TRUE
    | FALSE
    | MAYBE
    (** arithmetic *)
    | PLUS
    | MINUS
    | ASTERISK
    | FORWARD_SLASH
    | CARET
    | INCREMENT_OP
    (** signals *)
    | WHEN
    | USE
    (** declarations *)
    | CONST_CONST  (** cannot be changed in any way *)
    | CONST_VAR  (** can be edited but not re-assigned *)
    | VAR_CONST  (** can be re-assigned but not edited *)
    | VAR_VAR  (** can be edited and re-assigned *)
    | CONST_CONST_CONST  (** constant and immutable, affects all users globally forever *)
    | ASSIGNMENT
    (** comparison *)
    | MUCH_LOOSER_CHECK  (** "=" *)
    | LOOSE_CHECK  (** "==" *)
    | PRECISE_CHECK  (** "===" *)
    | MORE_PRECISE_CHECK  (** "====" *)
    (** files *)
    | FILE_DELIM  (** 5+ equal signs *)
    (** functions *)
    | FUNCTION
    | ARROW  (** function foo(a, b) => ... *)
    | RETURN
    | COMMA
    (** types *)
    | COLON
    | INT_T
    | STRING_T
    | CHAR_T
    | DIGIT_T
    | INT9_T
    | INT99_T
    | REGEXP_T
    (** prev/next *)
    | PREVIOUS
    | NEXT
    | CURRENT
    (** imports *)
    | IMPORT
    | EXPORT
    | TO  (** export foo to "filename.db" *)
    (** classes *)
    | CLASS  (** "class" or "className" *)
    | NEW
    (** delete *)
    | DELETE
    (** async/await *)
    | ASYNC
    | AWAIT
    | NOOP
    (** reverse *)
    | REVERSE
    (** literals *)
    | IDENTIFIER of string 
    | STRING of string
    | INTEGER of int
    | FLOAT of float
    | INFINITY  (** usable in lifetimes *)
    (** misc *)
    | UNDEFINED  (** works like in JS; dicts with unset keys return this *)
    (** eof *)
    | EOF;;


type token = {
    ttype: token_type;
    lexeme: string;
    line: int;
    col: int;
}
