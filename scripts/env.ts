import "dotenv/config";
import { env } from "process";

export const RPC_ENDPOINT: string = env.RPC_ENDPOINT!;
export const CHAIN_ID: string = env.CHAIN_ID!;
export const CHAIN_PREFIX: string = env.CHAIN_PREFIX!;
export const CHAIN_DENOM: string = env.CHAIN_DENOM!;
export const MNEMONIC: string = env.MNEMONIC!;
