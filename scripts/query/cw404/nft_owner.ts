import * as fs from "fs";
import { getQueryClient } from "../../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();

  const tokenId = 1;

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      owner_of: {
        token_id: tokenId.toString(),
        include_expired: true,
      },
    })
    .then((res) => {
      console.log(`NFT token id ${tokenId}'s owner is ${JSON.stringify(res)}`);
    });
};

run();
