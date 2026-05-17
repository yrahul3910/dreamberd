open Cmdliner

let file =
  let doc = "The DreamBerd source file to execute." in
  Arg.(value & pos 0 (some string) None & info [] ~docv:"FILE" ~doc)

let verbose =
  let doc = "Print internal interpreter state." in
  Arg.(value & flag & info [ "v"; "verbose" ] ~doc)

let tokens =
  let doc = "Print the token stream produced by the scanner." in
  Arg.(value & flag & info [ "tokens" ] ~doc)

let run file verbose tokens =
  match file with
  | None -> print_endline "No file provided!"
  | Some f ->
      Printf.printf "Interpreting %s (verbose: %b)\n" f verbose;
      let source = Interpreter.Files.read_file f in
      Printf.printf "Contents: %s" source;
      if tokens then begin
        let toks = Interpreter.Scanner.scan_tokens source 0 [] [] in
        List.iter
          (fun t -> print_endline (Interpreter.Tokens.token_type_to_string t))
          toks
      end;
      Interpreter.Console.error "foo bar baz" 4 "message"

let cmd =
  let doc = "A DreamBerd interpreter" in
  let info = Cmd.info "dreamberd" ~doc in
  Cmd.v info Term.(const run $ file $ verbose $ tokens)

let () = exit (Cmd.eval cmd)
