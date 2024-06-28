use displaydoc::Display;

/// Certificate application lifecycle: {state}
#[derive(Debug, Display)]
pub struct CertFSM<State> {
    pub state: State,
}

/// <Draft>
#[derive(Debug, Display)]
pub struct Draft;
impl CertFSM<Draft> {
    pub fn new() -> Self {
        CertFSM { state: Draft }
    }
    pub fn submit(self) -> CertFSM<Requested> {
        CertFSM { state: Requested }
    }
    pub fn update(self) -> CertFSM<Draft> {
        CertFSM { state: Draft }
    }
}

/// <Requested>
#[derive(Debug, Display)]
pub struct Requested;
impl CertFSM<Requested> {
    pub fn issue(self) -> CertFSM<Issued> {
        CertFSM { state: Issued }
    }
    pub fn decline(self) -> CertFSM<Declined> {
        CertFSM { state: Declined }
    }
    pub fn feedback(self) -> CertFSM<Draft> {
        CertFSM { state: Draft }
    }
}

/// <Issued>
#[derive(Debug, Display)]
pub struct Issued;
impl CertFSM<Issued> {
    pub fn revocate(self, date: String) -> CertFSM<Invalid> {
        CertFSM { state: Invalid }
    }
    pub fn expire(self) -> CertFSM<Expired> {
        CertFSM { state: Expired }
    }
}

/// <Invalid>
#[derive(Debug, Display)]
pub struct Invalid;

/// <Expired>
#[derive(Debug, Display)]
pub struct Expired;

/// <Declined>
#[derive(Debug, Display)]
pub struct Declined;
