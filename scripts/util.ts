import {
  CosmWasmClient,
  SigningCosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";

import { CHAIN_DENOM, CHAIN_PREFIX, RPC_ENDPOINT } from "./env";

export const getSigningClient = async () => {
  const mnemonic = process.env.MNEMONIC!;
  const offlineSigner = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: CHAIN_PREFIX,
  });

  const accounts = await offlineSigner.getAccounts();
  const signerAddress = accounts[0]!.address;
  const siggingClient = await SigningCosmWasmClient.connectWithSigner(
    RPC_ENDPOINT,
    offlineSigner,
    {
      gasPrice: GasPrice.fromString(`0.04${CHAIN_DENOM}`),
    }
  );
  return { signerAddress, siggingClient };
};

export const getQueryClient = async () => {
  return await CosmWasmClient.connect(RPC_ENDPOINT);
};
