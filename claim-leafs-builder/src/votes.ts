import { gql, request } from "graphql-request";
import { SNAPSHOT_API_ENDPOINT, SNAPSHOT_SPACE, SNAPSHOT_START_TIMESTAMP } from "./snapshot";

const VOTES_FOR_PROPOSALS_QUERY = (proposalIds?: string[]) => gql`
  query {
    votes(first: 1000, where: { proposal_in: [${(proposalIds || [])
    .map((id) => '"' + id + '"')
    .join(",")}]}) {
      proposal {
        id
      }
      voter
      vp
    }
  }
`;

export type Vote = {
  address: `0x${string}`;
  points: number;
};

type VotesByProposalQueryResult = {
  votes: {
    voter: `0x${string}`;
    /** Note this is a float. It can have more than 4 decimals */
    vp: number;
    proposal: {
      id: string;
    };
  }[];
};

export const fetchVotes = async (proposalIds: string[]): Promise<Vote[]> => {
  const result = await request<VotesByProposalQueryResult>(
    SNAPSHOT_API_ENDPOINT,
    VOTES_FOR_PROPOSALS_QUERY(proposalIds),
  );

  let votesByUser: Record<
    `0x${string}`,
    {
      address: `0x${string}`;
      totalPoints: number;
      totalVotesCast: number;
    }
  > = {};

  for (const vote of result.votes) {
    const { voter, vp } = vote;
    if (!votesByUser[voter]) {
      // First vote we've recorded for this user in this group.
      votesByUser[voter] = {
        address: voter,
        totalPoints: vp,
        totalVotesCast: 1,
      };
    } else {
      votesByUser[voter].totalPoints += vp;
      votesByUser[voter].totalVotesCast += 1;
    }
  }

  const qualifyingVoters = Object.values(votesByUser)
    // filter out anyone who didn't vote on all of the proposals.
    .filter(userData => userData.totalVotesCast === proposalIds.length)
    .map(userData => { return { address: userData.address, points: userData.totalPoints / proposalIds.length } });

  return qualifyingVoters;
};