let Move
    : Type
    = { destination_directory : Text }

let Copy
    : Type
    = { destination_directory : Text }

let FilterAction
    : Type
    = < Move : Move | Copy : Copy >

let Regex
    : Type
    = { regex : Text }

let Mimetype
    : Type
    = { mimetype : Text }

let Filter
    : Type
    = < Regex : Regex | Mimetype : Mimetype >

let action
    : FilterAction
    = FilterAction.Move { destination_directory = "folder" }

in  action
