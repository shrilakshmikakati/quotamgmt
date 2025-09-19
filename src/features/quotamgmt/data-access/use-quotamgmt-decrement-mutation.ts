import { QuotamgmtAccount, getDecrementInstruction } from '@project/anchor'
import { useMutation } from '@tanstack/react-query'
import { useWalletUiSigner } from '@/components/solana/use-wallet-ui-signer'
import { useWalletTransactionSignAndSend } from '@/components/solana/use-wallet-transaction-sign-and-send'
import { toastTx } from '@/components/toast-tx'
import { useQuotamgmtAccountsInvalidate } from './use-quotamgmt-accounts-invalidate'

export function useQuotamgmtDecrementMutation({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const invalidateAccounts = useQuotamgmtAccountsInvalidate()
  const signer = useWalletUiSigner()
  const signAndSend = useWalletTransactionSignAndSend()

  return useMutation({
    mutationFn: async () => await signAndSend(getDecrementInstruction({ quotamgmt: quotamgmt.address }), signer),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}
