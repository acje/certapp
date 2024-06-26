use displaydoc::Display;

/// Certificate application lifecycle: {state}
#[derive(Debug, Display)]
pub struct CertApp<State> {
    pub state: State,
}

/// <Draft>, Draft saved date: {draft_saved_date}, Comment: {comment}
#[derive(Debug, Display)]
pub struct Draft {
    draft_saved_date: String,
    comment: String,
}
impl CertApp<Draft> {
    pub fn new(date: String) -> Self {
        CertApp {
            state: Draft {
                draft_saved_date: date,
                comment: String::new(),
            },
        }
    }
    pub fn submit(self, date: String) -> CertApp<Requested> {
        CertApp {
            state: Requested {
                draft_saved_date: self.state.draft_saved_date,
                requested_date: date,
            },
        }
    }
    pub fn update(self, date: String) -> CertApp<Draft> {
        CertApp {
            state: Draft {
                draft_saved_date: date,
                comment: self.state.comment,
            },
        }
    }
}

/// <Requested> Draft saved date: {draft_saved_date}, Requested date: {requested_date}
#[derive(Debug, Display)]
pub struct Requested {
    draft_saved_date: String,
    requested_date: String,
}
impl CertApp<Requested> {
    pub fn issue(self, date: String, expiration_date: String) -> CertApp<Issued> {
        CertApp {
            state: Issued {
                draft_saved_date: self.state.draft_saved_date,
                requested_date: self.state.requested_date,
                issued_date: date,
                expiration_date,
            },
        }
    }
    pub fn decline(self, reason: String, date: String) -> CertApp<Declined> {
        CertApp {
            state: Declined {
                draft_saved_date: self.state.draft_saved_date,
                requested_date: self.state.requested_date,
                declined_date: date,
                reason,
            },
        }
    }
    pub fn feedback(self, comment: String) -> CertApp<Draft> {
        CertApp {
            state: Draft {
                draft_saved_date: self.state.draft_saved_date,
                comment,
            },
        }
    }
}

/// <Issued> Draft saved date: {draft_saved_date}, Requested date: {requested_date}, , Issued date: {issued_date}, Expiration date: {expiration_date}
#[derive(Debug, Display)]
pub struct Issued {
    draft_saved_date: String,
    requested_date: String,
    issued_date: String,
    expiration_date: String,
}

impl CertApp<Issued> {
    pub fn revocate(self, date: String) -> CertApp<Invalid> {
        CertApp {
            state: Invalid {
                draft_saved_date: self.state.draft_saved_date,
                requested_date: self.state.requested_date,
                issued_date: self.state.issued_date,
                expiration_date: self.state.expiration_date,
                revocation_date: date,
            },
        }
    }
    pub fn expire(self) -> CertApp<Expired> {
        CertApp {
            state: Expired {
                draft_saved_date: self.state.draft_saved_date,
                requested_date: self.state.requested_date,
                issued_date: self.state.issued_date,
                expiration_date: self.state.expiration_date,
            },
        }
    }
}

/// <Invalid> Draft saved date: {draft_saved_date}, Requested date: {requested_date}, Issued date: {issued_date}, Expiration date: {expiration_date}, Revocation date: {revocation_date}
#[derive(Debug, Display)]
pub struct Invalid {
    draft_saved_date: String,
    requested_date: String,
    issued_date: String,
    expiration_date: String,
    revocation_date: String,
}

/// <Expired> Draft saved date: {draft_saved_date}, Requested date: {requested_date}, Issued date: {issued_date}, Expiration date: {expiration_date}
#[derive(Debug, Display)]
struct Expired {
    draft_saved_date: String,
    requested_date: String,
    issued_date: String,
    expiration_date: String,
}

/// <Declined> Draft saved date: {draft_saved_date}, Requested date: {requested_date}, Declined date: {declined_date}, Reason: {reason}
#[derive(Debug, Display)]
pub struct Declined {
    draft_saved_date: String,
    requested_date: String,
    declined_date: String,
    reason: String,
}
