use std::collections::HashMap;

use alloy::hex::FromHexError;
use cynic::{http::ReqwestExt, GraphQlError, QueryBuilder};
use itertools::Itertools;
use thiserror::Error;

use super::{ProposalGroup, ProposalInterval, SNAPSHOT_URL};

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
) -> Result<HashMap<ProposalInterval, ProposalGroup>, Vec<GraphQlError>> {
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
                .flatten()
                .into_group_map_by(|p| ProposalInterval::from(p))
                .into_iter()
                .filter_map(|(k, v)| ProposalGroup::try_from(v).ok().map(|g| (k, g)))
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

#[derive(Error, Debug)]
pub enum ProposalError {
    #[error("Invalid id")]
    InvalidId(#[from] FromHexError),
}

impl TryFrom<Vec<types::Proposal>> for ProposalGroup {
    type Error = ProposalError;

    fn try_from(value: Vec<types::Proposal>) -> Result<Self, Self::Error> {
        value
            .into_iter()
            .try_fold(ProposalGroup::default(), |mut acc, p| match p.id.parse() {
                Ok(p) => {
                    acc.ids.push(p);
                    acc.ids.sort();
                    Ok(acc)
                }
                Err(err) => Err(ProposalError::InvalidId(err)),
            })
    }
}
