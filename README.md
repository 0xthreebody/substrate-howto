**ORIGINAL:** <https://gitlab.com/chrisdcosta/basictest/blob/master/README.md>  


## The story so far

I've set a substrate node using
	
	substrate-node-new basictest-node meek

I built the demo and ran it, it all works fine, but I've removed from lib.rs

Now I'm attempting to build a simple claims module, but I can't get this to compile.

I am pretty sure I have multiple errors here, but the code intention should be more-or-less
self-explanatory I have not written it correctly. I'm very new to rust.

To try to figure this out I have commented the claims struct and other elements leaving only the Index struct related to `AccountsByClaimIndex` storage but I intend to work through the issues.


There are a few of issues I really am not sure about:
the two `type` definitions... `ClaimIndex`, `HashRef` 
- are they necessary and done correctly? 
- do they make sense in context?

The two `structs` that I have declared are not declared properly either. I cannot figure out how to do this.
I wanted to use the structs be able to store a number of values.

- I have presumed that for a `struct` the square bracket lists the types used by the struct elements. Is this correct?

For example in "Claim" struct I wanted to use an "Address" type but when I used `balances::address::Address` this seems to imply both an `AccountId` and `AccountIndex`
are available/required. Also this gives me problems with the storage declaration. Honestly I am at a loss.  

I have tried to refer to the reference implementations directly in the SRML folder in substrate, but I am 
clearly missing something.


So - specific advice needed: 
- How do I store a values using a `struct`
- How do I work with `addresses` in a struct (or an appropriate reference that deals with the public address) other than `origin` (which already has `T::Origin`)
- Am I missing something when working with structs (`impl` or `traits` that I haven't yet understood)  
- anything special I need to know about dealing with a hashed value in a `struct`
- maybe structs are not how you do any of this!!?


