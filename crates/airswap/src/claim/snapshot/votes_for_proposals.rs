use std::ops::Div;

use alloy::hex::FromHexError;
use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
use cynic::{http::ReqwestExt, GraphQlError, QueryBuilder};
use itertools::Itertools;
use thiserror::Error;

use super::{AggregatedVote, ProposalGroup, Vote, SNAPSHOT_URL};

mod types {
    use crate::claim::snapshot::schema;

    use cynic::{QueryFragment, QueryVariables};

    #[derive(QueryVariables, Debug)]
    pub struct VotesForProposalsVariables {
        pub proposal_in: Option<Vec<Option<String>>>,
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
    proposal_group: ProposalGroup,
) -> Result<Vec<AggregatedVote>, Vec<GraphQlError>> {
    let proposal_count = proposal_group.len() as u8;
    let operation = types::VotesForProposals::build(types::VotesForProposalsVariables {
        proposal_in: Some(
            proposal_group
                .ids
                .into_iter()
                .map(|id| Option::Some(id.to_string()))
                .collect(),
        ),
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
                .flatten()
                .filter_map(|v| Vote::try_from(v).ok())
                .into_grouping_map_by(|v| v.voter.clone())
                .fold(
                    (BigDecimal::zero(), 0_u8),
                    |(acc_points, acc_vote_count), _, v| {
                        (acc_points + v.points, acc_vote_count + 1)
                    },
                )
                .into_iter()
                .filter_map(|(voter, (points, vote_count))| {
                    if vote_count == proposal_count {
                        Some(AggregatedVote::new(voter, points.div(proposal_count)))
                    } else {
                        None
                    }
                })
                .collect()),
            None => Ok(vec![]),
        },
        None => Ok(vec![]),
    }
}

#[derive(Error, Debug)]
pub enum VoteError {
    #[error("No proposal")]
    NoProposal,
    #[error("Invalid address")]
    InvalidAddress(#[from] FromHexError),
    #[error("Cann't parse decimal {0}")]
    InvalidDecimal(#[from] ParseBigDecimalError),
}

impl TryFrom<types::Vote> for Vote {
    type Error = VoteError;

    fn try_from(value: types::Vote) -> Result<Self, Self::Error> {
        let proposal = value.proposal.ok_or(VoteError::NoProposal)?;
        let voter = value.voter.parse()?;
        let points = BigDecimal::try_from(value.vp.unwrap_or_default())?;

        let vote = Vote {
            proposal_id: proposal.id,
            voter,
            points,
        };

        Ok(vote)
    }
}
