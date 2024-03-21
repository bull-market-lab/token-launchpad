# Token Launchpad

This token launchpad contract supports 2 token standard

- Cosmos SDK native coin backed by token factory module.
- CW404.

## CW404

An [ERC-404](https://github.com/Pandora-Labs-Org/erc404) implementation in CosmWasm. Please see ERC-404 repo and [Pandora docs](https://pandoralabs.mintlify.app/introduction) for introduction to ERC-404.

TLDR: CW404 is a new token standard for Cosmos chains. It enables native NFT fragmentation and it complies to Cosmos native token standard and CW721 NFT standard.

Let's look at an example, say we create a collection called `Bad NFT`, ticker is `BNFT`. The NFT supply is 1000, and each NFT equals to 1000 FT, FT ticker is `uBNFT`. When Bob has 1000 `uBNFT`, he also has 1 `BNFT`, i.e. 1 NFT.

Bob can list his 1 `BNFT` on an NFT marketplace like Stargaze, just like any regular NFT (e.g. Bad Kids).
If Bob sends 1000 `uBNFT` to Alice, both his `uBNFT` and `BNFT` balance become 0, and alice balance becomes 1000 `uBNFT` and 1 `BNFT`.
If Bob sends 500 `uBNFT` to Alice, his `uBNFT` and `BNFT` balance become 500 and 0, essentially he burned hist NFT. Alice's `uBNFT` and `BNFT` balance become 500 and 0. Total supply of NFT is reduced by one. If Bob sends his rest of 500 `uBNFT` to Alice, Alice's `uBNFT` and `BNFT` balance become 1000 and 1, an NFT is automatically minted.

How does 404 differentiates from NFT fractiaonalization app before?

|      | Existing NFT fractionalization app                                                                                                                                        | ERC-404 / CW404                                                                                                                                                                                   |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pros | Built for existing NFT, so you can fractionalize blue chip NFTs like Bad Kids.                                                                                            | Fractionalization baked into the NFT contract so it happens automatically when FT is transferred, mint and burned.                                                                                |
| Cons | Fractionalization happens in external contract, usually NFT holders deposit to some vault, i.e. multiple steps to use. And user needs to trust the fractionalization app. | 1. Need to bootstrap the new standard. Code is new, there could be bugs. 2. When you burn NFT and remint later, you won't get the same NFT, there could be workaround, but needs more exploration |

## Cosmos SDK native coin

This launchpad contract allows anyone to create native Cosmos SDK coin backed by token factory module and create a pool on [Astroport](https://astroport.fi/) with some seed liquidity in 1 transaction. It also serves as a registry for all the coins created.

It should be compatible with any Cosmos SDK chains with token factory module. Update `store_code_and_init.ts` script and `.env` to deploy to your preferred chain.

See `scripts` in `package.json` for all examples.

You can fork the contract and add a creation fee so you earn money when people create coins through your contract.

### Note

To make the token launch fairer, this contract will mint all the supply and deposit all the supply to the Astroport pool, creator has the option to make the coin immutable so there will be no more mint in the future.

The impact of depositing all supply to Astroport is at the beginning the pool could be filled with create token and very little paired token (e.g. NTRN). Astroport pool has a max slippage of 50%, say if you create a token called MEME, paired with 1 NTRN as seed liquidity. After the pool is created, people can only buy up to 1 NTRN of MEME, then up to 2 NTRN, then 4, then 8, etc. If they try to buy 2 NTRN at the beginning, the slippage would be be 66.666% which exceeds the max slippage, causing swap to fail. So as token creator, if you want the token to be more buyable, it's recommended to provide more seed liquidity in paired token.

### Warning

Contract hasn't been audited, use at your own risk.
