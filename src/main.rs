#![warn(clippy::pedantic)]
use cert_fsm::CertApp;
use cert_fsm_tiny::CertFSM;
mod cert_fsm;
mod cert_fsm_tiny;

fn main() {
    let doc = CertApp::new("2024-01-01".to_string());
    println!("Doc1: {doc}");
    let doc = doc.submit("2024-01-02".to_string());
    println!("Doc1: {doc}");
    let doc = doc.decline("Det er et nei. 🤦".to_string(), "2024-01-03".to_string());
    println!("Doc1: {doc}");

    //This will not compile because the document is in a declined state at this point
    //let doc = doc.submit("2024-01-04".to_string());

    // Chained state transitions
    let doc2 = CertApp::new("2024-01-01".to_string())
        .submit("2024-01-02".to_string())
        .issue("2024-01-03".to_string(), "2024-01-04".to_string());

    println!("Doc2: {doc2}");

    // chained example 2
    let doc3 = CertApp::new("2024-01-01".to_string())
        .submit("2024-01-02".to_string())
        .feedback("I need more information".to_string())
        .update("2024-01-03".to_string())
        .submit("2024-01-04".to_string())
        .issue("2024-01-05".to_string(), "2024-01-06".to_string())
        .revocate("2024-01-07".to_string());
    println!("Doc3: {doc3}");

    let doc4 = CertFSM::new(
        "This is the doc title".to_string(),
        "Here goes the content of the applicaton".to_string(),
    )
    .update("Here goes the updated content of the application".to_string())
    .submit()
    .feedback()
    .update("Here goes the updated content of the application again".to_string())
    .submit()
    .issue()
    .expire();

    println!("Doc4: {doc4}");
}
