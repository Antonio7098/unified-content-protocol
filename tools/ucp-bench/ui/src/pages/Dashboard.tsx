import { useQuery } from '@tanstack/react-query'
import { fetchCategories, fetchProviders, fetchRuns } from '../api/client'
import { Activity, CheckCircle, XCircle, Clock, DollarSign } from 'lucide-react'
import { Link } from 'react-router-dom'

export default function Dashboard() {
  const { data: categories } = useQuery({ queryKey: ['categories'], queryFn: fetchCategories })
  const { data: providers } = useQuery({ queryKey: ['providers'], queryFn: fetchProviders })
  const { data: runs } = useQuery({ queryKey: ['runs'], queryFn: fetchRuns })

  const recentRuns = runs?.slice(0, 5) || []
  const totalTests = categories?.reduce((sum, c) => sum + c.test_count, 0) || 0

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <p className="text-gray-500">Overview of your LLM benchmarking system</p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <StatCard
          icon={<Activity className="text-blue-500" />}
          label="Test Categories"
          value={categories?.length || 0}
        />
        <StatCard
          icon={<CheckCircle className="text-green-500" />}
          label="Total Test Cases"
          value={totalTests}
        />
        <StatCard
          icon={<Clock className="text-purple-500" />}
          label="Available Providers"
          value={providers?.length || 0}
        />
        <StatCard
          icon={<DollarSign className="text-orange-500" />}
          label="Total Runs"
          value={runs?.length || 0}
        />
      </div>

      {/* Categories */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <h2 className="text-lg font-semibold mb-4">Test Categories</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {categories?.map(cat => (
            <div key={cat.id} className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="font-medium">{cat.name}</span>
                <span className="text-sm bg-blue-100 text-blue-700 px-2 py-0.5 rounded">
                  {cat.test_count} tests
                </span>
              </div>
              <p className="text-sm text-gray-500">{cat.description}</p>
              <div className="mt-2 flex gap-1">
                {cat.tags.map(tag => (
                  <span key={tag} className="text-xs bg-gray-100 text-gray-600 px-2 py-0.5 rounded">
                    {tag}
                  </span>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Providers */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <h2 className="text-lg font-semibold mb-4">Available Providers</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {providers?.map(provider => (
            <div key={provider.id} className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <div className={`w-2 h-2 rounded-full ${provider.available ? 'bg-green-500' : 'bg-gray-300'}`} />
                <span className="font-medium">{provider.name}</span>
              </div>
              <div className="space-y-1">
                {provider.models.map(model => (
                  <div key={model.id} className="text-sm text-gray-500 truncate">
                    {model.name}
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Recent Runs */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold">Recent Runs</h2>
          <Link to="/history" className="text-sm text-blue-600 hover:underline">
            View all
          </Link>
        </div>
        {recentRuns.length === 0 ? (
          <p className="text-gray-500 text-center py-8">No benchmark runs yet</p>
        ) : (
          <div className="space-y-2">
            {recentRuns.map(run => (
              <Link
                key={run.run_id}
                to={`/run/${run.run_id}`}
                className="block border border-gray-200 rounded-lg p-4 hover:bg-gray-50 transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <span className="font-medium">{run.run_id.slice(0, 8)}...</span>
                    <span className="text-sm text-gray-500 ml-2">
                      {new Date(run.started_at).toLocaleString()}
                    </span>
                  </div>
                  <RunStatusBadge status={run.status} />
                </div>
                {run.summary && (
                  <div className="mt-2 flex gap-4 text-sm text-gray-500">
                    <span>{run.summary.total_passed}/{run.summary.total_tests} passed</span>
                    <span>${run.summary.total_cost_usd.toFixed(4)}</span>
                  </div>
                )}
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}

function StatCard({ icon, label, value }: { icon: React.ReactNode; label: string; value: number }) {
  return (
    <div className="bg-white rounded-lg border border-gray-200 p-4">
      <div className="flex items-center gap-3">
        {icon}
        <div>
          <p className="text-2xl font-bold">{value}</p>
          <p className="text-sm text-gray-500">{label}</p>
        </div>
      </div>
    </div>
  )
}

function RunStatusBadge({ status }: { status: any }) {
  if (status === 'completed') {
    return <span className="bg-green-100 text-green-700 px-2 py-1 rounded text-sm">Completed</span>
  }
  if (status === 'pending') {
    return <span className="bg-gray-100 text-gray-700 px-2 py-1 rounded text-sm">Pending</span>
  }
  if (status === 'cancelled') {
    return <span className="bg-yellow-100 text-yellow-700 px-2 py-1 rounded text-sm">Cancelled</span>
  }
  if (typeof status === 'object' && 'running' in status) {
    return (
      <span className="bg-blue-100 text-blue-700 px-2 py-1 rounded text-sm">
        Running ({Math.round(status.running.progress * 100)}%)
      </span>
    )
  }
  if (typeof status === 'object' && 'failed' in status) {
    return <span className="bg-red-100 text-red-700 px-2 py-1 rounded text-sm">Failed</span>
  }
  return null
}
