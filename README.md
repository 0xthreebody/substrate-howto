# How to build a Custom Substrate Module from scratch.

ORIGINAL: <https://gitlab.com/chrisdcosta/basictest/blob/master/Howto.md#how-to-build-a-custom-substrate-module-from-scratch>  

I wrote this piece because I am new to Rust, and struggling to hack out a new module from existing ones without coming across multiple compile errors. Also, looking at both the examples in the SRML modules, and the crates that contained some of the underlying code showed that there were some incredible possibilities, that weren't full explored yet in the SRML modules.

What I set out to do was build a simple module from scratch that covered some of the general requirements that may be needed in projects:
- storing and retrieving single values on chain 
- storing and retrieving multiple values on chain, using a structure, a mapping, a vector array
- working with and storing one or more addresses
- working with a hashed value
- working with a boolean value

Initially I deployed the demo.rs Coinflip game from [Gavin Wood's fantastic presentation in October 2018](https://www.youtube.com/watch?v=0IoUZdDi5Is) to make sure that my installation was functionning correctly. Note that the implementation code has changed since that presentation, but you can find the up-to-date code [here](https://substrate.readme.io/docs/creating-a-custom-substrate-chain#section-step-2-add-alice-to-your-network). I also built a small function that allowed anyone to steal the funds from the Pot (!) and added to the UI to check I could actually work with this. All was good.

Then, when I tried to write a module by reviewing the SRML modules and trying to copy some of the functionality. I hit a wall because I did not understand the imports to the modules and why they are important. I reached out to Gavin who gave me a pointer for some specific questions, and this post is building on that learning by breaking things down for absolute beginners (which I guess I still am).

## Step 1 - Start with the lib.rs file
Why here? Basically because the compiler is going to help you resolve issues with its error messages as we go along. In this case I added the following lines, just as Gavin did in the demo. You will have to search for the appropriate positions, this is not a free lunch!

	mod basictest;
	...

	impl basictest::Trait for Runtime {
	}
	...

	Basictest: basictest::{Module, Call, Storage, Config<T>},

There will be an issue with the last line because we have not implemented any of that stuff yet, so change it to:

	Basictest: basictest::{},

## Step 2 - start with an empty module
Why? because if you start with an empty module you get to understand what is going on. Let's create an empty module file called `basictest.rs`

Let's compile your code using `./build.sh` as per Gav's demo. You will hit some errors in `lib.rs` but at this stage do not worry.

Add the following structure declaration in `basictest.rs`: 

	#[derive(PartialEq, Eq, Clone, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct Claim<Balance, AccountId, Hash, Boolean> {
		pub claimant: AccountId,
		pub respondent: AccountId,
		pub amount: Balance,
		pub doc_hash: Hash,
		pub settled: Boolean,
	}

It is very important that you add the first two lines - they are needed for building a struct if it is going to be used for storage. In particular `derive` is used with `structs`, `enums` and `unions`. 

If you compile again you should not get any specific errors on the `basictest.rs` file. You will still get errors in the `lib.rs` file.

Add the following line, which defines a `Trait` for our module. 

	pub trait Trait: balances::Trait {}

You get the first compile error on `basictest.rs`  This is because we now need to use an external crate. It is already declared in the `lib.rs` file but we need to tell the module to use it. Add this line.

	use balances;

Next we will create a module to handle our functions.

	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
	}

Now we have to go back to `lib.rs` and tell it that we have created this module and call. Change this line: 

	Basictest: basictest::{Module, Call},

At this point the program should compile without errors. Congratulations!

## Step 3 on-chain storage, single value.
This part is necessary if you are going to store data on chain. It helps if you have your data model decided but I am going to explain the various aspects going forward.

Add the following lines:

	decl_storage! {
		trait Store for Module<T: Trait> as Basictest {}
	}

You should still be able to compile with no errors.

We are going to create on-chain storage for a single value - in this case a claim index of type `u32` a 32-bit unsigned integer and use it in the storage definition.

It is important to understand that Substrate declares the storage definition as a getter function that is available to the front-end UI as well as this back-end code. The declaration also performs as a setter function when called appropriately, which we will do later. 

Lets see how that works, add this line:

	pub type ClaimIndex = u32;

now inside our `Store`we are going to create the declaration that will handle on-chain storage.	
	
	decl_storage! {
		trait Store for Module<T: Trait> as Basictest {
			// Simple Claim index counter. Returns the latest ClaimIndex Number
			pub ClaimsCount get(claims_count): ClaimIndex;
		}
	}

Declaring the storage this way is equivalent to `pub ClaimsCount get(claims_count): u32;` but because we name the custom type, we can use it in our code in a readable way. 

## Step 4 Getting and Setting the value in storage
In this step we are going to get the value from storage (if any) and set it to a new value.

First we need to create a function that will do this. This function will be seen in our front-end so we will also need to get it to return a result. In this case we need to use some functionality from `srml_support`. Add this line:

	use srml_support::{dispatch::Result};

Now we can build a boilerplate function with the ability to return a result:

	fn processClaim() -> Result {
		Ok(())
	}


Before we can add a value we need to again use some functionality from `srml_supoort`. We need to allow us to store a value:

	use srml_support::{StorageValue, dispatch::Result};

Now we can add some functionality to our function. These two lines do a great deal of work!

	fn processClaim() -> Result {	
		let c = Self::claims_count(); // get nr of claims, c will always be the current value
		<ClaimsCount<T>>::put(c + 1); // Store new count. c is new value.
		Ok(())
	}

In the first line we are declaring the counter `c` as a function based on our declaration of the on-chain storage we declared earlier. Remember this is a getter function which means therefore that `c` will always reference the current value held in storage.

