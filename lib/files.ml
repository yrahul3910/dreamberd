let read_file f = 
    let lines = [] in
    let ic = open_in f in
    try
        let rec read_lines lines ic = 
            let updated_lines = input_line ic :: lines in
                try
                    read_lines updated_lines ic 
                with End_of_file -> 
                    close_in_noerr ic;
                    lines |> List.rev |> String.concat "\n" in
        read_lines lines ic
    with 
    e ->
        close_in_noerr ic;
        raise e
