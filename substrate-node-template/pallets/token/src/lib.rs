#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	traits::{Currency, ExistenceRequirement},
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
//use sp_runtime::traits::StaticLookup;
use codec::{Encode, Decode};
use primitives::Token;
use frame_support::dispatch::DispatchResult;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct TokenInfo {
	pub token_balance: u32,						//可用余额
	pub token_stake: u32,						  //交易质押
	pub token_vote: u32,						   //投票质押
}

impl Default for TokenInfo {
	fn default() -> Self {
		TokenInfo {
			token_balance: 0,
			token_stake: 0,
			token_vote: 0,
		}
	}
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		pub BuyRate get(fn buyrate):  u32;							 //购买通证汇率
		pub SellRate get(fn sellrate):  u32;							//出售通证汇率
		pub BalanceToken get(fn balancetoken):  map hasher(blake2_128_concat) T::AccountId => TokenInfo;		//某地址对应的通证数量
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		TokenAcountNotExist,
		BalanceNotEnough,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		pub fn buytoken(
			origin, 
			buy_token: u32, 
			treasure: T::AccountId, 
			amount_price: BalanceOf<T>
		) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			BuyRate::put(102);

			let amount = buy_token * BuyRate::get() / 100;

			if <BalanceOf<T>>::from(amount) == amount_price {
				let mut tokeninfo = <BalanceToken<T>>::get(&sender);
				tokeninfo.token_balance += buy_token;

				BalanceToken::<T>::insert(&sender, tokeninfo);

				T::Currency::transfer(&sender, &treasure, amount_price, ExistenceRequirement::KeepAlive)?;
			}

			Ok(())
		}

		#[weight = 0]
		pub fn selltoken(
			origin, 
			sell_token: u32, 
			treasure: T::AccountId, 
			amount_price: BalanceOf<T>
		) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			SellRate::put(99);

			let amount = sell_token * SellRate::get() / 100;

			if <BalanceOf<T>>::from(amount) == amount_price {
				let mut tokeninfo = <BalanceToken<T>>::get(&sender);
				ensure!(tokeninfo.token_balance > sell_token, Error::<T>::BalanceNotEnough);

				tokeninfo.token_balance -= sell_token;

				BalanceToken::<T>::insert(&sender, tokeninfo);
				
				T::Currency::transfer(&treasure, &sender, amount_price, ExistenceRequirement::KeepAlive)?;
			}

			Ok(())
		} 

		#[weight = 0]
		pub fn transfertoken(
			_origin, 
			from: T::AccountId, 
			to: T::AccountId, 
			token_amount: u32
		) -> dispatch::DispatchResult{
			Self::do_transfertoken(from, to, token_amount)?;
			Ok(())
		}

		#[weight = 0]
		pub fn incentivetoken(
			origin, 
			incentive_status: bool, 
			incentive_token: u32
		) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			Self::do_incentivetoken(sender, incentive_status, incentive_token)?;
			Ok(())
		}

		#[weight = 0]
		pub fn staketoken(
			origin, 
			stake_token: u32
		) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			Self::do_staketoken(sender, stake_token)?;
			Ok(())
		}

		#[weight = 0]
		pub fn votetoken(
			origin, 
			vote_token: u32
		) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			Self::do_votetoken(sender, vote_token)?;
			Ok(())
		}

	}
}

impl<T:Trait> Token<T::AccountId> for Module<T>{
	fn do_incentivetoken(sender: T::AccountId,incentive_status: bool, incentive_token: u32) -> dispatch::DispatchResult {
		let mut tokeninfo = <BalanceToken<T>>::get(&sender);
		match incentive_status {
			true => tokeninfo.token_balance += incentive_token,				//正向激励
			false => tokeninfo.token_balance -= incentive_token,			//负向激励
		}
		BalanceToken::<T>::insert(&sender, tokeninfo);
		Ok(())
	}
	
	fn do_staketoken(sender: T::AccountId,stake_token:u32) -> dispatch::DispatchResult {
		let mut tokeninfo = <BalanceToken<T>>::get(&sender);
		tokeninfo.token_balance -= stake_token;
		tokeninfo.token_stake += stake_token;

		BalanceToken::<T>::insert(&sender, tokeninfo);
		Ok(())
	}
	
	fn do_votetoken(sender: T::AccountId,vote_token:u32) -> dispatch::DispatchResult {
		let mut tokeninfo = <BalanceToken<T>>::get(&sender);
		tokeninfo.token_balance -= vote_token;
		tokeninfo.token_vote += vote_token;

		BalanceToken::<T>::insert(&sender, tokeninfo);

		Ok(())
	}
	
	fn do_transfertoken(from: T::AccountId, to: T::AccountId, token_amount: u32) -> DispatchResult {
		let mut from_tokeninfo = <BalanceToken<T>>::get(&from);
		let mut to_tokeninfo = <BalanceToken<T>>::get(&to);

		ensure!(from_tokeninfo.token_balance > token_amount, Error::<T>::BalanceNotEnough);

		from_tokeninfo.token_balance -= token_amount;
		to_tokeninfo.token_balance += token_amount;
		Ok(())
	}
}
