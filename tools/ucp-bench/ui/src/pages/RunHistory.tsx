import { useQuery } from '@tanstack/react-query'
import { fetchRuns } from '../api/client'
import { Link } from 'react-router-dom'

export default function RunHistory() {
  const { data: runs, isLoading } = useQuery({ queryKey: ['runs'], queryFn: fetchRuns })

  if (isLoading) return <div className="text-gray-500">Loading...</div>

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold text-gray-900">Run History</h1>
      
      {runs?.length === 0 ? (
        <div className="bg-white rounded-lg border border-gray-200 p-8 text-center">
          <p className="text-gray-500">No benchmark runs yet</p>
          <Link to="/new" className="text-blue-600 hover:underline mt-2 inline-block">
            Create your first benchmark
          </Link>
        </div>
      ) : (
        <div className="bg-white rounded-lg border border-gray-200 overflow-hidden">
          <table className="w-full">
            <thead className="bg-gray-50 border-b border-gray-200">
              <tr>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-500">Run ID</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-500">Started</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-500">Status</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-500">Results</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-500">Cost</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              {runs?.map(run => (
                <tr key={run.run_id} className="hover:bg-gray-50">
                  <td className="px-4 py-3">
                    <Link to={`/run/${run.run_id}`} className="text-blue-600 hover:underline font-mono text-sm">
                      {run.run_id.slice(0, 8)}...
                    </Link>
                  </td>
                  <td className="px-4 py-3 text-sm text-gray-500">
                    {new Date(run.started_at).toLocaleString()}
                  </td>
                  <td className="px-4 py-3">
                    <StatusBadge status={run.status} />
                  </td>
                  <td className="px-4 py-3 text-sm">
                    {run.summary && (
                      <span className={run.summary.total_failed > 0 ? 'text-red-600' : 'text-green-600'}>
                        {run.summary.total_passed}/{run.summary.total_tests} passed
                      </span>
                    )}
                  </td>
                  <td className="px-4 py-3 text-sm text-gray-500">
                    ${run.summary?.total_cost_usd.toFixed(4) || '0.0000'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}

function StatusBadge({ status }: { status: any }) {
  if (status === 'completed') return <span className="bg-green-100 text-green-700 px-2 py-1 rounded text-xs">Completed</span>
  if (status === 'pending') return <span className="bg-gray-100 text-gray-700 px-2 py-1 rounded text-xs">Pending</span>
  if (status === 'cancelled') return <span className="bg-yellow-100 text-yellow-700 px-2 py-1 rounded text-xs">Cancelled</span>
  if (typeof status === 'object' && 'running' in status) return <span className="bg-blue-100 text-blue-700 px-2 py-1 rounded text-xs">Running</span>
  if (typeof status === 'object' && 'failed' in status) return <span className="bg-red-100 text-red-700 px-2 py-1 rounded text-xs">Failed</span>
  return null
}
