import { useQueryClient } from '@tanstack/react-query'
import { useQuotamgmtAccountsQueryKey } from './use-quotamgmt-accounts-query-key'

export function useQuotamgmtAccountsInvalidate() {
  const queryClient = useQueryClient()
  const queryKey = useQuotamgmtAccountsQueryKey()

  return () => queryClient.invalidateQueries({ queryKey })
}
