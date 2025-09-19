import { QuotamgmtAccount } from '@project/anchor'
import { Button } from '@/components/ui/button'

import { useQuotamgmtCloseMutation } from '@/features/quotamgmt/data-access/use-quotamgmt-close-mutation'

export function QuotamgmtUiButtonClose({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const closeMutation = useQuotamgmtCloseMutation({ quotamgmt })

  return (
    <Button
      variant="destructive"
      onClick={() => {
        if (!window.confirm('Are you sure you want to close this account?')) {
          return
        }
        return closeMutation.mutateAsync()
      }}
      disabled={closeMutation.isPending}
    >
      Close
    </Button>
  )
}
