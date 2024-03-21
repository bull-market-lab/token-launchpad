import * as fs from "fs";
import { getQueryClient } from "../../util";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();

  await queryClient
    .queryContractSmart(launchpadContractAddress, {
      coins: {},
    })
    .then((res) => {
      console.log(`tokens: ${JSON.stringify(res, null, 2)}`);
    });
};

run();
