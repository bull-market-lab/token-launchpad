import * as fs from "fs";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

import { getSigningClient } from "./util";

const storeCode = async (
  signerAddress: string,
  signingClient: SigningCosmWasmClient
) => {
  const wasmCodeDirectory = "artifacts/";
  const cw404 = wasmCodeDirectory + "cw404_base-aarch64.wasm";

  const cw404CodeId = (
    await signingClient.upload(
      signerAddress,
      fs.readFileSync(cw404),
      "auto",
      "cw404 test"
    )
  ).codeId;
  console.log("cw404 codeId", cw404CodeId);
  await new Promise((resolve) => setTimeout(resolve, 5000));

  fs.writeFileSync(
    "scripts/code_ids.json",
    JSON.stringify({
      cw404CodeId,
    })
  );

  return {
    cw404CodeId,
  };
};

const init = async (
  signerAddress: string,
  signingClient: SigningCosmWasmClient,
  { cw404CodeId }
) => {
  const cw404ContractAddress = (
    await signingClient.instantiate(
      signerAddress,
      cw404CodeId,
      {
        admin: signerAddress,
        minter: signerAddress,
        royalty_payment_address: signerAddress,
        royalty_percentage: "10",
        max_nft_supply: "1000",
        // e.g. "atom", then base denom is "uatom", 1 ATOM = 1_000_000 uatom, 1 atom = 1 atom NFT
        subdenom: "bad404",
        denom_description: "cw404 experiment",
        denom_name: "Bad 404",
        denom_symbol: "BAD404",
        denom_uri: "dummy.com",
        denom_uri_hash: "dummy_hash",
      },
      "cw404",
      "auto",
      {
        admin: signerAddress,
      }
    )
  ).contractAddress;
  console.log("cw404 contract address", cw404ContractAddress);

  fs.writeFileSync(
    "scripts/contract_addresses.json",
    JSON.stringify({
      cw404ContractAddress,
    })
  );
};

const run = async () => {
  const { signerAddress, siggingClient } = await getSigningClient();
  const { cw404CodeId } = await storeCode(signerAddress, siggingClient);

  await init(signerAddress, siggingClient, {
    cw404CodeId,
  });
};

run();
