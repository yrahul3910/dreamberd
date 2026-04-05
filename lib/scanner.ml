(*
Source - https://stackoverflow.com/a/79563247
Posted by Maëlan
Retrieved 2026-04-05, License - CC BY-SA 4.0
*)
let uchar_seq_of_utf8 (s : string) : Uchar.t Seq.t =
  let n = String.length s in
  let rec aux i () =
    if i >= n then Seq.Nil
    else
      let d = String.get_utf_8_uchar s i in
      let k = Uchar.utf_decode_length d in
      let u = Uchar.utf_decode_uchar d in
      (* ^ if d is an invalid utf8 sequence, then k = 1
       *   and u = the replacement character (U+FFFD) *)
      Seq.Cons (u, aux (i + k))
  in
  aux 0

let char_token_map = Hashtbl.create 100;;

[ ('(', Tokens.LEFT_PAREN) ]
|> List.map (fun el -> (Uchar.of_char (fst el), snd el))
|> List.iter (fun el -> Hashtbl.add char_token_map (fst el) (snd el))
;;

let rec scan_tokens source =
  match source with
  | Seq.Nil -> Seq.return Tokens.EOF
  | Seq.Cons (x, rest) -> (
      match Hashtbl.find_opt char_token_map x with
      | None -> Seq.empty (* TODO: fail? *)
      | Some tok -> Seq.append (Seq.return tok) (scan_tokens (rest ())))
