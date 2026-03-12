const BASE = import.meta.env.VITE_API_URL ?? ''

const getAuth = () => localStorage.getItem('lb_auth')

const req = (method, path, body, contentType) => {
  const headers = {}
  const auth = getAuth()
  if (auth) headers['Authorization'] = auth
  if (contentType) headers['Content-Type'] = contentType
  return fetch(BASE + path, { method, headers, body })
}

export const isAuthenticated = () => !!getAuth()

export const login = async (username, password) => {
  const res = await fetch(BASE + '/buckets', {
    headers: { Authorization: `${username}:${password}` },
  })
  if (res.ok) {
    localStorage.setItem('lb_auth', `${username}:${password}`)
    return true
  }
  return false
}

export const logout = () => localStorage.removeItem('lb_auth')

// Buckets
export const listBuckets = () => req('GET', '/buckets')
export const createBucket = (name) => req('PUT', `/buckets/${encodeURIComponent(name)}`)
export const deleteBucket = (name) => req('DELETE', `/buckets/${encodeURIComponent(name)}`)
export const renameBucket = (name, newName) =>
  req('PATCH', `/buckets/${encodeURIComponent(name)}`, JSON.stringify({ name: newName }), 'application/json')

// Objects
export const listObjects = (bucket, prefix) =>
  req('GET', `/${encodeURIComponent(bucket)}${prefix ? `?prefix=${encodeURIComponent(prefix)}` : ''}`)
export const uploadObject = (bucket, key, file) =>
  req('PUT', `/${encodeURIComponent(bucket)}/${key}`, file, file.type || 'application/octet-stream')
export const deleteObject = (bucket, key) =>
  req('DELETE', `/${encodeURIComponent(bucket)}/${key}`)
export const renameObject = (bucket, key, newKey) =>
  req('PATCH', `/${encodeURIComponent(bucket)}/${key}`, JSON.stringify({ key: newKey }), 'application/json')
export const downloadUrl = (bucket, key) =>
  `${BASE}/${encodeURIComponent(bucket)}/${key}`
