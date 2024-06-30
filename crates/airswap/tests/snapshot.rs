use std::{collections::HashMap, env, fs::File};

use airswap::{
    claim::{GroupedProposal, MerkleTree, ProposalGroup},
    pool::{rootsByTreeReturn, PoolContractInstance},
};
use alloy::providers::ProviderBuilder;
use alloy::{
    hex,
    primitives::{address, B256},
};
use dotenv::dotenv;
use itertools::Itertools;

#[tokio::test]
async fn test_votes_for_proposals() {
    dotenv().ok();

    let eth_rpc = env::var("ETH_RPC_URL").unwrap();

    let group = ProposalGroup::new(vec![
        hex!("6509ffd1d00d4862e94bd250d7dd0abbb77054c5ab28c289f614362bee805866").into(),
        hex!("77de42127551bd8007cf8493b1e584f0775f195a20f37583c5d267da5369aa49").into(),
    ]);

    let pool_instance = PoolContractInstance::new(
        address!("bbcec987E4C189FCbAB0a2534c77b3ba89229F11"),
        ProviderBuilder::new().on_http(eth_rpc.parse().unwrap()),
    );

    let tree = group.hash();

    println!("tree: {}", tree);

    let rootsByTreeReturn { _0: root } = pool_instance.rootsByTree(tree).call().await.unwrap();

    println!("root: {}", root);

    let proposals = serde_json::from_reader::<_, HashMap<B256, GroupedProposal>>(
        File::open("./proposals/proposals.json").unwrap(),
    )
    .unwrap();

    // Monthly Update: 1 Jan 2024
    let proposal = proposals.get(&tree).unwrap();

    let tree = MerkleTree::from_leaves(proposal.votes.iter().map(B256::from).sorted());

    for vote in &proposal.votes {
        let proof = tree.get_proof(&B256::from(vote)).unwrap();

        let result = pool_instance
            .verify(vote.address, root, vote.points, proof)
            .call()
            .await
            .unwrap();

        assert!(result._0);
    }
}
