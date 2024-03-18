import * as fs from "fs";
import { getSigningClient } from "../../util";
import { toBinary } from "@cosmjs/cosmwasm-stargate";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, signingClient } = await getSigningClient();

  const tokenId = 1;
  const receipientContractAddress =
    "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";

  await signingClient
    .execute(
      signerAddress,
      cw404ContractAddress,
      {
        send_nft: {
          contract: receipientContractAddress,
          token_id: tokenId.toString(),
          msg: toBinary("hello"),
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
