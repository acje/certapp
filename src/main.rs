#![warn(clippy::pedantic)]
use cert_fsm::CertApp;
mod cert_fsm;

fn main() {
    let doc = CertApp::new("2024-01-01".to_string());
    println!("{doc}");
    let doc = doc.submit("2024-01-02".to_string());
    println!("{doc}");
    let doc = doc.decline("Det er et nei. 🤦".to_string(), "2024-01-03".to_string());
    println!("{doc}");

    //This will not compile because the document is in a declined state at this point
    //let doc = doc.submit("2024-01-04".to_string());

    // Chained state transitions
    let doc2 = CertApp::new("01.02.2024".to_string())
        .submit("02.02.2024".to_string())
        .issue("03.02.2024".to_string(), "04.02.2024".to_string());

    println!("{doc2}");
}
