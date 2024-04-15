use sqids::Sqids;

pub fn generate_sqid() -> Sqids {
	Sqids::builder()
		.min_length(6)
		.alphabet("abcdefghijklmnopqrstuvwxyz0123456789".chars().collect())
		.build()
		.unwrap()
}
