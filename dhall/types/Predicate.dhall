let Filter = ./Filter.dhall

in  < Check : Filter | And : List Filter | Or : List Filter >
