import * as fs from "fs";
import { getSigningClient, getQueryClient } from "../../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();
  const { signerAddress, signingClient } = await getSigningClient();
  const sendAmount = 700;
  const recipientAddress = "neutron1d5k3kzd98683zcg6p8ev8h56tg6tk83pry02xx";
  const configResp = await queryClient.queryContractSmart(
    cw404ContractAddress,
    {
      config: {},
    }
  );
  const baseDenom = configResp.config.denom_metadata.base;
  await signingClient
    .sendTokens(
      signerAddress,
      recipientAddress,
      [
        {
          denom: baseDenom,
          amount: sendAmount.toString(),
        },
      ],
      "auto",
      "memooooo"
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
