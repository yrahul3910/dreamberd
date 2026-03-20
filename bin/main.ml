open Cmdliner

let file =
  let doc = "The DreamBerd source file to execute." in
  Arg.(value & pos 0 (some string) None & info [] ~docv:"FILE" ~doc)

let verbose =
  let doc = "Print internal interpreter state." in
  Arg.(value & flag & info ["v"; "verbose"] ~doc)

let run file verbose =
  match file with
  | None -> print_endline "No file provided!"
  | Some f -> 
        Printf.printf "Interpreting %s (verbose: %b)\n" f verbose;
        Printf.printf "Contents: %s" (Interpreter.Files.read_file f);
        Interpreter.Console.error "foo bar baz" 4 "message"

let cmd =
  let doc = "A DreamBerd interpreter" in
  let info = Cmd.info "dreamberd" ~doc in
  Cmd.v info Term.(const run $ file $ verbose)

let () = exit (Cmd.eval cmd)
