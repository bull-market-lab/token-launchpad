import * as fs from "fs";
import { getSigningClient } from "../../util";
import { CHAIN_DENOM } from "../../env";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, signingClient } = await getSigningClient();
  // 1_000_000_000 is 1 billion, this is in denom not base denom, we convert to base denom in the contract
  const totalSupply = 1_000_000_000;
  // this is in base denom, e.g. 1_000 untrn
  // outcome will be 1_000 untrn paired with totalSupply denom or 10^6 * totalSupply base denom
  const initialNtrnLiquidity = 3_000;
  await signingClient
    .execute(
      signerAddress,
      launchpadContractAddress,
      {
        create_coin: {
          immutable: true,
          initial_supply_in_denom: totalSupply.toString(),
          max_supply_in_denom: totalSupply.toString(),
          // e.g. "atom", then base denom is "uatom", 1 ATOM = 1_000_000 uatom
          subdenom: "meme100",
          denom_description: "meme experiment 100",
          denom_name: "Meme100",
          denom_symbol: "MEME100",
          denom_uri: "dummy.com",
          denom_uri_hash: "dummy_hash",
        },
      },
      "auto",
      "launch tokennnnnn",
      [
        {
          denom: CHAIN_DENOM,
          amount: initialNtrnLiquidity.toString(),
        },
      ]
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
