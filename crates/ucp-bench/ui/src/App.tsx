import { BrowserRouter, Routes, Route, Link, useLocation } from 'react-router-dom'
import { useState } from 'react'
import {
  LayoutDashboard,
  Play,
  History,
  BookOpen,
  FileText,
  Cpu,
  Sparkles
} from 'lucide-react'
import Dashboard from './pages/Dashboard'
import NewBenchmark from './pages/NewBenchmark'
import RunHistory from './pages/RunHistory'
import RunDetails from './pages/RunDetails'
import TestDetails from './pages/TestDetails'
import TestCatalog from './pages/TestCatalog'
import DocumentViewer from './pages/DocumentViewer'
import CoreBenchmark from './pages/CoreBenchmark'
import Playground from './pages/Playground'

function Sidebar() {
  const location = useLocation()
  const [isExpanded, setIsExpanded] = useState(false)
  
  const links = [
    { to: '/', icon: LayoutDashboard, label: 'Dashboard' },
    { to: '/new', icon: Play, label: 'New Benchmark' },
    { to: '/core', icon: Cpu, label: 'Core Benchmarks' },
    { to: '/playground', icon: Sparkles, label: 'Playground' },
    { to: '/tests', icon: BookOpen, label: 'Test Catalog' },
    { to: '/document', icon: FileText, label: 'Document' },
    { to: '/history', icon: History, label: 'Run History' },
  ]

  return (
    <aside 
      className={`bg-white border-r border-gray-200 min-h-screen transition-all duration-300 ${
        isExpanded ? 'w-64' : 'w-16'
      }`}
      onMouseEnter={() => setIsExpanded(true)}
      onMouseLeave={() => setIsExpanded(false)}
    >
      <div className="p-4 border-b border-gray-200 overflow-hidden">
        {isExpanded ? (
          <div className="animate-in fade-in duration-200">
            <h1 className="text-xl font-bold text-gray-900">UCP Bench</h1>
            <p className="text-sm text-gray-500">LLM Benchmarking System</p>
          </div>
        ) : (
          <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
            <span className="text-white font-bold text-sm">UC</span>
          </div>
        )}
      </div>
      <nav className="p-2 space-y-1">
        {links.map(({ to, icon: Icon, label }) => {
          const isActive = location.pathname === to
          return (
            <Link
              key={to}
              to={to}
              className={`flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${
                isActive 
                  ? 'bg-blue-50 text-blue-700' 
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
              title={!isExpanded ? label : undefined}
            >
              <Icon size={20} className="flex-shrink-0" />
              {isExpanded && <span className="font-medium whitespace-nowrap">{label}</span>}
            </Link>
          )
        })}
      </nav>
    </aside>
  )
}

function App() {
  return (
    <BrowserRouter future={{ v7_startTransition: true, v7_relativeSplatPath: true }}>
      <div className="flex min-h-screen bg-gray-50">
        <Sidebar />
        <main className="flex-1 p-8">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/new" element={<NewBenchmark />} />
            <Route path="/core" element={<CoreBenchmark />} />
            <Route path="/playground" element={<Playground />} />
            <Route path="/tests" element={<TestCatalog />} />
            <Route path="/document" element={<DocumentViewer />} />
            <Route path="/tests/:testId" element={<TestDetails />} />
            <Route path="/history" element={<RunHistory />} />
            <Route path="/run/:runId" element={<RunDetails />} />
            <Route path="/run/:runId/test/:testId" element={<TestDetails />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  )
}

export default App
