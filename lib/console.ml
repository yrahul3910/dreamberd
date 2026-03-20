let error line col msg =
    Printf.printf "%s\n" line;
    Printf.printf "%*s" (col + 2) "^\n";
    Printf.printf "%*s" (col + 1) "|";
    Printf.printf "%s  " (String.make (String.length(line) - col + 5) '_');
    Printf.printf "%s %s\n" "\x1b[33;1merror:\x1b[0m" msg;

