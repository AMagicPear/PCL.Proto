import { invoke, isTauri } from '@tauri-apps/api/core'

// 通用 Modrinth API 请求
async function modrinthFetch<T>(endpoint: string): Promise<T> {
  if (isTauri()) return invoke<T>('fetch_with_modrinth', { endpoint })
  const response = await fetch(`https://api.modrinth.com/v2/${endpoint}`)
  if (!response.ok) throw new Error(`Modrinth 请求失败：HTTP ${response.status}`)
  return response.json()
}

interface SearchOptions {
  query?: string
  facets?: string[][] // 二维数组，支持 AND/OR 逻辑
  index?: 'relevance' | 'downloads' | 'follows' | 'newest' | 'updated'
  offset?: number
  limit?: number
}

export interface ISearchHit {
  title: string
  description: string
  icon_url: string
  categories: string[]
  downloads: number
  date_modified: string
  project_id: string
  project_type: 'mod' | 'modpack' | 'resourcepack' | 'shader'
}

export interface IProject {
  slug: string
  id: string
  title: string
  description: string
  categories: string[]
  client_side: 'required' | 'optional' | 'unsupported' | 'unknown'
  server_side: 'required' | 'optional' | 'unsupported' | 'unknown'
  body: string
  status:
    | 'approved'
    | 'archived'
    | 'rejected'
    | 'draft'
    | 'unlisted'
    | 'processing'
    | 'withheld'
    | 'scheduled'
    | 'private'
    | 'unknown'
  requested_status: 'approved' | 'archived' | 'unlisted' | 'private' | 'draft' | null
  additional_categories: string[]
  issues_url: string | null
  source_url: string | null
  wiki_url: string | null
  donation_urls: string[]
  project_type: 'mod' | 'modpack' | 'resourcepack' | 'shader'
  downloads: number
  published: string
  updated: string
  versions: string[]
  loaders: string[]
  team: string
  organization: string | null
  icon_url: string
}

export interface IVersion {
  id: string
  name: string
  version_number: string
  game_versions: string[]
  loaders: string[]
  date_published: string
  files: { url: string; filename: string; primary: boolean; size: number }[]
}

// 搜索项目
async function searchProjects(options: SearchOptions = {}) {
  const params = new URLSearchParams()
  if (options.query) params.append('query', options.query)
  if (options.facets) params.append('facets', JSON.stringify(options.facets))
  if (options.index) params.append('index', options.index)
  if (options.offset) params.append('offset', options.offset.toString())
  if (options.limit) params.append('limit', options.limit.toString())
  const data = await modrinthFetch<{
    hits: ISearchHit[]
    offset: number
    limit: number
    total_hits: number
  }>(`search?${params.toString()}`)
  if (
    !data ||
    !Array.isArray(data.hits) ||
    !Number.isFinite(data.total_hits) ||
    data.hits.some(
      (item) =>
        !item ||
        typeof item.project_id !== 'string' ||
        typeof item.title !== 'string' ||
        typeof item.description !== 'string' ||
        !Number.isFinite(item.downloads) ||
        typeof item.date_modified !== 'string' ||
        !Array.isArray(item.categories),
    )
  )
    throw new Error('Modrinth 搜索结果格式错误')
  return data
}

// 获取项目详情
async function getProject(id: string): Promise<IProject> {
  const data = await modrinthFetch<IProject>(`project/${encodeURIComponent(id)}`)
  if (
    !data ||
    typeof data.id !== 'string' ||
    typeof data.title !== 'string' ||
    typeof data.description !== 'string' ||
    typeof data.slug !== 'string' ||
    !Number.isFinite(data.downloads) ||
    typeof data.updated !== 'string' ||
    !Array.isArray(data.categories)
  )
    throw new Error('Modrinth 项目信息格式错误')
  return data
}

// 获取项目版本
async function getProjectVersions(id: string) {
  const data = await modrinthFetch<IVersion[]>(`project/${encodeURIComponent(id)}/version`)
  if (
    !Array.isArray(data) ||
    data.some(
      (item) =>
        !item ||
        typeof item.id !== 'string' ||
        typeof item.name !== 'string' ||
        !Array.isArray(item.game_versions) ||
        !Array.isArray(item.loaders) ||
        !Array.isArray(item.files) ||
        item.files.some(
          (file) => !file || typeof file.url !== 'string' || typeof file.filename !== 'string',
        ),
    )
  )
    throw new Error('Modrinth 版本列表格式错误')
  return data
}

export default {
  searchProjects,
  getProject,
  getProjectVersions,
}
