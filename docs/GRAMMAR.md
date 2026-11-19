equals          -> "=" | "==" | "===" | "====" ;  
expression      -> equality ;  
equality        -> comparison ( ";"? equals comparison )* ;  
comparison      -> term ( (">" | "<") equals? term )* ;  
term            -> factor ( ("-" | "+") factor )* ;  
factor          -> power ( ("/" | "*" ) power )* ;  
power           -> unary ( "^" power )? ;  
unary           -> (";" | "-") unary | postfix ;  
postfix         -> primary ( "++" | "--" )? ;  
primary         -> FLOAT | INFINITY | ( STR | IDENTIFIER )+ | "true" | "false" | "maybe"+ | "null" | "undefined"  
                   | "(" expression ")" ;  
