import { QuotamgmtUiCard } from './quotamgmt-ui-card'
import { useQuotamgmtAccountsQuery } from '@/features/quotamgmt/data-access/use-quotamgmt-accounts-query'

export function QuotamgmtUiList() {
  const quotamgmtAccountsQuery = useQuotamgmtAccountsQuery()

  if (quotamgmtAccountsQuery.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }

  if (!quotamgmtAccountsQuery.data?.length) {
    return (
      <div className="text-center">
        <h2 className={'text-2xl'}>No accounts</h2>
        No accounts found. Initialize one to get started.
      </div>
    )
  }

  return (
    <div className="grid lg:grid-cols-2 gap-4">
      {quotamgmtAccountsQuery.data?.map((quotamgmt) => (
        <QuotamgmtUiCard key={quotamgmt.address} quotamgmt={quotamgmt} />
      ))}
    </div>
  )
}
