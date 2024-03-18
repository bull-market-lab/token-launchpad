import * as fs from "fs";
import { getSigningClient } from "../../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, signingClient } = await getSigningClient();

  const operator = signerAddress;

  await signingClient
    .execute(
      signerAddress,
      cw404ContractAddress,
      {
        approve_all: {
          operator,
          expires: undefined,
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
