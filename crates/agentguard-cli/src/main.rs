use agentguard_core::Decision;

fn main() {
    // A startup smoke test only; this CLI does not load or evaluate policies.
    let _ = Decision::default();
    println!("AgentGuard CLI scaffold: policy evaluation is not implemented.");
}
