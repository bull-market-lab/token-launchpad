import * as fs from "fs";
import { getQueryClient, getSigningClient } from "./util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      recycled_token_ids: {},
    })
    .then((res) => {
      console.log(`recycled NFT token ids: ${JSON.stringify(res)}`);
    });
};

run();
