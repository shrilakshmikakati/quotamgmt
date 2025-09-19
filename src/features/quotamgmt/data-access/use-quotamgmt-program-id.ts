import { useSolana } from '@/components/solana/use-solana'
import { useMemo } from 'react'
import { getQuotamgmtProgramId } from '@project/anchor'

export function useQuotamgmtProgramId() {
  const { cluster } = useSolana()
  return useMemo(() => getQuotamgmtProgramId(cluster.id), [cluster])
}
