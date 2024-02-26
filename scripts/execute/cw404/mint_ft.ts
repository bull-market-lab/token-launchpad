import * as fs from "fs";
import { getSigningClient } from "../../util";

const run = async () => {
  // const { cw404ContractAddress } = JSON.parse(
  //   fs.readFileSync("scripts/contract_addresses.json").toString()
  // );
  const { signerAddress, siggingClient } = await getSigningClient();

  const mintAmount = 5_500_000;
  await siggingClient
    .execute(
      signerAddress,
      // cw404ContractAddress,
      "neutron10tyrk0znjhufmv0usk8557wvdtdkzxmpfp7eq848vd7du6eug7vqw48rzc",
      {
        mint_ft: {
          amount: mintAmount.toString(),
          recipient: signerAddress,
          mint_group_name: "everyone",
          merkle_proof: undefined,
        },
      },
      "auto",
      "memooooo",
      []
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
