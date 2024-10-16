use crate::models::ecpay_result_model::EcpayResultVO;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Write;
use std::str;

pub fn is_check_mac_value_match(
    hash_key: &str,
    hash_iv: &str,
    input: &HashMap<String, String>,
) -> Result<EcpayResultVO, Box<dyn std::error::Error>> {
    let mut check_mac_value = String::new();
    let mut key_value_map: HashMap<String, String> = HashMap::new();

    // Iterate over the input HashMap
    for (key, value) in input {
        if key.to_lowercase() != "checkmacvalue" {
            key_value_map.insert(key.to_lowercase(), value.to_string());
        } else {
            check_mac_value = value.to_string();
        }
    }

    // Build the check string
    let mut check_str = String::new();
    for (key, value) in &key_value_map {
        if !check_str.is_empty() {
            check_str.push('&');
        }
        write!(&mut check_str, "{}={}", key, value)?;
    }
    check_str = format!("HashKey={}&{}&HashIV={}", hash_key, check_str, hash_iv);

    // URL encode the string
    let encoded_str = utf8_percent_encode(&check_str, NON_ALPHANUMERIC)
        .to_string()
        .to_lowercase();

    // Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(encoded_str.as_bytes());
    let hash_result = hasher.finalize();

    // Convert hash to hex
    let new_check_mac_value = hash_to_hex(&hash_result).to_uppercase();

    // Populate result VO
    let mut ecpay_result_vo = EcpayResultVO::default();
    if new_check_mac_value == check_mac_value
        && key_value_map
            .get("rtncode")
            .map(|v| v == "1")
            .unwrap_or(false)
    {
        ecpay_result_vo.is_match = 1;
        ecpay_result_vo.card4no = key_value_map.get("card4no").cloned();
        ecpay_result_vo.card6no = key_value_map.get("card6no").cloned();
        ecpay_result_vo.customfield1 = key_value_map.get("customfield1").cloned();
        ecpay_result_vo.customfield2 = key_value_map.get("customfield2").cloned();
        ecpay_result_vo.gwsr = key_value_map.get("gwsr").cloned();
        ecpay_result_vo.merchantid = key_value_map.get("merchantid").cloned();
        ecpay_result_vo.merchanttradeno = key_value_map.get("merchanttradeno").cloned();
        ecpay_result_vo.paymentdate = key_value_map.get("paymentdate").cloned();
        ecpay_result_vo.rtncode = key_value_map.get("rtncode").cloned();
        ecpay_result_vo.rtnmsg = key_value_map.get("rtnmsg").cloned();
        ecpay_result_vo.storeid = key_value_map.get("storeid").cloned();
        ecpay_result_vo.tradeamt = key_value_map.get("tradeamt").cloned();
        ecpay_result_vo.tradeno = key_value_map.get("tradeno").cloned();
    } else {
        ecpay_result_vo.is_match = 0;
        ecpay_result_vo.rtncode = key_value_map.get("rtncode").cloned();
        ecpay_result_vo.rtnmsg = key_value_map.get("rtnmsg").cloned();
    }

    Ok(ecpay_result_vo)
}

fn hash_to_hex(hash: &[u8]) -> String {
    let hex_chars = "0123456789ABCDEF".as_bytes();
    let mut hex_str = String::with_capacity(hash.len() * 2);

    for byte in hash {
        hex_str.push(hex_chars[(byte >> 4) as usize] as char);
        hex_str.push(hex_chars[(byte & 0x0F) as usize] as char);
    }

    hex_str
}
