use std::{collections::HashMap, convert};

use cynic::{http::ReqwestExt, GraphQlError, QueryBuilder};
use itertools::Itertools;

use super::{Proposal, ProposalInterval, SNAPSHOT_URL};

mod types {
    use crate::claim::snapshot::schema;

    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", schema_path = "schema/snapshot.graphql")]
    pub struct Proposals {
        #[arguments(where: { space: "vote.airswap.eth" })]
        pub proposals: Option<Vec<Option<Proposal>>>,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(schema_path = "schema/snapshot.graphql")]
    pub struct Proposal {
        pub id: String,
        pub title: String,
        pub start: i32,
        pub end: i32,
        pub snapshot: Option<String>,
        pub state: Option<String>,
    }
}

pub async fn get_grouped_proposals(
) -> Result<HashMap<ProposalInterval, Vec<Proposal>>, Vec<GraphQlError>> {
    let operation = types::Proposals::build(());

    let response = reqwest::Client::new()
        .post(SNAPSHOT_URL)
        .run_graphql(operation)
        .await
        .unwrap();

    match response.data {
        Some(data) => match data.proposals {
            Some(proposals) => Ok(proposals
                .into_iter()
                .filter_map(convert::identity)
                .into_group_map_by(|p| ProposalInterval::from(p))
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(Proposal::from).collect()))
                .collect()),
            None => Ok(HashMap::new()),
        },
        None => match response.errors {
            Some(errors) => Err(errors),
            None => Err(vec![]),
        },
    }
}

impl From<&types::Proposal> for ProposalInterval {
    fn from(value: &types::Proposal) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<types::Proposal> for Proposal {
    fn from(value: types::Proposal) -> Self {
        Self {
            id: value.id,
            title: value.title,
            snapshot: value.snapshot,
            state: value.state,
        }
    }
}
