use ipi::value::hash::Hash;

#[test]
fn hash_small() {
    let data = b"hello world";

    let hash = Hash::with_bytes(data).to_string();
    assert_eq!(
        hash,
        "bafkreifzjut3te2nhyekklss27nh3k72ysco7y32koao5eei66wof36n5e",
    );
}

#[test]
fn hash_chunk() {
    let data = &[0; 262_144];

    let hash = Hash::with_bytes(data).to_string();
    assert_eq!(
        hash,
        "bafkreiekhhjkxu4ztk3tyng3er3ijhg56mb44oe3gwbgquhzu4afrg2ksa",
    );
}

#[test]
fn hash_dag_level_1() {
    let data = &[0; 262_144 * 174];

    let hash = Hash::with_bytes(data).to_string();
    assert_eq!(
        hash,
        "bafybeibxsa3ioclowpaq7b6gxl65gzqneopfr3fnhedak6sqr4bjz5lnyq",
    );
}

#[test]
fn hash_dag_level_2_entry() {
    let data = &[0; 262_144 * 174 + 1];

    let hash = Hash::with_bytes(data).to_string();
    assert_eq!(
        hash,
        "bafybeihqwzd3o6q6v3pmwhzjy22vokhr767burokmqemg63hptx2nqd7ym",
    );
}
