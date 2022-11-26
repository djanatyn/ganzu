use crate::config::Config;
use crate::filter::{Filter, FilterAction};
use crate::rule::Rule;

pub mod dest {
    pub const IMAGES: &str = "/home/djanatyn/images";
    pub const DOCUMENTS: &str = "/home/djanatyn/documents";
    pub const ARCHIVES: &str = "/home/djanatyn/archives";
}

pub fn config() -> Config {
    Config {
        rules: vec![
            Rule {
                filters: vec![
                    Filter::Mimetype("image/png"),
                    Filter::Mimetype("image/jpeg"),
                    Filter::Mimetype("image/gif"),
                    Filter::Mimetype("image/bmp"),
                    Filter::Mimetype("image/svg+xml"),
                ],
                action: FilterAction::Move { dest: dest::IMAGES },
            },
            Rule {
                filters: vec![
                    Filter::Mimetype("text/html"),
                    Filter::Mimetype("text/plain"),
                ],
                action: FilterAction::Move {
                    dest: dest::DOCUMENTS,
                },
            },
            Rule {
                filters: vec![Filter::Mimetype("application/zip")],
                action: FilterAction::Move {
                    dest: dest::ARCHIVES,
                },
            },
        ],
    }
}
