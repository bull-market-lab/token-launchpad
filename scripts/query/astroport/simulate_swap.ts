import * as fs from "fs";
import { getQueryClient } from "../../util";
import { CHAIN_DENOM } from "../../env";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();

  const token = await queryClient.queryContractSmart(launchpadContractAddress, {
    token: {
      subdenom: "meme3",
    },
  });
  const poolAddress: string =
    token.token_info.astroport_pair_info.contract_addr;
  const lpTokenAddress = token.token_info.astroport_pair_info.liquidity_token;
  console.log("poolAddress", poolAddress);
  console.log("lpTokenAddress", lpTokenAddress);

  const swappedAmount = 100_000;
  await queryClient
    .queryContractSmart(poolAddress, {
      simulation: {
        offer_asset: {
          info: {
            native_token: {
              denom: CHAIN_DENOM,
            },
          },
          amount: swappedAmount.toString(),
        },
      },
    })
    .then((res) => {
      console.log("simulated swap res", res);
    });
};

run();
