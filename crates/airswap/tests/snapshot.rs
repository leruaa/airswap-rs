use airswap::claim;

#[tokio::test]
async fn test_grouped_proposals() {
    let proposals = claim::get_grouped_proposals().await.unwrap();

    assert!(!proposals.is_empty());
}

#[tokio::test]
async fn test_votes_for_proposals() {
    let votes = claim::get_votes_for_proposals(vec![
        "0x16348b60360fd13e7e9a44ffec492a15feb9100f6224c051d0cf4bd8197fc0c5",
        "0xe1ed5c1f03f27dd98a6db947491476ee9bdd5a42fa69e3b4dd0945f301a25467",
        "0x41ad55c33632abb0f419de80f62b583c72ce6c74b674d9b77e5ce5d958f6dbcb",
    ])
    .await
    .unwrap();

    assert!(!votes.is_empty());
}
