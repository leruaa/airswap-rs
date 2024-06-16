import { gql, request } from "graphql-request";
import { SNAPSHOT_API_ENDPOINT, SNAPSHOT_SPACE, SNAPSHOT_START_TIMESTAMP } from "./snapshot";
import { encodePacked, keccak256 } from "viem";

export type Proposal = {
  id: string;
  title: string;
  /** This is a **unix timestamp** (seconds, not ms).  */
  start: number;
  /** This is a **unix timestamp** (seconds, not ms).  */
  end: number;
  /** Block number as a string */
  snapshot: string;
  state: "closed" | "open" | "pending";
};

type ProposalsQueryResult = {
  proposals: Proposal[];
};

const PROPOSALS_QUERY = (snapshotSpace: string, startTimestamp: number) => gql`
  query {
    proposals(
      first: 100
      skip: 0
      where: { space_in: ["${snapshotSpace}"], created_gte: ${startTimestamp} }
      orderBy: "created"
      orderDirection: desc
    ) {
      id
      title
      start
      end
      snapshot
      state
    }
  }
`;

export const fetchProposals = async (): Promise<Proposal[][]> => {
  const result = await request<ProposalsQueryResult>(
    SNAPSHOT_API_ENDPOINT,
    PROPOSALS_QUERY(SNAPSHOT_SPACE, SNAPSHOT_START_TIMESTAMP),
  );
  const proposalGroups: Proposal[][] = [];

  // group all proposals that have the same start and end
  result.proposals.forEach((proposal) => {
    const group = proposalGroups.find(
      (group) =>
        group[0].start === proposal.start && group[0].end === proposal.end,
    );
    if (group) {
      group.push(proposal);
    } else {
      proposalGroups.push([proposal]);
    }
  });

  return proposalGroups;
};

export const getGroupHash = (ids: string[]) =>
  keccak256(
    encodePacked(
      new Array(ids.length).fill("bytes32"),
      ids
        .sort()
        .map((id) => (id.length < 66 ? "0x" + id.padStart(64, "0") : id)),
    ),
  );
