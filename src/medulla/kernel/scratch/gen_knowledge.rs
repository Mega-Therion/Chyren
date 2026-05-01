use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MatrixProgram {
    pub domain: String,
    pub version: String,
    pub integrity_hash: String,
    pub payload: Vec<u8>,
}

fn create_program(domain: &str, version: &str, content: &str) -> MatrixProgram {
    let payload = content.as_bytes().to_vec();
    let integrity_hash = hex::encode(Sha256::digest(&payload));
    MatrixProgram {
        domain: domain.to_string(),
        version: version.to_string(),
        integrity_hash,
        payload,
    }
}

fn main() {
    let p1 = create_program("mathematics", "1.0.0", "Axiom: Riemann Zeta Function ζ(s) = ∑ n⁻ˢ. Critical strip: 0 < Re(s) < 1.");
    let p2 = create_program("topology", "1.0.0", "Definition: A topological space is a set X with a collection of subsets T satisfying the three axioms of openness.");
    let p3 = create_program("logic", "1.0.0", "Principle: The Yettragrammaton (R.W.Ϝ.Y.) requires that truth be measurable, verifiable, and recursively stable.");

    std::fs::write("knowledge_injection/zeta_axioms.json", serde_json::to_string_pretty(&p1).unwrap()).unwrap();
    std::fs::write("knowledge_injection/topology_foundations.json", serde_json::to_string_pretty(&p2).unwrap()).unwrap();
    std::fs::write("knowledge_injection/logic_purity.json", serde_json::to_string_pretty(&p3).unwrap()).unwrap();
    
    println!("Generated 3 axiom programs in knowledge_injection/");
}
