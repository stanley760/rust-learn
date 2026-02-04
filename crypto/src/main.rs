mod secp256k1;

use secp256k1::{Secp256k1Error, Secp256k1Signer, Secp256k1Verifier};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Secp256k1 椭圆曲线签名演示 ===\n");

    // 1. 生成密钥对
    println!("1. 生成密钥对...");
    let signer = Secp256k1Signer::generate();
    let verifier = Secp256k1Verifier::from_signer(&signer);

    println!("   私钥: {}", hex::encode(&signer.to_bytes()));
    println!("   公钥 (压缩): {}", hex::encode(&signer.public_key_compressed()));
    println!("   公钥 (未压缩): {}", hex::encode(&signer.public_key_uncompressed()));
    println!();

    // 2. 签名消息
    let message = "Hello, Secp256k1!";
    println!("2. 签名消息: \"{}\"", message);
    let signature = signer.sign(message.as_bytes());
    println!("   签名 (r): {}", hex::encode(&signature.r().to_bytes()));
    println!("   签名 (s): {}", hex::encode(&signature.s().to_bytes()));
    println!();

    // 3. 验证签名
    println!("3. 验证签名...");
    let is_valid = verifier.verify(message.as_bytes(), &signature)?;
    println!("   签名验证结果: {}", if is_valid { "有效 ✓" } else { "无效 ✗" });
    println!();

    // 4. 验证错误消息
    println!("4. 验证错误消息...");
    let wrong_message = "Wrong message";
    let is_valid = verifier.verify(wrong_message.as_bytes(), &signature)?;
    println!("   错误消息验证结果: {}", if is_valid { "有效 ✗ (意外!)" } else { "无效 ✓ (预期)" });
    println!();

    // 5. 密钥序列化演示
    println!("5. 密钥序列化演示...");
    let keypair = signer.export_keypair();
    println!("   导出的私钥长度: {} 字节", keypair.private_key.len());
    println!("   导出的公钥长度: {} 字节", keypair.public_key.len());

    // 从字节重建
    let restored_signer = Secp256k1Signer::from_bytes(&keypair.private_key)?;
    let restored_verifier = Secp256k1Verifier::from_bytes(&keypair.public_key)?;
    let is_valid = restored_verifier.verify(message.as_bytes(), &signature)?;
    println!("   重建后验证结果: {}", if is_valid { "有效 ✓" } else { "无效 ✗" });
    println!();

    // 6. 带恢复 ID 的签名
    println!("6. 带恢复 ID 的签名演示...");
    let (sig, recovery_id) = signer.sign_recoverable(message.as_bytes());
    println!("   恢复 ID: {}", recovery_id);

    let recovered_key =
        Secp256k1Verifier::recover_from_signature(message.as_bytes(), &sig, recovery_id)?;
    println!("   恢复的公钥: {}", hex::encode(recovered_key.to_sec1_bytes()));
    println!("   原始公钥:   {}", hex::encode(signer.verifying_key().to_sec1_bytes()));
    println!("   公钥匹配: {}", recovered_key == signer.verifying_key());
    println!();

    // 7. 交互式演示
    println!("=== 交互式演示 ===");
    println!("输入要签名的消息（按 Enter 跳过）:");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    if !user_input.trim().is_empty() {
        let user_signer = Secp256k1Signer::generate();
        let user_verifier = Secp256k1Verifier::from_signer(&user_signer);

        println!("\n您的私钥: {}", hex::encode(&user_signer.to_bytes()));
        println!("您的公钥: {}", hex::encode(&user_signer.public_key_compressed()));

        let user_sig = user_signer.sign(user_input.trim().as_bytes());
        println!("签名: {}", hex::encode(&user_sig.to_bytes()));

        let valid = user_verifier.verify(user_input.trim().as_bytes(), &user_sig)?;
        println!("验证结果: {}", if valid { "有效 ✓" } else { "无效 ✗" });
    }

    println!("\n=== 演示完成 ===");
    Ok(())
}

// 为 Box<dyn std::error::Error> 实现 From<Secp256k1Error>
impl From<Secp256k1Error> for Box<dyn std::error::Error> {
    fn from(err: Secp256k1Error) -> Self {
        Box::new(err) as Box<dyn std::error::Error>
    }
}
