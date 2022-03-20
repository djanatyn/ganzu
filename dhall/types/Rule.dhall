let Predicate = ./Predicate.dhall

let FilterAction = ./FilterAction.dhall

in  { name : Text, match : Predicate, action : FilterAction }
