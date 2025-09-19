import { useSolana } from '@/components/solana/use-solana'

export function useQuotamgmtAccountsQueryKey() {
  const { cluster } = useSolana()

  return ['quotamgmt', 'accounts', { cluster }]
}