The second line is referencing the storage location `ClaimsCount` and putting the result of `c + 1` . In this case `c` behaves like a variable for the sake of the calculation, but as soon as it is stored, `c` will have the new value.

## Step 5 on-chain storage, using a struct.
This one is a little more difficult. Let's add some input arguments to our function:

	fn processClaim(
		origin,
		respondent: T::AccountId,
		amount: T::Balance,
		documenttext: String
		) -> Result {
		
		let c = Self::claims_count(); // get nr of claims, c will always be the current value
		<ClaimsCount<T>>::put(c + 1); // Store new count. c is new value.
	
		Ok(())
	}

These arguments will need to be completed in our transaction creation process when calling `processClaim`. We are going to ask the function to confirm that the transaction has been signed, but before we do that, we need to import that functionality:

	use system::{ensure_signed}; 

Reviewing our parameters we see that the first one is `origin`. This is first by convention and indeed we do not need to specify the type because it is included in the `decl_module!` above:

	...
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
	...

Origin has a specific meaning - it is the sender of the transaction. Internally it reference the Account ID and the Account index.

The other arguments are typed also. So far so good, but now we need to do something with them. What we are about to do is take these arguments, and map them to the struct, whilst at the same time inserting the values into the storage. First let's include the functionalty for mapping, and setting up the values we want to store:

	use srml_support::{StorageValue, StorageMap, dispatch::Result};

Now let's set up some of the input arguments:

	let claimant = ensure_signed(origin)?;
	let amount: T::Balance = amount.into();
	let settled: bool = false;

The first line takes the sender and makes sure that it has signed the transaction. Presumably we can create modules that can execute code but do not require a signature (I have not tries this yet.).

The second line puts the value into amount.

The third line sets a boolean vale to false, answering the question "has this claim been settled?". 

To go further we are going to need to import three more standard modules for functionality. We are going to take the string entered in `documenttext` and encode it as a hash from it. To do this we need to declare a variable with the type `T::Hash`  and include the modle for encoding:

	use parity_codec::Encode;

Next we will need to declare the system module so that we can access the system traits. We can do this two ways:
- we can explicitlt declare it like this: `use system;`
- or else we can adapt the system declaration we made earlier, by asking it to refer to itself: `use system::{self, ensure_origin}`

Finally we need to import the hash functions from the `runtime_primitives` module:

	use runtime_primitives::traits::{Hash};

Now that we have completed this, we can finally perform our hash on the data entered by the user... but wait... oh dear, there is a problem with the `String` input type! In Rust strings are a special beast. They are essentially a vector array of 8 byte characters. We will not be able to compile this until we declare the input type better.

So first let's declare a custom type. We need to include the Vec functionality from srml_support:

	use srml_support::{StorageValue, StorageMap, dispatch::Result, dispatch::Vec};

Then we can declare our input type:

		fn processClaim(
			origin,
			respondent: T::AccountId,
			amount: T::Balance,
			documenttext: Vec<u8>
			) -> Result {

And at last we can handle our hashing instruction:

	let doc_hash: T::Hash = documenttext.using_encoded(<T as system::Trait>::Hashing::hash);

The last step is to take everything we have done so far and use it to store the claim on the chain:

	// Store claim on chain
	<ClaimsByClaimIndex<T>>::insert(c, Claim { 
		claimant, 
		respondent, 
		amount,
		doc_hash, 
		settled
	});

We are referencing the storage location `ClaimsByClaimIndex` and using the index `c` we are inserting our data using the struct `Claim` referencing the variables in this function.

This should now compile without any errors!

## Wrap Up
Wo what have we learnt? We have learned about taking inputs from the front-end, including texts, addresses, and balances. We have stored an index number and retrieved it, as well as moving on to the next available number and having it updated in storage. We have created a hash from some of our data and applied all the data to a struct, mapping it to the index number and stored the struct. This is what your module should look like:

	use parity_codec::Encode;
	use srml_support::{StorageValue, StorageMap, dispatch::Result, dispatch::Vec};
	use runtime_primitives::traits::{Hash};
	use system::{self, ensure_signed};
	use balances;
	 
	 
	#[derive(PartialEq, Eq, Clone, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct Claim<Balance, AccountId, Hash, Boolean> {
		pub claimant: AccountId,
		pub respondent: AccountId,
		pub amount: Balance,
		pub doc_hash: Hash,
		pub settled: Boolean,
	}
	 	
	pub type ClaimIndex = u32;
	 
	pub trait Trait: balances::Trait {}
		    
	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {
			
			fn processClaim(
				origin,
				respondent: T::AccountId,
				amount: T::Balance,
				documenttext: Vec<u8>
				) -> Result {
				
				let claimant = ensure_signed(origin)?;
				let amount: T::Balance = amount.into();
				let settled: bool = false;
	 
				let doc_hash: T::Hash = documenttext.using_encoded(<T as system::Trait>::Hashing::hash);
	 
				let c = Self::claims_count();
				// Store the claim counter
				<ClaimsCount<T>>::put(c + 1); 
	 
				// Store claim on chain
				<ClaimsByClaimIndex<T>>::insert(c, Claim { 
					claimant, 
					respondent, 
					amount,
					doc_hash, 
					settled
				});
	 
			
				Ok(())
			}
		}
	}
	 
	decl_storage! {
		trait Store for Module<T: Trait> as Basictest {
			// Simple Claim index storage. Getter returns the latest ClaimIndex Number
			pub ClaimsCount get(claims_count): ClaimIndex;
	 
			// Struct storage mapped to claim index. Getter returns the claim in structured format, also declares the types used in the struct. 
			pub ClaimsByClaimIndex get(claims_by_claim_index): map ClaimIndex => Option<Claim<T::Balance, T::AccountId, T::Hash, bool>>;
		}
	}   

