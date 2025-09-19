import { QuotamgmtAccount } from '@project/anchor'
import { Button } from '@/components/ui/button'

import { useQuotamgmtSetMutation } from '@/features/quotamgmt/data-access/use-quotamgmt-set-mutation'

export function QuotamgmtUiButtonSet({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const setMutation = useQuotamgmtSetMutation({ quotamgmt })

  return (
    <Button
      variant="outline"
      onClick={() => {
        const value = window.prompt('Set value to:', quotamgmt.data.count.toString() ?? '0')
        if (!value || parseInt(value) === quotamgmt.data.count || isNaN(parseInt(value))) {
          return
        }
        return setMutation.mutateAsync(parseInt(value))
      }}
      disabled={setMutation.isPending}
    >
      Set
    </Button>
  )
}
