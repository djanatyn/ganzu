# ganzu

[`ganzu`](https://vlasisku.lojban.org/ganzu) is a rust library to organize your files. it is inspired by [organize](https://organize.readthedocs.io/en/latest/), the file management automation tool.

>  ganzu -gaz- gismu
> x1 organizes x2 [relative chaos] into x3 [ordered/organized result] by system/principle(s) x4.

with ganzu, instead of placing your rules in a configuration file which is evaluated at runtime, you build a new rust program!

## define filters and actions in rust

ganzu is a library. you compile your *own* binary to organize files, based on rules you define:

```rust
Rule {
    filters: vec![
        Filter::Mimetype("text/html"),
        Filter::Mimetype("text/plain"),
    ],
    action: FilterAction::Move {
        dest: dest::DOCUMENTS,
    },
}
```

## compile into a rust program, run against stdin

ganzu is flexible - to organize files, just pipe their paths:

``` sh
find ~/Downloads -type f | ganzu plan
```

## get feedback on matched files

you can see which filters matched particular files, and which files did not match any filters

``` rust
[src/main.rs:284] (filter, file) = (
    Mimetype(
        "text/plain",
    ),
    FileSnapshot {
        input_name: "/home/djanatyn/query.graphql",
        absolute_path: "/home/djanatyn/query.graphql",
        last_mtime: 1643587565,
        mimetype: "text/plain",
    },
)
```

## generate a plan before modifying files

instead of modifying files directly, we generate a plan (like terraform). it is in [kdl](https://kdl.dev), you can inspect and modify it yourself:

```kdl
meta version="1.0" created="1669472812"

matches {
  file {
    snapshot \
        input_name="query.graphql" \
        absolute_path="/home/djanatyn/query.graphql" \
        last_mtime="1643587565"
    action {
        move dest="/home/djanatyn/documents"
    }
  }
}
```
