import { MerkleTree } from "merkletreejs";
import BigNumber from "bignumber.js";
import { encodePacked, keccak256 } from "viem";
import { Vote } from "./votes";

export const buildMerkleLeaf = (vote: Vote) => {
  let points = BigInt(vote.points);

  return keccak256(
    encodePacked(
      ["address", "uint256"],
      [
        vote.address,
        points,
      ],
    ),
  );
}

export const buildMerkleTree = (leaves: `0x${string}`[]): MerkleTree => {
  return new MerkleTree(leaves, keccak256, { sort: true });
}