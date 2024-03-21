import * as fs from "fs";
import { getQueryClient } from "../../util";

const run = async () => {
  const { launchpadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const queryClient = await getQueryClient();
  const contract_addr = "neutron1";

  await queryClient
    .queryContractSmart(launchpadContractAddress, {
      cw404_collection_by_contract: {
        contract_addr,
      },
    })
    .then((res) => {
      console.log(`token: ${JSON.stringify(res, null, 2)}`);
    });
};

run();
