extern crate bitcoin;
extern crate cfd_rust;
extern crate hex;
extern crate secp256k1;

use self::cfd_rust as cfd;
// use alloc::string::ToString;

fn main() {
  let prvikey_cfd_ret = cfd::Privkey::generate(&cfd::Network::Mainnet, true);
  if let Err(e) = prvikey_cfd_ret {
    println!("Err: {}", e);
    return;
  }
  let prvikey_cfd = prvikey_cfd_ret.unwrap();

  let sec_key_ret = secp256k1::key::SecretKey::from_slice(prvikey_cfd.to_slice());
  if let Err(e) = sec_key_ret {
    println!("Err: {}", e);
    return;
  }
  let sec_key = sec_key_ret.unwrap();

  let privkey_btc = bitcoin::PrivateKey {
    compressed: true,
    network: bitcoin::Network::Bitcoin,
    key: sec_key,
  };
  let wif_btc = privkey_btc.to_wif();
  let wif_cfd = prvikey_cfd.to_wif();
  println!("wif(btc)   : {}", wif_btc);
  println!("wif(cfd)   : {}", wif_cfd);

  let secp = secp256k1::Secp256k1::new();
  let pubkey_btc = privkey_btc.public_key(&secp);
  let pubkey_btc_str = hex::encode(&pubkey_btc.to_bytes());

  let pubkey_cfd_ret = prvikey_cfd.get_pubkey();
  if let Err(e) = pubkey_cfd_ret {
    println!("Err: {}", e);
    return;
  }
  let pubkey_cfd = pubkey_cfd_ret.unwrap();

  println!("pubkey(btc): {}", pubkey_btc_str);
  println!("pubkey(cfd): {}", pubkey_cfd.to_hex());
  println!("pubkey(btc): {}", pubkey_btc.to_string());
}
