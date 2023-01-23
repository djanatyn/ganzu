//! Parse and generate KDL manifests.
//!
//! These manifests describe:
//! - what files are meant to be transferred,
//! - the aboslute path of each file,
//! - the last modification time of each file, and
//! - what action to perform on each file.
//!
//! This means that we can "execute" a manifest after validating it.
//!
//! To validate a manifest, we need to check:
//! - that all paths are still valid,
//! - that no files have been modified since the manifest was created.

const EXAMPLE_META: &'static str = r#"
meta version="1.0" created="1669472812"
"#;

const EXAMPLE_MANIFEST: &'static str = r#"
meta version="1.0" created="1669472812"

matches {
  files {
    snapshot \
        input_name="query.graphql" \
        absolute_path="/home/djanatyn/query.graphql" \
        last_mtime="1643587565"
    action {
        move dest="/home/djanatyn/documents"
    }
  }
}
"#;

#[derive(knuffel::Decode, Debug)]
pub struct Meta {
    #[knuffel(property)]
    version: String,

    #[knuffel(property)]
    created: String,
}

#[derive(knuffel::Decode, Debug)]
enum TopLevelNode {
    Meta(Meta),
}

#[derive(knuffel::Decode, Debug)]
pub struct Doc {
    #[knuffel(child)]
    meta: Meta,

    #[knuffel(child)]
    matches: Matches,
}

#[derive(knuffel::Decode, Debug)]
pub struct Matches {}

/// WIP: experimenting with kdl
pub fn kdl_exploration() {
    let mut doc = kdl::KdlDocument::new();
    let mut meta = kdl::KdlNode::new("meta");
    meta.insert("version", "1.0");

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("failed")
        .as_secs();
    meta.insert("created", now.to_string());
    doc.nodes_mut().push(meta);

    println!("{}", doc.to_string());

    dbg!(knuffel::parse::<Doc>("example.kdl", &doc.to_string()));
}

#[cfg(test)]
mod tests {
    use super::{TopLevelNode, EXAMPLE_MANIFEST, EXAMPLE_META};

    #[test]
    fn parse_example_manifest_kdl() {
        dbg!(EXAMPLE_MANIFEST.parse::<kdl::KdlDocument>()).unwrap();
    }

    #[test]
    fn parse_example_meta() {
        dbg!(knuffel::parse::<Vec<TopLevelNode>>(
            "meta.kdl",
            EXAMPLE_META
        ))
        .unwrap();
    }
}
