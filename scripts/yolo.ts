import * as fs from "fs";

import { getSigningClient, getQueryClient } from "./util";
import {
  CosmWasmClient,
  SigningCosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";

const printBalance = async (
  queryClient: CosmWasmClient,
  cw404ContractAddress: string,
  signerAddress: string
) => {
  await queryClient
    .queryContractSmart(cw404ContractAddress, {
      balance: {
        owner: signerAddress,
      },
    })
    .then((res) => {
      console.log("admin balance", JSON.stringify(res));
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
};

const mintFt = async (
  siggingClient: SigningCosmWasmClient,
  cw404ContractAddress: string,
  signerAddress: string,
  amount: string
) => {
  await siggingClient
    .execute(
      signerAddress,
      cw404ContractAddress,
      {
        mint_ft: {
          amount,
        },
      },
      "auto",
      "memooooo",
      []
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

const contractSend = async (
  siggingClient: SigningCosmWasmClient,
  cw404ContractAddress: string,
  signerAddress: string,
  recipientAddress: string,
  amount: string
) => {
  await siggingClient
    .execute(
      signerAddress,
      cw404ContractAddress,
      {
        send_ft: {
          recipient_addr: recipientAddress,
          amount,
        },
      },
      "auto",
      "memooooo",
      []
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

const run = async () => {
  const { cw404ContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, siggingClient } = await getSigningClient();
  const queryClient = await getQueryClient();

  // printBalance(queryClient, cw404ContractAddress, signerAddress);

  const mintAmount = 2_500_000;

  // mintFt(
  //   siggingClient,
  //   cw404ContractAddress,
  //   signerAddress,
  //   mintAmount.toString()
  // );

  const sendAmount = 900_000;

  // contractSend(
  //   siggingClient,
  //   cw404ContractAddress,
  //   signerAddress,
  //   signerAddress,
  //   sendAmount.toString()
  // );

  printBalance(queryClient, cw404ContractAddress, signerAddress);
};

run();

// factory/neutron1xdtwh5jr4zjx8g3zh29jud75c666wua7tsmum3ajm6ylf782etfs60dj2h/wstETH
// {
//   "query_denom": {
//     "denom": "factory/neutron1xdtwh5jr4zjx8g3zh29jud75c666wua7tsmum3ajm6ylf782etfs60dj2h/wstETH"
//   }
// }
