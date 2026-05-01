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
    (** single-line comment *)
    | COMMENT
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
    | UNDEFINED  (** works like in JS; dicts with unset keys return this; also 1/0 *)
    (** eof *)
    | EOF;;


let token_type_to_string = function
  | BANG n -> Printf.sprintf "BANG(%d)" n
  | SEMICOLON -> "SEMICOLON"
  | QUESTION_MARK -> "QUESTION_MARK"
  | LEFT_PAREN -> "LEFT_PAREN"
  | RIGHT_PAREN -> "RIGHT_PAREN"
  | LEFT_BRACE -> "LEFT_BRACE"
  | RIGHT_BRACE -> "RIGHT_BRACE"
  | LEFT_BRACKET -> "LEFT_BRACKET"
  | RIGHT_BRACKET -> "RIGHT_BRACKET"
  | LEFT_ANGULAR -> "LEFT_ANGULAR"
  | RIGHT_ANGULAR -> "RIGHT_ANGULAR"
  | TRUE -> "TRUE"
  | FALSE -> "FALSE"
  | MAYBE -> "MAYBE"
  | PLUS -> "PLUS"
  | MINUS -> "MINUS"
  | ASTERISK -> "ASTERISK"
  | FORWARD_SLASH -> "FORWARD_SLASH"
  | CARET -> "CARET"
  | INCREMENT_OP -> "INCREMENT_OP"
  | COMMENT -> "COMMENT"
  | WHEN -> "WHEN"
  | USE -> "USE"
  | CONST_CONST -> "CONST_CONST"
  | CONST_VAR -> "CONST_VAR"
  | VAR_CONST -> "VAR_CONST"
  | VAR_VAR -> "VAR_VAR"
  | CONST_CONST_CONST -> "CONST_CONST_CONST"
  | ASSIGNMENT -> "ASSIGNMENT"
  | MUCH_LOOSER_CHECK -> "MUCH_LOOSER_CHECK"
  | LOOSE_CHECK -> "LOOSE_CHECK"
  | PRECISE_CHECK -> "PRECISE_CHECK"
  | MORE_PRECISE_CHECK -> "MORE_PRECISE_CHECK"
  | FILE_DELIM -> "FILE_DELIM"
  | FUNCTION -> "FUNCTION"
  | ARROW -> "ARROW"
  | RETURN -> "RETURN"
  | COMMA -> "COMMA"
  | COLON -> "COLON"
  | INT_T -> "INT_T"
  | STRING_T -> "STRING_T"
  | CHAR_T -> "CHAR_T"
  | DIGIT_T -> "DIGIT_T"
  | INT9_T -> "INT9_T"
  | INT99_T -> "INT99_T"
  | REGEXP_T -> "REGEXP_T"
  | PREVIOUS -> "PREVIOUS"
  | NEXT -> "NEXT"
  | CURRENT -> "CURRENT"
  | IMPORT -> "IMPORT"
  | EXPORT -> "EXPORT"
  | TO -> "TO"
  | CLASS -> "CLASS"
  | NEW -> "NEW"
  | DELETE -> "DELETE"
  | ASYNC -> "ASYNC"
  | AWAIT -> "AWAIT"
  | NOOP -> "NOOP"
  | REVERSE -> "REVERSE"
  | IDENTIFIER s -> Printf.sprintf "IDENTIFIER(%s)" s
  | STRING s -> Printf.sprintf "STRING(%s)" s
  | INTEGER n -> Printf.sprintf "INTEGER(%d)" n
  | FLOAT f -> Printf.sprintf "FLOAT(%g)" f
  | INFINITY -> "INFINITY"
  | UNDEFINED -> "UNDEFINED"
  | EOF -> "EOF"

type token = {
    ttype: token_type;
    lexeme: string;
    line: int;
    col: int;
}
