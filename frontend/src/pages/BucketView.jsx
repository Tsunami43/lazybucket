import { useState, useEffect, useRef } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { Archive, Folder, Upload, Download, Eye, Pencil, Trash2, LogOut } from 'lucide-react'
import { listObjects, uploadObject, deleteObject, renameObject, downloadUrl, downloadObject, logout } from '../api'

function formatSize(bytes) {
  if (bytes === 0) return '—'
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`
  return `${(bytes / 1024 / 1024 / 1024).toFixed(1)} GB`
}

export default function BucketView() {
  const { name } = useParams()
  const navigate = useNavigate()
  const [objects, setObjects] = useState([])
  const [loading, setLoading] = useState(true)
  const [prefix, setPrefix] = useState('')
  const [uploading, setUploading] = useState(false)
  const [renaming, setRenaming] = useState(null)
  const [renameTo, setRenameTo] = useState('')
  const [confirmDelete, setConfirmDelete] = useState(null)
  const fileInput = useRef(null)

  const load = async () => {
    setLoading(true)
    const res = await listObjects(name, prefix || undefined)
    if (res.ok) setObjects(await res.json())
    setLoading(false)
  }

  useEffect(() => { load() }, [name, prefix])

  const handleUpload = async (e) => {
    const files = Array.from(e.target.files)
    if (!files.length) return
    setUploading(true)
    for (const file of files) {
      const key = prefix ? `${prefix}${file.name}` : file.name
      await uploadObject(name, key, file)
    }
    setUploading(false)
    fileInput.current.value = ''
    load()
  }

  const handleDelete = (key) => {
    setConfirmDelete(key)
  }

  const confirmDeleteObject = async () => {
    await deleteObject(name, confirmDelete)
    setConfirmDelete(null)
    load()
  }

  const handleRename = async (e) => {
    e.preventDefault()
    if (!renameTo.trim()) return
    await renameObject(name, renaming.key, renameTo.trim())
    setRenaming(null)
    setRenameTo('')
    load()
  }

  return (
    <div className="layout">
      <nav className="navbar">
        <div className="navbar-brand">
          <Archive size={22} strokeWidth={1.5} />
          <span>LazyBucket</span>
        </div>
        <button className="btn btn-ghost" onClick={() => { logout(); navigate('/login') }}>
          <LogOut size={16} />
          Sign out
        </button>
      </nav>

      <main className="main-content">
        <button className="back-btn" onClick={() => navigate('/buckets')}>
          ← All buckets
        </button>

        <div className="page-header">
          <h1 className="page-title">
            <Folder size={24} strokeWidth={1.5} />
            <span style={{ marginLeft: 8 }}>{name}</span>
          </h1>
          <div style={{ display: 'flex', gap: 10, alignItems: 'center' }}>
            <input
              type="text"
              placeholder="Filter by prefix…"
              value={prefix}
              onChange={e => setPrefix(e.target.value)}
              style={{
                padding: '8px 12px',
                borderRadius: 8,
                border: '1.5px solid #e5e7eb',
                fontSize: 13,
                outline: 'none',
                width: 180,
              }}
            />
            <button
              className="btn btn-primary"
              onClick={() => fileInput.current.click()}
              disabled={uploading}
            >
              {uploading ? <span className="spinner" /> : <><Upload size={15} /> Upload</>}
            </button>
            <input
              ref={fileInput}
              type="file"
              multiple
              className="upload-input"
              onChange={handleUpload}
            />
          </div>
        </div>

        {loading ? (
          <div className="empty"><p>Loading…</p></div>
        ) : objects.length === 0 ? (
          <div className="empty">
            <Folder size={48} strokeWidth={1} />
            <p>No files yet. Upload something.</p>
          </div>
        ) : (
          <div className="table-wrap">
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Size</th>
                  <th>Type</th>
                  <th>Created</th>
                  <th>ETag</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {objects.map(obj => (
                  <tr key={obj.key}>
                    <td className="td-name" title={obj.key}>{obj.key}</td>
                    <td>{formatSize(obj.size)}</td>
                    <td>
                      {obj.content_type
                        ? <span className="badge">{obj.content_type.split('/')[1] ?? obj.content_type}</span>
                        : '—'}
                    </td>
                    <td style={{ fontSize: 12, color: '#6b7280', whiteSpace: 'nowrap' }}>
                      {obj.created_at ? new Date(obj.created_at.replace(' ', 'T')).toLocaleDateString('ru-RU') : '—'}
                    </td>
                    <td style={{ fontFamily: 'monospace', fontSize: 11, color: '#9ca3af' }}>
                      {obj.etag.slice(0, 12)}…
                    </td>
                    <td>
                      <div className="td-actions">
                        <a
                          href={downloadUrl(name, obj.key)}
                          target="_blank"
                          rel="noreferrer"
                          className="btn-icon"
                          title="Preview"
                        >
                          <Eye size={15} />
                        </a>
                        <button
                          className="btn-icon"
                          title="Download"
                          onClick={() => downloadObject(name, obj.key)}
                        >
                          <Download size={15} />
                        </button>
                        <button
                          className="btn-icon"
                          title="Rename"
                          onClick={() => { setRenaming(obj); setRenameTo(obj.key) }}
                        >
                          <Pencil size={15} />
                        </button>
                        <button
                          className="btn-icon danger"
                          title="Delete"
                          onClick={() => handleDelete(obj.key)}
                        >
                          <Trash2 size={15} />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </main>

      {confirmDelete && (
        <div className="modal-overlay" onClick={() => setConfirmDelete(null)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h2>Delete file</h2>
            <p>Are you sure you want to delete <strong>"{confirmDelete}"</strong>?</p>
            <div className="modal-actions">
              <button className="btn btn-secondary" onClick={() => setConfirmDelete(null)}>Cancel</button>
              <button className="btn btn-danger" onClick={confirmDeleteObject}>Delete</button>
            </div>
          </div>
        </div>
      )}

      {renaming && (
        <div className="modal-overlay" onClick={() => setRenaming(null)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h2>Rename file</h2>
            <form onSubmit={handleRename}>
              <div className="form-group">
                <label>New path</label>
                <input
                  type="text"
                  value={renameTo}
                  onChange={e => setRenameTo(e.target.value)}
                  autoFocus
                />
              </div>
              <div className="modal-actions">
                <button type="button" className="btn btn-secondary" onClick={() => setRenaming(null)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">Rename</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}
