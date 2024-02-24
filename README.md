# CW404

A [ERC-404](https://github.com/Pandora-Labs-Org/erc404) implementation in CosmWasm. Please see ERC-404 repo and [Pandora docs](https://pandoralabs.mintlify.app/introduction) for introduction to ERC-404.

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
