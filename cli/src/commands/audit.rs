use orvpass_core::models::{ItemData, VaultItem};
use std::collections::HashMap;

pub fn execute(items: &[VaultItem], json_output: bool) {
    let mut total = 0;
    let mut weak = 0;
    let mut reused_count = 0;
    let mut pass_map: HashMap<String, Vec<String>> = HashMap::new();

    for item in items {
        if let ItemData::Login(login) = &item.data {
            total += 1;
            if let Some(pass) = &login.password {
                if pass.len() < 12 {
                    weak += 1;
                }
                pass_map.entry(pass.clone()).or_default().push(item.title.clone());
            }
        }
    }

    for (_, titles) in &pass_map {
        if titles.len() > 1 {
            reused_count += titles.len();
        }
    }

    let score = if total == 0 {
        100
    } else {
        let penalty = ((weak as f32 / total as f32) * 40.0) + ((reused_count as f32 / total as f32) * 40.0);
        (100.0 - penalty).clamp(10.0, 100.0) as u32
    };

    if json_output {
        let out = serde_json::json!({
            "score": score,
            "total_accounts": total,
            "weak_passwords": weak,
            "reused_passwords": reused_count,
            "status": if score >= 90 { "EXCELLENT" } else if score >= 70 { "GOOD" } else { "NEEDS_ATTENTION" }
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("🛡️  ORVPASS WATCHDOG SECURITY AUDIT");
        println!("=====================================");
        println!("  Health Score:      {}% ({})", score, if score >= 90 { "✅ SECURE" } else { "⚠️ ATTENTION NEEDED" });
        println!("  Total Logins:      {}", total);
        println!("  Weak (<12 chars):  {}", weak);
        println!("  Reused Passwords:  {}", reused_count);
        println!("=====================================");
        if weak > 0 || reused_count > 0 {
            println!("💡 Recommendation: Run 'orvpass generate -d' to upgrade weak passwords to Diceware passphrases.");
        } else {
            println!("✨ Vault is 100% compliant with zero detected vulnerabilities.");
        }
    }
}
