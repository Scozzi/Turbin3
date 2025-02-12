## NFT Marketplace

A simple web3 marketplace for creating and managing NFT listings.

## How to run

1. Clone the repository
2. Run `anchor build`
3. Run `anchor run`

## How to use

1. Run the program
2. Call the `initialize` function with the name of the marketplace and the fee in basis points
3. Call the `listing` function with the to create listing with price of the NFT
4. Call the `delist` function to remove a listing
5. Call the `purchase` (as the taker)function to purchase the NFT