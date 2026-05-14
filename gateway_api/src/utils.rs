pub fn gateway_base_url(network_id: u8) -> &'static str {
    match network_id {
        1 => "https://mainnet.radixdlt.com",
        2 => "https://stokenet.radixdlt.com",
        _ => "https://mainnet.radixdlt.com",
    }
}