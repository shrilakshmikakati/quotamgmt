import { QuotamgmtAccount } from '@project/anchor'
import { Button } from '@/components/ui/button'
import { useQuotamgmtIncrementMutation } from '../data-access/use-quotamgmt-increment-mutation'

export function QuotamgmtUiButtonIncrement({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const incrementMutation = useQuotamgmtIncrementMutation({ quotamgmt })

  return (
    <Button variant="outline" onClick={() => incrementMutation.mutateAsync()} disabled={incrementMutation.isPending}>
      Increment
    </Button>
  )
}
