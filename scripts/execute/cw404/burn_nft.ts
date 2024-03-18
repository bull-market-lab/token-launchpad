import * as fs from "fs";
import { getSigningClient } from "../../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, signingClient } = await getSigningClient();

  const tokenId = 1;

  await signingClient
    .execute(
      signerAddress,
      cw404ContractAddress,
      {
        burn: {
          token_id: tokenId.toString(),
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
