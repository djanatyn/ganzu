const EXAMPLE_MANIFEST: &'static str = r#"
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
"#;

#[derive(knuffel::Decode, Debug)]
pub struct Meta {
    #[knuffel(property)]
    version: String,

    #[knuffel(property)]
    created: String,
}

#[derive(knuffel::Decode, Debug)]
pub struct Doc {
    #[knuffel(child)]
    meta: Meta,
}

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
