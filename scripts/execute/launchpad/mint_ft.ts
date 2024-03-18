import * as fs from "fs";
import { getSigningClient } from "../../util";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, signingClient } = await getSigningClient();

  const mintAmount = 5_500_000;
  await signingClient
    .execute(
      signerAddress,
      launchpadContractAddress,
      {
        mint_ft: {
          collection_addr:
            "neutron10tyrk0znjhufmv0usk8557wvdtdkzxmpfp7eq848vd7du6eug7vqw48rzc",
          amount: mintAmount.toString(),
          recipient: signerAddress,
          mint_group_name: "everyone",
          merkle_proof: undefined,
        },
      },
      "auto",
      "memooooo",
      [
        {
          denom: "untrn",
          amount: (1_100).toString(),
        },
      ]
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
