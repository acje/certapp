#![warn(clippy::pedantic)]
use cert_fsm::CertApp;
mod cert_fsm;

fn main() {
    let doc = CertApp::new("01.01.2024".to_string());
    println!("{doc}");
    let doc = doc.submit("02.01.2024".to_string());
    println!("{doc}");
    let doc = doc.decline("Det er et nei. 🤦".to_string(), "03.01.2024".to_string());
    println!("{doc}");

    //This will not compile because the document is in a declined state at this point
    //let doc = doc.submit("04.01.2024".to_string());

    // Chained state transitions
    let doc2 = CertApp::new("01.02.2024".to_string())
        .submit("02.02.2024".to_string())
        .issue("03.02.2024".to_string(), "04.02.2024".to_string());

    println!("{doc2}");
}
