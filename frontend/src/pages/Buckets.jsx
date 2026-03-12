import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { Archive, Folder, Plus, Pencil, Trash2, LogOut } from 'lucide-react'
import { listBuckets, createBucket, deleteBucket, renameBucket, logout } from '../api'

export default function Buckets() {
  const navigate = useNavigate()
  const [buckets, setBuckets] = useState([])
  const [loading, setLoading] = useState(true)
  const [showCreate, setShowCreate] = useState(false)
  const [newName, setNewName] = useState('')
  const [renaming, setRenaming] = useState(null)
  const [renameTo, setRenameTo] = useState('')
  const [errorMsg, setErrorMsg] = useState(null)
  const [confirmDelete, setConfirmDelete] = useState(null)

  const load = async () => {
    const res = await listBuckets()
    if (res.ok) setBuckets(await res.json())
    else setBuckets([])
    setLoading(false)
  }

  useEffect(() => { load() }, [])

  const handleCreate = async (e) => {
    e.preventDefault()
    if (!newName.trim()) return
    const res = await createBucket(newName.trim())
    if (res.status === 409) {
      setShowCreate(false)
      setErrorMsg(`Bucket "${newName.trim()}" already exists.`)
      setNewName('')
      return
    }
    setNewName('')
    setShowCreate(false)
    load()
  }

  const handleDelete = (name) => {
    setConfirmDelete(name)
  }

  const confirmDeleteBucket = async () => {
    await deleteBucket(confirmDelete)
    setConfirmDelete(null)
    load()
  }

  const handleRename = async (e) => {
    e.preventDefault()
    if (!renameTo.trim()) return
    await renameBucket(renaming, renameTo.trim())
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
        <div className="page-header">
          <h1 className="page-title">Buckets</h1>
          <button className="btn btn-primary" onClick={() => setShowCreate(true)}>
            <Plus size={16} />
            New bucket
          </button>
        </div>

        {loading ? (
          <div className="empty"><p>Loading…</p></div>
        ) : buckets.length === 0 ? (
          <div className="empty">
            <Archive size={48} strokeWidth={1} />
            <p>No buckets yet. Create your first one.</p>
          </div>
        ) : (
          <div className="buckets-grid">
            {buckets.map(b => (
              <div key={b.name} className="bucket-card" onClick={() => navigate(`/buckets/${b.name}`)}>
                <div className="bucket-icon"><Folder size={32} strokeWidth={1.5} /></div>
                <div className="bucket-name">{b.name}</div>
                <div className="bucket-date">
                  {b.created_at ? new Date(b.created_at.replace(' ', 'T')).toLocaleDateString('ru-RU') : '—'}
                </div>
                <div className="bucket-actions" onClick={e => e.stopPropagation()}>
                  <button
                    className="btn btn-secondary"
                    style={{ fontSize: 12, padding: '4px 10px' }}
                    onClick={() => { setRenaming(b.name); setRenameTo(b.name) }}
                  >
                    <Pencil size={13} />
                    Rename
                  </button>
                  <button
                    className="btn btn-danger"
                    style={{ fontSize: 12, padding: '4px 10px' }}
                    onClick={() => handleDelete(b.name)}
                  >
                    <Trash2 size={13} />
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>

      {showCreate && (
        <div className="modal-overlay" onClick={() => setShowCreate(false)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h2>New bucket</h2>
            <form onSubmit={handleCreate}>
              <div className="form-group">
                <label>Name</label>
                <input
                  type="text"
                  placeholder="my-bucket"
                  value={newName}
                  onChange={e => setNewName(e.target.value)}
                  autoFocus
                />
              </div>
              <div className="modal-actions">
                <button type="button" className="btn btn-secondary" onClick={() => setShowCreate(false)}>Cancel</button>
                <button type="submit" className="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {confirmDelete && (
        <div className="modal-overlay" onClick={() => setConfirmDelete(null)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h2>Delete bucket</h2>
            <p>Are you sure you want to delete <strong>"{confirmDelete}"</strong>?</p>
            <div className="modal-actions">
              <button className="btn btn-secondary" onClick={() => setConfirmDelete(null)}>Cancel</button>
              <button className="btn btn-danger" onClick={confirmDeleteBucket}>Delete</button>
            </div>
          </div>
        </div>
      )}

      {errorMsg && (
        <div className="modal-overlay" onClick={() => setErrorMsg(null)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h2>Error</h2>
            <p>{errorMsg}</p>
            <div className="modal-actions">
              <button className="btn btn-primary" onClick={() => setErrorMsg(null)}>OK</button>
            </div>
          </div>
        </div>
      )}

      {renaming && (
        <div className="modal-overlay" onClick={() => setRenaming(null)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h2>Rename bucket</h2>
            <form onSubmit={handleRename}>
              <div className="form-group">
                <label>New name</label>
                <input
                  type="text"
                  value={renameTo}
                  onChange={e => setRenameTo(e.target.value)}
                  autoFocus
                />
              </div>
              <div className="modal-actions">
                <button type="button" className="btn btn-secondary" onClick={() => setRenaming(null)}>Cancel</button>
                <button type="submit" className="btn btn-primary">Rename</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}
