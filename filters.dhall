let Types = ./types.dhall

let P = Types.Predicate

let F = Types.Filter

let A = Types.FilterAction

in    [ { name = "copy images"
        , match = P.Check (F.Regex ".(png|jpg)\$")
        , action = A.Move { dest = "images" }
        }
      ]
    : List Types.Rule
