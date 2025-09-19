import { QuotamgmtAccount } from '@project/anchor'
import { Button } from '@/components/ui/button'

import { useQuotamgmtDecrementMutation } from '../data-access/use-quotamgmt-decrement-mutation'

export function QuotamgmtUiButtonDecrement({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const decrementMutation = useQuotamgmtDecrementMutation({ quotamgmt })

  return (
    <Button variant="outline" onClick={() => decrementMutation.mutateAsync()} disabled={decrementMutation.isPending}>
      Decrement
    </Button>
  )
}
