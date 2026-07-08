expression      -> literal | unary | binary ;
literal         -> FLOAT | INFINITY | STR | "true" | "false" | "maybe" | "null" | "undefined" ;
unary           -> (";" | "-") expression | expression ("++" | "--") ;
binary          -> expression bin_operator expression ;
bin_operator    -> "+" | "-" | "*" | "/" | "^"
                    | ("<" | ">" | ";")? ("=" | "==" | "===" | "====")? ;
