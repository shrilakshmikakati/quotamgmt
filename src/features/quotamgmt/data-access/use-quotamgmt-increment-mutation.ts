import { QuotamgmtAccount, getIncrementInstruction } from '@project/anchor'
import { useMutation } from '@tanstack/react-query'
import { toastTx } from '@/components/toast-tx'
import { useWalletUiSigner } from '@/components/solana/use-wallet-ui-signer'
import { useWalletTransactionSignAndSend } from '@/components/solana/use-wallet-transaction-sign-and-send'
import { useQuotamgmtAccountsInvalidate } from './use-quotamgmt-accounts-invalidate'

export function useQuotamgmtIncrementMutation({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const invalidateAccounts = useQuotamgmtAccountsInvalidate()
  const signAndSend = useWalletTransactionSignAndSend()
  const signer = useWalletUiSigner()

  return useMutation({
    mutationFn: async () => await signAndSend(getIncrementInstruction({ quotamgmt: quotamgmt.address }), signer),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}
