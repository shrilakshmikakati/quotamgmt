import { AppExplorerLink } from '@/components/app-explorer-link'
import { ellipsify } from '@wallet-ui/react'

import { useQuotamgmtProgramId } from '@/features/quotamgmt/data-access/use-quotamgmt-program-id'

export function QuotamgmtUiProgramExplorerLink() {
  const programId = useQuotamgmtProgramId()

  return <AppExplorerLink address={programId.toString()} label={ellipsify(programId.toString())} />
}
