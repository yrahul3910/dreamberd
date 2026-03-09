(lang dune 3.21)

(name interpreter)

(generate_opam_files true)

(source
 (github yrahul3910/dreamberd))

(authors "Rahul Yedida <rahul@ryedida.me>")

(maintainers "Rahul Yedida <rahul@ryedida.me>")

(license MIT)

(documentation https://github.com/yrahul3910/dreamberd)

(package
 (name interpreter)
 (synopsis "A short synopsis")
 (description "A longer description")
 (depends ocaml cmdliner)
 (tags
  ("add topics" "to describe" your project)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/reference/dune-project/index.html
