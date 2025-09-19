import { ReactNode } from 'react'

import { useQuotamgmtProgram } from '@/features/quotamgmt/data-access/use-quotamgmt-program'

export function QuotamgmtUiProgramGuard({ children }: { children: ReactNode }) {
  const programAccountQuery = useQuotamgmtProgram()

  if (programAccountQuery.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }

  if (!programAccountQuery.data?.value) {
    return (
      <div className="alert alert-info flex justify-center">
        <span>Program account not found. Make sure you have deployed the program and are on the correct cluster.</span>
      </div>
    )
  }

  return children
}
