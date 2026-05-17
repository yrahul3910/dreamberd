let multi_char_tokens =
  [
    ("====", Tokens.MORE_PRECISE_CHECK);
    ("===", Tokens.PRECISE_CHECK);
    ("==", Tokens.LOOSE_CHECK);
    (* TODO: need to handle `Tokens.MUCH_LOOSER_CHECK`*)
    ("=", Tokens.ASSIGNMENT);
    ("//", Tokens.COMMENT);
    ("/", Tokens.FORWARD_SLASH);
  ]

(* map from character to token for single-character tokens *)
let char_token_map = Hashtbl.create 50

let () =
  [
    (' ', Tokens.SPACE 1);
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
    ('\'', Tokens.QUOTE "'");
    ('"', Tokens.QUOTE "\"");
    ('+', Tokens.PLUS);
    ('-', Tokens.MINUS);
    ('*', Tokens.ASTERISK);
    ('^', Tokens.CARET);
    (',', Tokens.COMMA);
    (':', Tokens.COLON);
    ('\r', Tokens.NEWLINE);
    ('\n', Tokens.NEWLINE);
  ]
  |> List.map (fun el -> (Uchar.of_char (fst el), snd el))
  |> List.iter (fun el -> Hashtbl.add char_token_map (fst el) (snd el))

(** Attempt to match a substring at a given pos with one of the *
    `multi_char_tokens` above, in order. *)
let try_match s pos =
  List.find_opt
    (fun (str, _) ->
      let len = String.length str in
      pos + len <= String.length s && String.sub s pos len = str)
    multi_char_tokens

(** Attempt to parse a digit in `s` at `pos`. Returns the parsed float and the
    pos to continue at. *)
let parse_digit s pos =
  let get_end ss p =
    String.sub ss p (String.length ss - p)
    |> String.to_seq
    |> Seq.take_while (fun c -> (c >= '0' && c <= '9') || c == '.' || c == '-')
    |> Seq.length
  in
  let j = get_end s pos in
  let substr = String.sub s pos j in
  (Float.of_string substr, pos + j)

(** Attempt to match any sequence of tokens in `s` starting at `pos`, under the
    assumption that we have already tested the above two. *)
let parse_token s pos =
  match String.get s pos with
  | '0' .. '9' ->
      let value, next = parse_digit s pos in
      Ok ([ Tokens.FLOAT value ], next)
  | _ -> Error (pos + 1)

(** Main token-scanning loop. * * Takes a `source` string, a `pos`ition, the
    `acc`umulated tokens so far, and `errs` so far. *)
let rec scan_tokens source pos acc errs =
  if pos >= String.length source then List.rev (Tokens.EOF :: acc)
  else
    (* first attempt multi-char token matching *)
    match try_match source pos with
    (* comments need to be ignored till \n *)
    | Some ("//", _) -> (
        match String.index_from_opt source pos '\n' with
        | Some newPos ->
            scan_tokens source (newPos + 1) (Tokens.NEWLINE :: acc) errs
        | None -> List.rev (Tokens.EOF :: acc))
    | Some (str, tok) ->
        scan_tokens source (pos + String.length str) (tok :: acc) errs
    | None -> (
        (* match single-character tokens *)
        match Hashtbl.find_opt char_token_map (Uchar.of_char source.[pos]) with
        | Some tok -> scan_tokens source (pos + 1) (tok :: acc) errs
        (* match numbers, reserved keywords, and identifiers *)
        | None -> (
            match parse_token source pos with
            (* Need to insert in reversed order since we use `List.rev` at the end *)
            | Ok (toks, newPos) ->
                scan_tokens source newPos (List.rev toks @ acc) errs
            | Error newPos -> scan_tokens source newPos acc (pos :: errs)))
