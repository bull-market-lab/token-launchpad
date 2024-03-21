import * as fs from "fs";
import { getSigningClient } from "../../util";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, signingClient } = await getSigningClient();

  await signingClient
    .execute(
      signerAddress,
      launchpadContractAddress,
      {
        create_cw404_collection: {
          royalty_payment_address: signerAddress,
          royalty_percentage: "10",
          max_nft_supply: "1000",
          // e.g. "atom", then base denom is "uatom", 1 ATOM = 1_000_000 uatom, 1 atom = 1 atom NFT
          subdenom: "bad404",
          denom_description: "cw404 experiment",
          denom_name: "Bad 404",
          denom_symbol: "BAD404",
          denom_uri: "dummy.com",
          denom_uri_hash: "dummy_hash",
          mint_groups: [],
        },
      },
      "auto",
      "memooooo",
      [
        {
          denom: "untrn",
          amount: (2_500).toString(),
        },
      ]
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
