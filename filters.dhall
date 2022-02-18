let FilterAction
    : Type
    = < Move : { destination_directory : Text }
      | Copy : { destination_directory : Text }
      >

let Filter
    : Type
    = < Regex : { regex : Text } | Mimetype : { mimetype : Text } >

let action
    : FilterAction
    = FilterAction.Move { destination_directory = "folder" }

in  action
