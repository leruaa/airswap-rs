#[derive(Debug)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub snapshot: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ProposalInterval {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub points: f64,
}
