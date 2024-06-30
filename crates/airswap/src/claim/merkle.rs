use std::fmt::Debug;

use alloy::primitives::{keccak256, B256};
use thiserror::Error;

#[derive(Clone)]
pub struct MerkleTree {
    layers: Vec<Vec<B256>>,
}

impl MerkleTree {
    pub fn from_leaves<I>(leaves: I) -> Self
    where
        I: Iterator<Item = B256>,
    {
        let mut layers = vec![leaves.collect::<Vec<_>>()];

        while layers.last().map_or(0, |n| n.len()) > 1 {
            let nodes = layers.last().unwrap();
            let mut current_layer = vec![];

            for chunk in nodes.chunks(2) {
                if chunk.len() == 2 {
                    current_layer.push(hash_pair(chunk.first().unwrap(), chunk.last().unwrap()));
                } else {
                    current_layer.push(*chunk.first().unwrap());
                }
            }

            layers.push(current_layer)
        }

        Self { layers }
    }

    pub fn get_proof(&self, leaf: &B256) -> Result<Vec<B256>, MerkleError> {
        let mut index = self
            .layers
            .first()
            .and_then(|leaves| leaves.iter().position(|l| l == leaf))
            .ok_or(MerkleError::LeafNotFound)?;
        let mut proofs = vec![];

        for layer in self.layers.iter() {
            let sibling_index = sibling_index(index);
            if is_tree_node(layer, sibling_index) {
                proofs.push(layer[sibling_index]);
            }

            index /= 2;
        }

        Ok(proofs)
    }
}

impl Debug for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MerkleTree")
            .field("data", &Node::new(&self.layers, self.layers.len() - 1, 0))
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum MerkleError {
    #[error("The leaf don't exists in the tree")]
    LeafNotFound,
}

struct Node<'a> {
    layers: &'a Vec<Vec<B256>>,
    height: usize,
    index: usize,
}

impl<'a> Node<'a> {
    pub fn new(layers: &'a Vec<Vec<B256>>, height: usize, index: usize) -> Self {
        Self {
            layers,
            height,
            index,
        }
    }
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.height == 0 {
            f.write_str(&self.layers[self.height][self.index].to_string())
        } else {
            let left_child_index = left_child_index(self.index);
            let right_child_index = right_child_index(self.index);
            let is_left_child = is_tree_node(&self.layers[self.height - 1], left_child_index);
            let is_right_child = is_tree_node(&self.layers[self.height - 1], right_child_index);

            if is_left_child || is_right_child {
                let mut tuple_formatter =
                    f.debug_tuple(&self.layers[self.height][self.index].to_string());

                if is_left_child {
                    tuple_formatter.field(&Node::new(
                        self.layers,
                        self.height - 1,
                        left_child_index,
                    ));
                }

                if is_right_child {
                    tuple_formatter.field(&Node::new(
                        self.layers,
                        self.height - 1,
                        right_child_index,
                    ));
                }

                tuple_formatter.finish()
            } else {
                f.write_str(&self.layers[self.height][self.index].to_string())
            }
        }
    }
}

pub fn is_tree_node(layer: &[B256], i: usize) -> bool {
    i < layer.len()
}

fn hash_pair(a: &B256, b: &B256) -> B256 {
    let mut s = [a.0, b.0];
    s.sort();
    let bytes = s.concat();
    keccak256(bytes)
}

fn left_child_index(i: usize) -> usize {
    2 * i
}

fn right_child_index(i: usize) -> usize {
    2 * i + 1
}

fn sibling_index(i: usize) -> usize {
    if i % 2 == 0 {
        // left node
        i + 1
    } else {
        i - 1
    }
}
