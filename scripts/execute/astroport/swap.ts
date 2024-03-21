import * as fs from "fs";
import { getQueryClient, getSigningClient } from "../../util";
import { CHAIN_DENOM } from "../../env";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();
  const { signerAddress, signingClient } = await getSigningClient();

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

  const simulateRes = await queryClient
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
      return res;
    });
  const maxSpread: string = simulateRes.spread_amount;
// 5000000000000000;
// 9090909090910;
  await signingClient
    .execute(
      signerAddress,
      poolAddress,
      {
        swap: {
          offer_asset: {
            info: {
              native_token: {
                denom: CHAIN_DENOM,
              },
            },
            amount: swappedAmount.toString(),
          },
          //   belief_price: "123",
          // need to set this extremely high cause initially the pool is almost all created token and almost no NTRN
          // currently set to the max spread allowed by astroport which is 50%
          max_spread: "0.5",
          //   to: "terra...",
        },
      },

      "auto",
      "launch tokennnnnn",
      [
        {
          denom: CHAIN_DENOM,
          amount: swappedAmount.toString(),
        },
      ]
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
