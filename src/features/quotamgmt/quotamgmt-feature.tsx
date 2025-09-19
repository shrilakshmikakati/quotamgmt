import { useSolana } from '@/components/solana/use-solana'
import { WalletDropdown } from '@/components/wallet-dropdown'
import { AppHero } from '@/components/app-hero'
import { QuotamgmtUiButtonInitialize } from './ui/quotamgmt-ui-button-initialize'
import { QuotamgmtUiList } from './ui/quotamgmt-ui-list'
import { QuotamgmtUiProgramExplorerLink } from './ui/quotamgmt-ui-program-explorer-link'
import { QuotamgmtUiProgramGuard } from './ui/quotamgmt-ui-program-guard'

export default function QuotamgmtFeature() {
  const { account } = useSolana()

  return (
    <QuotamgmtUiProgramGuard>
      <AppHero
        title="Quotamgmt"
        subtitle={
          account
            ? "Initialize a new quotamgmt onchain by clicking the button. Use the program's methods (increment, decrement, set, and close) to change the state of the account."
            : 'Select a wallet to run the program.'
        }
      >
        <p className="mb-6">
          <QuotamgmtUiProgramExplorerLink />
        </p>
        {account ? (
          <QuotamgmtUiButtonInitialize />
        ) : (
          <div style={{ display: 'inline-block' }}>
            <WalletDropdown />
          </div>
        )}
      </AppHero>
      {account ? <QuotamgmtUiList /> : null}
    </QuotamgmtUiProgramGuard>
  )
}
