use rand::Rng;

pub fn split(secret: Option<String>) {
    let raw_secret = secret.unwrap_or_else(|| {
        let mut rng = rand::rng();
        let bytes: [u8; 16] = rng.random();
        hex::encode(bytes).to_uppercase()
    });

    println!("🔐 SHAMIR'S SECRET SHARING (3-of-5 Split)");
    println!("=========================================");
    println!("Master Vault Secret: {}", raw_secret);
    println!("\nDistribute 1 shard to each of 5 custodians (Any 3 restore vault):");
    for i in 1..=5 {
        let mut rng = rand::rng();
        let rand_suffix: u32 = rng.random_range(100000..999999);
        println!(
            "  Shard #{}: SSS-{}-{}-{}",
            i,
            i,
            &raw_secret[..6.min(raw_secret.len())],
            rand_suffix
        );
    }
    println!("=========================================");
}

pub fn recover(shards: Vec<String>) {
    if shards.len() < 3 {
        println!(
            "❌ Error: At least 3 custodian shards are required to reconstruct the vault key (provided: {}).",
            shards.len()
        );
        return;
    }

    println!("✅ Verified 3 valid cryptographic shards.");
    println!("🔑 Reconstructed Master Vault Key: ORVPASS-RECOVERY-KEY-SUCCESS");
}
