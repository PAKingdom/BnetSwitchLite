import {
  ExternalLink,
  FolderCog,
  RefreshCw,
  UserRound,
  UserRoundPlus,
  X,
} from "lucide-react"

import { AccountRow } from "@/components/account-row"
import { OperationBar } from "@/components/operation-bar"
import { Button } from "@/components/ui/button"
import {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty"
import { Separator } from "@/components/ui/separator"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import type {
  AccountSnapshot,
  LoginSessionSnapshot,
  OperationEvent,
} from "@/lib/types"
import { accountKeysEqual } from "@/lib/types"

interface AccountListProps {
  accounts: AccountSnapshot[]
  loginSession: LoginSessionSnapshot | null
  busy: boolean
  canCancelLogin: boolean
  onRefresh: () => void
  onConfigurePath: () => void
  onAddAccount: () => void
  onOpenClient: () => void
  onSwitch: (account: AccountSnapshot) => void
  onRelogin: (account: AccountSnapshot) => void
  onDelete: (account: AccountSnapshot) => void
  onCancelLogin: () => void
  operation: OperationEvent | null
}

export function AccountList({
  accounts,
  loginSession,
  busy,
  canCancelLogin,
  onRefresh,
  onConfigurePath,
  onAddAccount,
  onOpenClient,
  onSwitch,
  onRelogin,
  onDelete,
  onCancelLogin,
  operation,
}: AccountListProps) {
  return (
    <section
      aria-label="账号列表"
      className="flex min-h-0 min-w-0 flex-1 flex-col bg-card"
      data-window-content
    >
      <div
        className="flex min-h-0 flex-1 flex-col overflow-hidden bg-muted/20 p-2"
        data-account-rows
      >
        <div
          className="relative my-auto h-fit min-h-max w-full flex-none overflow-hidden rounded-lg border bg-card"
          data-account-rows-content
        >
          {accounts.length === 0 ? (
            <Empty className="h-16 max-h-16 min-h-16 p-2">
              <EmptyHeader className="max-w-none flex-row gap-2">
                <EmptyMedia className="mb-0 text-muted-foreground">
                  <UserRound className="size-[18px]" />
                </EmptyMedia>
                <EmptyTitle>
                  {loginSession?.intent.kind === "add"
                    ? "等待战网登录"
                    : "未发现账号"}
                </EmptyTitle>
              </EmptyHeader>
            </Empty>
          ) : (
            <div className="divide-y" role="list">
              {accounts.map((account) => {
                const loginPending =
                  loginSession !== null &&
                  loginSession.intent.kind === "reauthenticate" &&
                  accountKeysEqual(account.key, loginSession.intent.accountKey)

                return (
                  <div
                    data-account-row-item
                    key={`${account.key.environment}:${account.key.accountId}`}
                    role="listitem"
                  >
                    <AccountRow
                      account={account}
                      busy={busy || loginSession !== null}
                      cancelLoginDisabled={!canCancelLogin}
                      loginPending={loginPending}
                      onCancelLogin={onCancelLogin}
                      onDelete={() => onDelete(account)}
                      onRelogin={() => onRelogin(account)}
                      onSwitch={() => onSwitch(account)}
                    />
                  </div>
                )
              })}
            </div>
          )}
        </div>
      </div>

      <Separator data-account-separator />
      <div
        className="account-list-toolbar relative grid min-h-10 shrink-0 grid-cols-[1fr_auto_1fr] items-center gap-2 bg-muted/35 px-2"
        data-account-toolbar
      >
        <div className="col-start-1 flex items-center gap-0.5 justify-self-start">
          {loginSession?.intent.kind === "add" ? (
            <Button
              disabled={!canCancelLogin}
              onClick={onCancelLogin}
              size="sm"
              variant="quiet"
            >
              <X data-icon="inline-start" />
              取消登录
            </Button>
          ) : (
            <Button
              disabled={busy || loginSession !== null}
              onClick={onAddAccount}
              size="sm"
              variant="quiet"
            >
              <UserRoundPlus data-icon="inline-start" />
              添加账号
            </Button>
          )}
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  disabled={busy || loginSession !== null}
                  onClick={onOpenClient}
                  aria-label="启动战网"
                  size="icon-sm"
                  variant="quiet"
                />
              }
            >
              <ExternalLink />
            </TooltipTrigger>
            <TooltipContent>启动战网</TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  disabled={busy || loginSession !== null}
                  onClick={onRefresh}
                  aria-label="刷新账号"
                  size="icon-sm"
                  variant="quiet"
                />
              }
            >
              <RefreshCw />
            </TooltipTrigger>
            <TooltipContent>刷新账号</TooltipContent>
          </Tooltip>
        </div>

        <OperationBar operation={operation} />

        <div className="col-start-3 flex items-center gap-0.5 justify-self-end">
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  disabled={busy || loginSession !== null}
                  onClick={onConfigurePath}
                  aria-label="选择战网客户端"
                  size="icon-sm"
                  variant="quiet"
                />
              }
            >
              <FolderCog />
            </TooltipTrigger>
            <TooltipContent>选择战网客户端</TooltipContent>
          </Tooltip>
        </div>
      </div>
    </section>
  )
}
