import * as fs from "fs";
import { getQueryClient } from "../../util";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();
  const creator_addr = "neutron1";
  await queryClient
    .queryContractSmart(launchpadContractAddress, {
      coins_by_creator: {
        creator_addr,
      },
    })
    .then((res) => {
      console.log(`tokens: ${JSON.stringify(res, null, 2)}`);
    });
};

run();
