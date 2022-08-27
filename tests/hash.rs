use ipi::value::hash::{Hash, Hasher};

fn test_hash(data: &[u8], expected_cid: &str) {
    // hash with Hash::with_bytes
    let hash = Hash::with_bytes(data).to_string();
    assert_eq!(hash, expected_cid);

    // hash with Hasher
    let mut hasher = Hasher::default();
    hasher.update(data);
    let hash = hasher.finalize().to_string();
    assert_eq!(hash, expected_cid);

    // hash with Hasher, coming with small chunks
    let mut hasher = Hasher::default();
    for chunk in data.chunks(1_000) {
        hasher.update(chunk);
    }
    let hash = hasher.finalize().to_string();
    assert_eq!(hash, expected_cid);

    // hash with Hasher, coming with large chunks
    let mut hasher = Hasher::default();
    for chunk in data.chunks(262_144 + 1) {
        hasher.update(chunk);
    }
    let hash = hasher.finalize().to_string();
    assert_eq!(hash, expected_cid);
}

#[test]
fn hash_small() {
    let data = b"hello world";
    let expected_cid = "bafkreifzjut3te2nhyekklss27nh3k72ysco7y32koao5eei66wof36n5e";

    test_hash(data, expected_cid);
}

#[test]
fn hash_chunk() {
    let data = &[0; 262_144];
    let expected_cid = "bafkreiekhhjkxu4ztk3tyng3er3ijhg56mb44oe3gwbgquhzu4afrg2ksa";

    test_hash(data, expected_cid);
}

#[test]
fn hash_dag_level_1() {
    let data = &[0; 262_144 * 174];
    let expected_cid = "bafybeibxsa3ioclowpaq7b6gxl65gzqneopfr3fnhedak6sqr4bjz5lnyq";

    test_hash(data, expected_cid);
}

#[test]
fn hash_dag_level_2_entry() {
    let data = &[0; 262_144 * 174 + 1];
    let expected_cid = "bafybeihqwzd3o6q6v3pmwhzjy22vokhr767burokmqemg63hptx2nqd7ym";

    test_hash(data, expected_cid);
}
