import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { isAuthenticated } from './api'
import Login from './pages/Login'
import Buckets from './pages/Buckets'
import BucketView from './pages/BucketView'

function PrivateRoute({ children }) {
  return isAuthenticated() ? children : <Navigate to="/login" replace />
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/buckets" element={<PrivateRoute><Buckets /></PrivateRoute>} />
        <Route path="/buckets/:name" element={<PrivateRoute><BucketView /></PrivateRoute>} />
        <Route path="*" element={<Navigate to={isAuthenticated() ? '/buckets' : '/login'} replace />} />
      </Routes>
    </BrowserRouter>
  )
}
