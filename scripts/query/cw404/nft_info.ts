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
      nft_info: {
        token_id: tokenId.toString(),
      },
    })
    .then((res) => {
      console.log(
        `NFT token id ${tokenId}'s token info ${JSON.stringify(res)}`
      );
    });
};

run();
