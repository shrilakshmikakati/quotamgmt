// Here we export some useful types and functions for interacting with the Anchor program.
import { Account, address, getBase58Decoder, SolanaClient } from 'gill'
import { SolanaClusterId } from '@wallet-ui/react'
import { getProgramAccountsDecoded } from './helpers/get-program-accounts-decoded'
import { Quotamgmt, QUOTAMGMT_DISCRIMINATOR, QUOTAMGMT_PROGRAM_ADDRESS, getQuotamgmtDecoder } from './client/js'
import QuotamgmtIDL from '../target/idl/quotamgmt.json'

export type QuotamgmtAccount = Account<Quotamgmt, string>

// Re-export the generated IDL and type
export { QuotamgmtIDL }

// This is a helper function to get the program ID for the Quotamgmt program depending on the cluster.
export function getQuotamgmtProgramId(cluster: SolanaClusterId) {
  switch (cluster) {
    case 'solana:devnet':
    case 'solana:testnet':
      // This is the program ID for the Quotamgmt program on devnet and testnet.
      return address('Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe')
    case 'solana:mainnet':
    default:
      return QUOTAMGMT_PROGRAM_ADDRESS
  }
}

export * from './client/js'

export function getQuotamgmtProgramAccounts(rpc: SolanaClient['rpc']) {
  return getProgramAccountsDecoded(rpc, {
    decoder: getQuotamgmtDecoder(),
    filter: getBase58Decoder().decode(QUOTAMGMT_DISCRIMINATOR),
    programAddress: QUOTAMGMT_PROGRAM_ADDRESS,
  })
}
