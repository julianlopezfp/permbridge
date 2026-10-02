use permbridge_core::Decision;

fn main() {
    // A startup smoke test only; this CLI does not load policies or compare agents.
    let _ = Decision::default();
    println!("PermBridge CLI scaffold: posture comparison is not implemented.");
}
