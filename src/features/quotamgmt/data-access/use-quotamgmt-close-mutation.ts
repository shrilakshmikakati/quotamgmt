import { QuotamgmtAccount, getCloseInstruction } from '@project/anchor'
import { useMutation } from '@tanstack/react-query'
import { useWalletTransactionSignAndSend } from '@/components/solana/use-wallet-transaction-sign-and-send'
import { useWalletUiSigner } from '@/components/solana/use-wallet-ui-signer'
import { toastTx } from '@/components/toast-tx'
import { useQuotamgmtAccountsInvalidate } from './use-quotamgmt-accounts-invalidate'

export function useQuotamgmtCloseMutation({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  const invalidateAccounts = useQuotamgmtAccountsInvalidate()
  const signAndSend = useWalletTransactionSignAndSend()
  const signer = useWalletUiSigner()

  return useMutation({
    mutationFn: async () => {
      return await signAndSend(getCloseInstruction({ payer: signer, quotamgmt: quotamgmt.address }), signer)
    },
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}
