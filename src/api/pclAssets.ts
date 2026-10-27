export function pclImagePath(source: string, base: string): string {
  if (/^(https?:|data:)/i.test(source)) return source
  const relative = source
    .replace(/^pack:\/\/application:,,,\//i, '')
    .replace(/^.*;component\//i, '')
    .replace(/^\/?images\//i, '')
  return `${base}pcl/images/${relative}`
}
