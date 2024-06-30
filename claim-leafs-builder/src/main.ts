import { buildMerkleLeaf, buildMerkleTree } from "./merkle";
import { fetchProposals, getGroupHash } from "./proposals";
import { fetchVotes } from "./votes";
import * as fs from "fs";

let proposals = await fetchProposals();

let output: any = {};

for (let group of proposals) {
  let ids = group.map(g => g.id);
  let votes = await fetchVotes(ids);
  let tree = getGroupHash(ids);
  let leaves = votes.map(buildMerkleLeaf);
  let merkleTree = buildMerkleTree(leaves);

  output[tree] = {
    "root": merkleTree.getHexRoot(),
    "votes": votes
  };
}

fs.writeFileSync('../crates/airswap/proposals/proposals.json', JSON.stringify(output, null, 2));