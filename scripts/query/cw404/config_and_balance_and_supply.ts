import * as fs from "fs";
import { getSigningClient, getQueryClient } from "../../util";

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress } = await getSigningClient();
  const queryClient = await getQueryClient();

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      config: {},
    })
    .then((res) => {
      console.log("config", JSON.stringify(res));
    });

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      supply: {},
    })
    .then((res) => {
      console.log("supply", JSON.stringify(res));
    });

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      balance: {
        owner: cw404ContractAddress,
      },
    })
    .then((res) => {
      console.log("cw404 contract balance", JSON.stringify(res));
    });

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      balance: {
        owner: signerAddress,
      },
    })
    .then((res) => {
      console.log("eoa admin balance", JSON.stringify(res));
    });

  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      balance: {
        owner: "neutron1d5k3kzd98683zcg6p8ev8h56tg6tk83pry02xx",
      },
    })
    .then((res) => {
      console.log("eoa2 balance", JSON.stringify(res));
    });
};

run();
