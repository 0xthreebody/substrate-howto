use AccountId;
use parity_codec::Encode;

use srml_support::{StorageValue, StorageMap, dispatch::Result};
use runtime_primitives::traits::BlakeTwo256;
// use primitives::hash::H256;
// use balances::{address::Address}; 
use balances; 
use system::{ensure_signed};
// use system;

pub trait Trait: balances::Trait {}

// ClaimIndex used as key for storage
pub type ClaimIndex = u32;

// result of fingerprinthash
pub type HashRef = BlakeTwo256;

pub type AccountAddr = AccountId;

// type defined for creating fingerprint of data.

// Claim details
// pub struct Claim<AccountAddr, Balance, HashRef, bool> {
// 	claimant: AccountAddr,
// 	respondent: AccountAddr,
// 	amount: Balance,
// 	docHash: HashRef,
// 	settled: bool,
// }

// determine parties in the claim
pub struct Index <AccountAddr>{
	claimant: AccountAddr,
	respondent: AccountAddr,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn processClaim(
			origin, //claimant address
			respondent: AccountAddr, // respondent address
			amount: T::Balance, // amount of claim
			doc: String, // a string reference provided by claimant
			settled: bool ) -> Result {
			
			let claimant = ensure_signed(origin)?;
			let amount = amount.into();
			let settled = false;

			let c = Self::claims_count(); // get nr of claims, c will always be the current value 
			<ClaimsCount<T>>::put(c + 1); // Store new count. c is new value now.
			
			// create a fingerprint Hash. is the result H256 or BlakeTwo... ?
			let docHash = T::Hashing::hash(&doc, c.to_string());
			
			// Store claim. 
			// <ClaimsList<T>>::insert(c, Claim { 
			// 	claimant, 
			// 	respondent, 
			// 	amount, 
			// 	docHash, 
			// 	settled 
			// });

			//Associate index with accounts
			<AccountsByClaimIndex<T>>::insert(c, Index { 
				claimant, 
				respondent
			});

			// Associate Accounts with indices
			// <ClaimsByAccount<T>>::insert(claimant, c);
			// <ClaimsByAccount<T>>::insert(respondent, c);

			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Basictest {		
		// Simple Claim index counter. Returns the latest ClaimIndex Number
		ClaimsCount get(claims_count): ClaimIndex;
		
		// Store for Accounts By ClaimIndex. Should return the two accounts associated with each claimIndex number (claimant and respondent). 
		AccountsByClaimIndex get(accounts_by_claim_index): map ClaimIndex => Option<Index<AccountAddr>>;	
		
		// Store for ClaimIndices associated with a given account. Should return the same index for either claimant and respondent parties of the same claim. 
		// ClaimsByAccount get(claim_index_by_account): map Address<T::AccountId, T::AccountIndex> => Option<Vec<ClaimIndex>>;			
		
		// Store for all claims by claim index.
		// ClaimsList get(claims_list): map ClaimIndex => Option<Claim<Address<T::AccountId, T::AccountIndex>, T::Balance, HashRef, bool>>;	
	}
}
