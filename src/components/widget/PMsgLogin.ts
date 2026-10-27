import { useModal } from '@/composables/useModal'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'

// Device-code login uses the existing application modal.
export function PMsgLogin(code: string, verificationUrl: string, caption: string) {
  const modal = useModal()
  return modal.open({
    title: pclText('Launch.Account.LoginDialog.Title'),
    content: caption,
    plainText: true,
    buttons: [
      {
        content: pclText('Launch.Account.LoginDialog.ReopenWebpage'),
        operation: () => {
          if (/^https:\/\//i.test(verificationUrl))
            window.open(verificationUrl, '_blank', 'noopener,noreferrer')
        },
      },
      {
        content: pclText('Launch.Account.LoginDialog.CopyCode'),
        operation: async () => {
          try {
            await navigator.clipboard.writeText(code)
            sideTip.show('已成功复制！', 'success')
          } catch {
            sideTip.show('复制失败，请检查剪贴板权限。', 'warn')
          }
        },
      },
      { content: pclText('Common.Action.Cancel'), operation: () => modal.close(false) },
    ],
  })
}
