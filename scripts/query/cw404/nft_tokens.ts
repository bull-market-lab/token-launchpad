import * as fs from "fs";
import { getQueryClient, getSigningClient } from "../../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress } = await getSigningClient();
  const queryClient = await getQueryClient();

  const tokenId = 1;

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      tokens: {
        owner: signerAddress,
      },
    })
    .then((res) => {
      console.log(
        `NFT token id ${tokenId}'s token info ${JSON.stringify(res)}`
      );
    });
};

run();
