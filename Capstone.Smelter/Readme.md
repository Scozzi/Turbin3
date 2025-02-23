# Capstone.Smelter - Compressed NFT System

A solana program for minting and managing compressed NFTs (cNFTs) with advanced features including weight influced randomization and crafting mechanics.

## Features

- Compressed NFT (cNFT) minting system
- State-of-the-art Merkle tree management
- Weighted randomization system for NFT distribution
- Crafting/burning mechanism for NFT transformation
- Gas-efficient operations using Solana's Token Program for minting, transferring, and burning cNFTs

## Technical Stack

- Anchor
- Metaplex 
- State Compression (for cNFTs)
- Merkle Tree Implementation

## Getting Started

1. Clone the repository
2. Install dependencies:
   ```bash
   yarn install
   ```
3. Build the program:
   ```bash
   anchor build
   ```
4. Deploy to localnet:
   ```bash
   anchor deploy
   ```
5. Run tests:
   ```bash
   anchor test
   ```

## Project Structure

- `programs/` - Contains the main Solana program logic
- `tests/` - Test suite
- `app/` - Frontend application (coming soon)

## Development Roadmap

1. [x] Project initialization
2. [ ] Basic cNFT minting implementation
3. [ ] Merkle tree management system
4. [ ] Weighted randomization system
5. [ ] Crafting/burning mechanism
6. [ ] Frontend integration

## License

MIT
