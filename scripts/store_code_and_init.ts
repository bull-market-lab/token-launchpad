import * as fs from "fs";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

import { getSigningClient } from "./util";

const storeCode = async (
  signerAddress: string,
  signingClient: SigningCosmWasmClient
) => {
  const wasmCodeDirectory = "artifacts/";
  const cw404 = wasmCodeDirectory + "cw404_base.wasm";
  const coin = wasmCodeDirectory + "coin_base.wasm";
  const launchpad = wasmCodeDirectory + "launchpad.wasm";

  const cw404CodeId = (
    await signingClient.upload(
      signerAddress,
      fs.readFileSync(cw404),
      "auto",
      "cw404"
    )
  ).codeId;
  console.log("cw404 codeId", cw404CodeId);
  await new Promise((resolve) => setTimeout(resolve, 5000));

  const coinCodeId = (
    await signingClient.upload(
      signerAddress,
      fs.readFileSync(coin),
      "auto",
      "coin"
    )
  ).codeId;
  console.log("coin codeId", coinCodeId);
  await new Promise((resolve) => setTimeout(resolve, 5000));

  const launchpadCodeId = (
    await signingClient.upload(
      signerAddress,
      fs.readFileSync(launchpad),
      "auto",
      "launchpad"
    )
  ).codeId;
  console.log("launchpad codeId", launchpadCodeId);

  fs.writeFileSync(
    "scripts/code_ids.json",
    JSON.stringify({
      cw404CodeId,
      coinCodeId,
      launchpadCodeId,
    })
  );

  return {
    cw404CodeId,
    coinCodeId,
    launchpadCodeId,
  };
};

const init = async (
  signerAddress: string,
  signingClient: SigningCosmWasmClient,
  { cw404CodeId, coinCodeId, launchpadCodeId }
) => {
  const astroportFactoryAddrOnTestnet =
    "neutron1jj0scx400pswhpjes589aujlqagxgcztw04srynmhf0f6zplzn2qqmhwj7";
  const astroportFactoryAddrOnMainnet =
    "neutron1hptk0k5kng7hjy35vmh009qd5m6l33609nypgf2yc6nqnewduqasxplt4e";

  const launchpadContractAddress = (
    await signingClient.instantiate(
      signerAddress,
      launchpadCodeId,
      {
        admin_addr: signerAddress,
        cw404_fee_collector: signerAddress,
        cw404_code_id: cw404CodeId.toString(),
        cw404_collection_creation_fee: (2_500).toString(),
        cw404_mint_fee: (1_000).toString(),
        astroport_factory_addr: astroportFactoryAddrOnTestnet,
        coin_fee_collector: signerAddress,
        coin_code_id: coinCodeId.toString(),
        coin_creation_fee: (2_500).toString(),
      },
      "token-launchpad",
      "auto",
      {
        admin: signerAddress,
      }
    )
  ).contractAddress;
  console.log("launchpad address", launchpadContractAddress);

  fs.writeFileSync(
    "scripts/contract_addresses.json",
    JSON.stringify({
      launchpadContractAddress,
    })
  );
};

const run = async () => {
  const { signerAddress, signingClient } = await getSigningClient();
  const { cw404CodeId, coinCodeId, launchpadCodeId } = await storeCode(
    signerAddress,
    signingClient
  );

  await init(signerAddress, signingClient, {
    // cw404CodeId: 3538,
    // coinCodeId: 3539,
    // launchpadCodeId: 3540,
    cw404CodeId,
    coinCodeId,
    launchpadCodeId,
  });
};

run();
