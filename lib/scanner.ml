let multi_char_tokens = [
  ("====", Tokens.MORE_PRECISE_CHECK);
  ("===",  Tokens.PRECISE_CHECK);
  ("==",   Tokens.LOOSE_CHECK);
  (* TODO: need to handle `Tokens.MUCH_LOOSER_CHECK`*)
  ("=",    Tokens.ASSIGNMENT);
  (* etc. *)
]

(* map from character to token for single-character tokens *)
let char_token_map = Hashtbl.create 50;;

let () = [
    (* The ! one is wrong, I'll fix it later *)
    ('!', Tokens.BANG 1);
    (';', Tokens.SEMICOLON);
    ('?', Tokens.QUESTION_MARK);
    ('(', Tokens.LEFT_PAREN);
    (')', Tokens.RIGHT_PAREN);
    ('{', Tokens.LEFT_BRACE);
    ('}', Tokens.RIGHT_BRACE);
    ('[', Tokens.LEFT_BRACKET);
    (']', Tokens.RIGHT_BRACKET);
    ('<', Tokens.LEFT_ANGULAR);
    ('>', Tokens.RIGHT_ANGULAR);
    ('+', Tokens.PLUS);
    ('-', Tokens.MINUS);
    ('*', Tokens.ASTERISK);
    ('/', Tokens.FORWARD_SLASH);
    ('^', Tokens.CARET);
    (',', Tokens.COMMA);
    (':', Tokens.COLON);
]
|> List.map (fun el -> (Uchar.of_char (fst el), snd el))
|> List.iter (fun el -> Hashtbl.add char_token_map (fst el) (snd el))


(** Attempt to match a substring at a given pos with one of the
* `multi_char_tokens` above, in order.
*)
let try_match s pos =
  List.find_opt
    (fun (str, _) ->
      let len = String.length str in
      pos + len <= String.length s &&
      String.sub s pos len = str)
    multi_char_tokens


(** Main token-scanning loop.
*
* Takes a `source` string, a `pos`ition, the `acc`umulated tokens so far, and `errs` so far.
*)
let rec scan_tokens source pos acc errs =
  if pos >= String.length source then List.rev (Tokens.EOF :: acc)
  else
    match try_match source pos with
    | Some (str, tok) -> scan_tokens source (pos + String.length str) (tok :: acc) errs
    | None ->
      match Hashtbl.find_opt char_token_map (Uchar.of_char source.[pos]) with
      | Some tok -> scan_tokens source (pos + 1) (tok :: acc) errs
      | None -> scan_tokens source (pos + 1) acc (pos :: errs)
