let FilterAction
    : Type
    = < Move : { dest : Text } | Copy : { dest : Text } >

let Filter
    : Type
    = < Regex : Text | Mimetype : Text >

let Predicate
    : Type
    = < Check : Filter | And : List Filter | Or : List Filter >

let Rule
    : Type
    = { name : Text, match : Predicate, action : FilterAction }

in    [ { name = "copy images"
        , match = Predicate.Check (Filter.Regex ".(png|jpg)\$")
        , action = FilterAction.Move { dest = "images" }
        }
      ]
    : List Rule
