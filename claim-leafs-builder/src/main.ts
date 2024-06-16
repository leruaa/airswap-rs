import { buildMerkleLeaf, buildMerkleTree } from "./merkle";
import { fetchProposals, getGroupHash } from "./proposals";
import { fetchVotes } from "./votes";

console.log("Hello");

let proposals = await fetchProposals();

for (let group of proposals) {
  let ids = group.map(g => g.id);
  let votes = await fetchVotes(ids);
  let tree = getGroupHash(ids);
  let leaves = votes.map(buildMerkleLeaf);
  let merkleTree = buildMerkleTree(leaves);

  console.log(tree, merkleTree.getHexRoot());
}