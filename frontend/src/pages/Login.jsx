import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { login } from '../api'

export default function Login() {
  const navigate = useNavigate()
  const [form, setForm] = useState({ login: '', password: '' })
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)

  const handleSubmit = async (e) => {
    e.preventDefault()
    setError('')
    setLoading(true)
    const ok = await login(form.login, form.password)
    setLoading(false)
    if (ok) {
      navigate('/buckets')
    } else {
      setError('Invalid login or password')
    }
  }

  return (
    <div className="login-page">
      <div className="login-card">
        <div className="login-logo">
          <div style={{ fontSize: 52 }}>🪣</div>
          <h1>LazyBucket</h1>
          <p>Simple file storage</p>
        </div>

        <form onSubmit={handleSubmit}>
          {error && <div className="error-msg">{error}</div>}

          <div className="form-group">
            <label>Login</label>
            <input
              type="text"
              placeholder="Enter login"
              value={form.login}
              onChange={e => setForm(f => ({ ...f, login: e.target.value }))}
              autoFocus
            />
          </div>

          <div className="form-group">
            <label>Password</label>
            <input
              type="password"
              placeholder="Enter password"
              value={form.password}
              onChange={e => setForm(f => ({ ...f, password: e.target.value }))}
            />
          </div>

          <button type="submit" className="btn btn-primary btn-full" disabled={loading}>
            {loading ? <span className="spinner" /> : 'Sign in'}
          </button>
        </form>
      </div>
    </div>
  )
}
