use omega_adccl::adccl_logic::ADCCL;
use omega_eval::{default_suite, EvalRunner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adccl = ADCCL::new(0.7, None);
    let suite = default_suite();

    println!("--- Running Adversarial Battery ---");
    let report = EvalRunner::run_suite(&suite, |case| {
        // Synthetic response: use input text as the "response" for offline smoke test.
        let response = format!("Response to: {}", case.input);
        let verification = adccl.verify(&response, &case.input);
        (response, verification.score)
    });

    println!("{}", report.render_text());

    Ok(())
}
