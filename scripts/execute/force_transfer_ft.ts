import * as fs from "fs";
import { getSigningClient } from "../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, siggingClient } = await getSigningClient();

  const sendAmount = 2_500_000;
  const recipientAddress = signerAddress;

  await siggingClient
    .execute(
      signerAddress,
      cw404ContractAddress,
      {
        send_ft: {
          recipient_addr: recipientAddress,
          amount: sendAmount.toString(),
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
