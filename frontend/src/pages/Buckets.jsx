import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { listBuckets, createBucket, deleteBucket, renameBucket, logout } from '../api'

export default function Buckets() {
  const navigate = useNavigate()
  const [buckets, setBuckets] = useState([])
  const [loading, setLoading] = useState(true)
  const [showCreate, setShowCreate] = useState(false)
  const [newName, setNewName] = useState('')
  const [renaming, setRenaming] = useState(null)
  const [renameTo, setRenameTo] = useState('')

  const load = async () => {
    const res = await listBuckets()
    if (res.ok) setBuckets(await res.json())
    setLoading(false)
  }

  useEffect(() => { load() }, [])

  const handleCreate = async (e) => {
    e.preventDefault()
    if (!newName.trim()) return
    await createBucket(newName.trim())
    setNewName('')
    setShowCreate(false)
    load()
  }

  const handleDelete = async (name) => {
    if (!confirm(`Delete bucket "${name}"?`)) return
    await deleteBucket(name)
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
          <span>🪣</span>
          <span>LazyBucket</span>
        </div>
        <button className="btn btn-ghost" onClick={() => { logout(); navigate('/login') }}>
          Sign out
        </button>
      </nav>

      <main className="main-content">
        <div className="page-header">
          <h1 className="page-title">Buckets</h1>
          <button className="btn btn-primary" onClick={() => setShowCreate(true)}>
            + New bucket
          </button>
        </div>

        {loading ? (
          <div className="empty"><p>Loading…</p></div>
        ) : buckets.length === 0 ? (
          <div className="empty">
            <div style={{ fontSize: 48 }}>🪣</div>
            <p>No buckets yet. Create your first one.</p>
          </div>
        ) : (
          <div className="buckets-grid">
            {buckets.map(name => (
              <div key={name} className="bucket-card" onClick={() => navigate(`/buckets/${name}`)}>
                <div className="bucket-icon">🪣</div>
                <div className="bucket-name">{name}</div>
                <div className="bucket-actions" onClick={e => e.stopPropagation()}>
                  <button
                    className="btn btn-secondary"
                    style={{ fontSize: 12, padding: '4px 10px' }}
                    onClick={() => { setRenaming(name); setRenameTo(name) }}
                  >
                    Rename
                  </button>
                  <button
                    className="btn btn-danger"
                    style={{ fontSize: 12, padding: '4px 10px' }}
                    onClick={() => handleDelete(name)}
                  >
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
