import * as fs from "fs";
import { getQueryClient } from "../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      num_tokens: {},
    })
    .then((res) => {
      console.log("NFT num tokens", JSON.stringify(res));
    });
};

run();
