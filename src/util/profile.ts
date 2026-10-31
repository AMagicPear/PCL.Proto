import { pclText } from '@/api/pclLocale'
import type { McProfile } from '@/stores/account'

/** 与 CE 的 ProfileUi.GetProfileInfo 保持一致。 */
export function getProfileTypeLabel(profile?: McProfile | null): string {
  if (!profile) return pclText('Launch.Account.Type.Offline')
  switch (profile.profileType) {
    case 'microsoft':
      return pclText('Launch.Account.Type.Microsoft')
    case 'authlib':
      return `${pclText('Launch.Account.Type.ThirdParty')}${profile.serverName ? ` / ${profile.serverName}` : ''}`
    case 'yggdrasilConnect':
      return `${pclText('Launch.Account.Type.ThirdParty')}${profile.serverName ? ` / ${profile.serverName}` : ' / Yggdrasil Connect'}`
    default:
      return pclText('Launch.Account.Type.Offline')
  }
}

export function getProfileInfo(profile?: McProfile | null): string {
  const label = getProfileTypeLabel(profile)
  return profile?.description ? `${label}，${profile.description}` : label
}
