import { QuotamgmtAccount } from '@project/anchor'
import { ellipsify } from '@wallet-ui/react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { QuotamgmtUiButtonClose } from './quotamgmt-ui-button-close'
import { QuotamgmtUiButtonDecrement } from './quotamgmt-ui-button-decrement'
import { QuotamgmtUiButtonIncrement } from './quotamgmt-ui-button-increment'
import { QuotamgmtUiButtonSet } from './quotamgmt-ui-button-set'

export function QuotamgmtUiCard({ quotamgmt }: { quotamgmt: QuotamgmtAccount }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Quotamgmt: {quotamgmt.data.count}</CardTitle>
        <CardDescription>
          Account: <AppExplorerLink address={quotamgmt.address} label={ellipsify(quotamgmt.address)} />
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex gap-4 justify-evenly">
          <QuotamgmtUiButtonIncrement quotamgmt={quotamgmt} />
          <QuotamgmtUiButtonSet quotamgmt={quotamgmt} />
          <QuotamgmtUiButtonDecrement quotamgmt={quotamgmt} />
          <QuotamgmtUiButtonClose quotamgmt={quotamgmt} />
        </div>
      </CardContent>
    </Card>
  )
}
