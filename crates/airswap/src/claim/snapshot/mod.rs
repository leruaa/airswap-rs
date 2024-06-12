mod proposals;
pub use proposals::get_grouped_proposals;

mod votes_for_proposals;
pub use votes_for_proposals::get_votes_for_proposals;

mod types;
pub use types::*;

mod schema {
    cynic::use_schema!("schema/snapshot.graphql");
}

const SNAPSHOT_URL: &str = "https://hub.snapshot.org/graphql";
