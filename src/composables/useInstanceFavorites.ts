import { ref } from 'vue'

const favoriteKey = 'pcl-instance-favorites'
const favorites = ref<string[]>(load())

function load(): string[] {
  try {
    const saved: unknown = JSON.parse(localStorage.getItem(favoriteKey) || '[]')
    if (Array.isArray(saved))
      return saved.filter((value): value is string => typeof value === 'string')
  } catch {
    /* Invalid preferences do not prevent using the list. */
  }
  return []
}

export function useInstanceFavorites() {
  function isFavorite(directory: string) {
    return favorites.value.includes(directory)
  }
  function toggleFavorite(directory: string): string {
    favorites.value = isFavorite(directory)
      ? favorites.value.filter((path) => path !== directory)
      : [...favorites.value, directory]
    try {
      localStorage.setItem(favoriteKey, JSON.stringify(favorites.value))
      return ''
    } catch (error) {
      return String(error)
    }
  }
  return { favorites, isFavorite, toggleFavorite }
}
