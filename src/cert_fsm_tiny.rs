use displaydoc::Display;

/// Application title: {title}, {content}, Comment: {comment}
#[derive(Debug, Display)]
pub struct Application {
    title: String,
    content: String,
    comment: String,
}

/// Certificate title: {title}, {content}, Expires: {expires}
#[derive(Debug, Display)]
pub struct Certificate {
    title: String,
    content: String,
    expires: u64,
}

#[derive(Debug, Display)]
pub enum Document {
    /// Doctype Application: {0}
    A(Application),
    /// Doctype Certificate: {0}
    C(Certificate),
}

/// Certificate application lifecycle: {state}, {document}
#[derive(Debug, Display)]
pub struct CertFSM<State> {
    state: State,
    document: Document,
}

/// <Draft>
#[derive(Debug, Display)]
pub struct Draft;

/// <Requested>
#[derive(Debug, Display)]
pub struct Requested;

/// <Issued>
#[derive(Debug, Display)]
pub struct Issued;

/// <Invalid>
#[derive(Debug, Display)]
pub struct Invalid;

/// <Expired>
#[derive(Debug, Display)]
pub struct Expired;

/// <Declined>
#[derive(Debug, Display)]
pub struct Declined;

impl CertFSM<Draft> {
    pub fn new(title: String, content: String) -> Self {
        CertFSM {
            state: Draft,
            document: Document::A(Application {
                title,
                content,
                comment: String::new(),
            }),
        }
    }
    pub fn submit(self) -> CertFSM<Requested> {
        CertFSM {
            state: Requested,
            document: self.document,
        }
    }
    pub fn update(self, content: String) -> CertFSM<Draft> {
        CertFSM {
            state: Draft,
            document: match self.document {
                Document::A(application) => Document::A(Application {
                    content,
                    ..application
                }),
                Document::C(certificate) => Document::C(certificate),
            },
        }
    }
}

impl CertFSM<Requested> {
    pub fn issue(self) -> CertFSM<Issued> {
        CertFSM {
            state: Issued,
            document: self.document,
        }
    }
    pub fn decline(self) -> CertFSM<Declined> {
        CertFSM {
            state: Declined,
            document: self.document,
        }
    }
    pub fn feedback(self) -> CertFSM<Draft> {
        CertFSM {
            state: Draft,
            document: self.document,
        }
    }
}

impl CertFSM<Issued> {
    pub fn revocate(self) -> CertFSM<Invalid> {
        CertFSM {
            state: Invalid,
            document: self.document,
        }
    }
    pub fn expire(self) -> CertFSM<Expired> {
        CertFSM {
            state: Expired,
            document: self.document,
        }
    }
}
