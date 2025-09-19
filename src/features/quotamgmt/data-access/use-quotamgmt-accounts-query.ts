import { useSolana } from '@/components/solana/use-solana'
import { useQuery } from '@tanstack/react-query'
import { getQuotamgmtProgramAccounts } from '@project/anchor'
import { useQuotamgmtAccountsQueryKey } from './use-quotamgmt-accounts-query-key'

export function useQuotamgmtAccountsQuery() {
  const { client } = useSolana()

  return useQuery({
    queryKey: useQuotamgmtAccountsQueryKey(),
    queryFn: async () => await getQuotamgmtProgramAccounts(client.rpc),
  })
}
