use crate::per_epoch_processing::Error;
use safe_arith::{SafeArith, SafeArithIter};
use types::{BeaconState, BeaconStateError, ChainSpec, EthSpec, Unsigned};

/// Process slashings.
pub fn process_slashings<T: EthSpec>(
    state: &mut BeaconState<T>,
    total_balance: u128,
    spec: &ChainSpec,
) -> Result<(), Error> {
    let epoch = state.current_epoch();
    let sum_slashings = state.get_all_slashings().iter().copied().safe_sum()? as u128;

    let adjusted_total_slashing_balance = std::cmp::min(
        sum_slashings.safe_mul(spec.proportional_slashing_multiplier_for_state(state) as u128)?,
        total_balance,
    );

    let (validators, balances, _) = state.validators_and_balances_and_progressive_balances_mut();
    for (index, validator) in validators.iter().enumerate() {
        if validator.slashed
            && epoch.safe_add(T::EpochsPerSlashingsVector::to_u64().safe_div(2)?)?
                == validator.withdrawable_epoch
        {
            let increment = spec.effective_balance_increment as u128;
            let effective_balance = validator.effective_balance as u128;
            let penalty_numerator = effective_balance
                .safe_div(increment)?
                .safe_mul(adjusted_total_slashing_balance)?;
            let penalty = penalty_numerator
                .safe_div(total_balance)?
                .safe_mul(increment)?;

            // Equivalent to `decrease_balance(state, index, penalty)`, but avoids borrowing `state`.
            let balance = balances
                .get_mut(index)
                .ok_or(BeaconStateError::BalancesOutOfBounds(index))?;
            *balance = balance.saturating_sub(penalty as u64);
        }
    }

    Ok(())
}
