import BN from "bn.js";
import { PublicKey } from '@solana/web3.js';

export interface Treasure {
  admin: PublicKey;
  name:String;
  uri:String;
  symbol:String;
  supply: BN,
  mints: BN;
  solFee: BN;
}