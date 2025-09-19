import { Button } from '@/components/ui/button'

import { useQuotamgmtInitializeMutation } from '@/features/quotamgmt/data-access/use-quotamgmt-initialize-mutation'

export function QuotamgmtUiButtonInitialize() {
  const mutationInitialize = useQuotamgmtInitializeMutation()

  return (
    <Button onClick={() => mutationInitialize.mutateAsync()} disabled={mutationInitialize.isPending}>
      Initialize Quotamgmt {mutationInitialize.isPending && '...'}
    </Button>
  )
}
