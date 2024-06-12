use std::convert;

use cynic::{http::ReqwestExt, GraphQlError, QueryBuilder};
use thiserror::Error;

use super::{Vote, SNAPSHOT_URL};

mod types {
    use crate::claim::snapshot::schema;

    use cynic::{QueryFragment, QueryVariables};

    #[derive(QueryVariables, Debug)]
    pub struct VotesForProposalsVariables<'a> {
        pub proposal_in: Option<Vec<Option<&'a str>>>,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Query",
        variables = "VotesForProposalsVariables",
        schema_path = "schema/snapshot.graphql"
    )]
    pub struct VotesForProposals {
        #[arguments(first: 1000, where: { proposal_in: $proposal_in })]
        pub votes: Option<Vec<Option<Vote>>>,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(schema_path = "schema/snapshot.graphql")]
    pub struct Vote {
        pub proposal: Option<Proposal>,
        pub voter: String,
        pub vp: Option<f64>,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(schema_path = "schema/snapshot.graphql")]
    pub struct Proposal {
        pub id: String,
    }
}

pub async fn get_votes_for_proposals(
    proposal_ids: Vec<&str>,
) -> Result<Vec<Vote>, Vec<GraphQlError>> {
    let operation = types::VotesForProposals::build(types::VotesForProposalsVariables {
        proposal_in: Some(proposal_ids.into_iter().map(Option::Some).collect()),
    });

    let response = reqwest::Client::new()
        .post(SNAPSHOT_URL)
        .run_graphql(operation)
        .await
        .unwrap();

    match response.data {
        Some(data) => match data.votes {
            Some(votes) => Ok(votes
                .into_iter()
                .filter_map(convert::identity)
                .filter_map(|v| Vote::try_from(v).ok())
                .collect()),
            None => todo!(),
        },
        None => todo!(),
    }
}

#[derive(Error, Debug)]
pub enum VoteError {
    #[error("No proposal")]
    NoProposal,
}

impl TryFrom<types::Vote> for Vote {
    type Error = VoteError;

    fn try_from(value: types::Vote) -> Result<Self, Self::Error> {
        let proposal = value.proposal.ok_or(VoteError::NoProposal)?;

        let vote = Vote {
            proposal_id: proposal.id,
            voter: value.voter,
            points: value.vp.unwrap_or_default(),
        };

        Ok(vote)
    }
}
