/// An example module that manages rewards for multiple tokens on a per-epoch basis. Rewards can be added for multiple
/// tokens by anyone for any epoch but only friended modules can increase/decrease claimer shares.
///
/// This module is designed to be integrated into a complete system that manages epochs and rewards.
///
/// The flow works as below:
/// 1. A rewards pool is created with a set of reward tokens (fungible assets). If coins are to be used as rewards,
/// developers can use the coin_wrapper module from move-examples/swap to convert coins into fungible assets.
/// 2. Anyone can add rewards to the pool for any epoch for multiple tokens by calling add_rewards.
/// 3. Friended modules can increase/decrease claimer shares for current epoch by calling increase_allocation and
/// decrease_allocation.
/// 4. Claimers can claim their rewards in all tokens for any epoch that has ended by calling claim_rewards, which
/// return a vector of all the rewards. Claiming also removes the claimer's shares from that epoch's rewards as their
/// rewards have all been claimed.
///
/// Although claimers have to be signers, this module can be easily modified to support objects (e.g. NFTs) as claimers.
module rewards_pool::rewards_pool {
    use aptos_framework::fungible_asset::{Self, FungibleAsset, FungibleStore, Metadata};
    use aptos_framework::primary_fungible_store;
    use aptos_framework::object::{Self, Object, ExtendRef};
    use aptos_std::pool_u64_unbound::{Self as pool_u64, Pool};
    use aptos_std::simple_map::{Self, SimpleMap};
    use aptos_std::smart_table::{Self, SmartTable};

    use rewards_pool::epoch;

    use std::signer;
    use std::vector;

    /// Rewards can only be claimed for epochs that have ended.
    const EREWARDS_CANNOT_BE_CLAIMED_FOR_CURRENT_EPOCH: u64 = 1;
    /// The rewards pool does not support the given reward token type.
    const EREWARD_TOKEN_NOT_SUPPORTED: u64 = 2;

    /// Data regarding the rewards to be distributed for a specific epoch.
    struct EpochRewards has store {
        /// Total amount of rewards for each reward token added to this epoch.
        total_amounts: SimpleMap<Object<Metadata>, u64>,
        /// Pool representing the claimer shares in this epoch.
        claimer_pool: Pool,
    }

    /// Data regarding the store object for a specific reward token.
    struct RewardStore has store {
        /// The fungible store for this reward token.
        store: Object<FungibleStore>,
        /// We need to keep the fungible store's extend ref to be able to transfer rewards from it during claiming.
        store_extend_ref: ExtendRef,
    }

    #[resource_group_member(group = aptos_framework::object::ObjectGroup)]
    struct RewardsPool has key {
        /// A mapping to track per epoch rewards data.
        epoch_rewards: SmartTable<u64, EpochRewards>,
        /// The stores where rewards are kept.
        reward_stores: SimpleMap<Object<Metadata>, RewardStore>,
    }



 
    /// This should only be called by system modules to increase the shares of a claimer for the current epoch.
    public(friend) fun increase_allocation(
        claimer: address,
        rewards_pool: Object<RewardsPool>,
        amount: u64,
    ) acquires RewardsPool {
        let epoch_rewards = &mut unchecked_mut_rewards_pool_data(&rewards_pool).epoch_rewards;
        let current_epoch_rewards = epoch_rewards_or_default(epoch_rewards, epoch::now());
        pool_u64::buy_in(&mut current_epoch_rewards.claimer_pool, claimer, amount);

    }


    inline fun epoch_rewards_or_default(
        epoch_rewards: &mut SmartTable<u64, EpochRewards>,
        epoch: u64,
    ): &mut EpochRewards acquires RewardsPool {
        if (!smart_table::contains(epoch_rewards, epoch)) {
            smart_table::add(epoch_rewards, epoch, EpochRewards {
                total_amounts: simple_map::new(),
                claimer_pool: pool_u64::create(),
            });
        };
        smart_table::borrow_mut(epoch_rewards, epoch)
    }


    inline fun unchecked_mut_rewards_pool_data(
        rewards_pool: &Object<RewardsPool>,
    ): &mut RewardsPool acquires RewardsPool {
        borrow_global_mut<RewardsPool>(object::object_address(rewards_pool))
    }

    #[test_only]
    friend rewards_pool::rewards_pool_tests;
}
