use types::ChainSpec;

pub fn apply_burn(base_reward: u64, spec: &ChainSpec) -> u64 {
    let seconds_per_slot = spec.seconds_per_slot;

    // First we compensate for the increased block frequency.
    let after_burn = base_reward * seconds_per_slot / 12;

    // Then we burn an additional 25%.
    after_burn * 3 / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_burn() {
        let before_burn: u64 = 1000000;
        let mut spec = ChainSpec::mainnet();

        // Default 12 second slots => 25% general burn.
        let after_burn = apply_burn(before_burn, &spec);
        assert_eq!(after_burn, 750000);

        // 6 second slots => 50% burn then 25% general burn.
        spec.seconds_per_slot = 6;
        let after_burn = apply_burn(before_burn, &spec);
        assert_eq!(after_burn, 375000);

        // 3 second slots => 75% burn then 25% general burn.
        spec.seconds_per_slot = 3;
        let after_burn = apply_burn(before_burn, &spec);
        assert_eq!(after_burn, 187500);
    }
}
