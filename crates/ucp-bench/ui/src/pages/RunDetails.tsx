import { useQuery } from '@tanstack/react-query'
import { useParams, Link } from 'react-router-dom'
import { fetchRun, fetchDocument } from '../api/client'
import { CheckCircle, XCircle, Clock, DollarSign, FileText } from 'lucide-react'

export default function RunDetails() {
  const { runId } = useParams<{ runId: string }>()
  const { data: run, isLoading } = useQuery({
    queryKey: ['run', runId],
    queryFn: () => fetchRun(runId!),
    refetchInterval: (data) => data?.status === 'completed' ? false : 2000,
  })
  const { data: documentData } = useQuery({
    queryKey: ['document'],
    queryFn: fetchDocument,
  })

  if (isLoading) return <div className="text-gray-500">Loading...</div>
  if (!run) return <div className="text-red-500">Run not found</div>

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Run Details</h1>
          <p className="text-gray-500 font-mono">{run.run_id}</p>
        </div>
        <StatusBadge status={run.status} />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="md:col-span-2 grid grid-cols-1 md:grid-cols-3 gap-4">
          <StatCard icon={<CheckCircle className="text-green-500" />} label="Passed" value={run.summary?.total_passed || 0} />
          <StatCard icon={<XCircle className="text-red-500" />} label="Failed" value={run.summary?.total_failed || 0} />
          <StatCard icon={<Clock className="text-blue-500" />} label="Duration" value={`${Math.round((run.summary?.total_duration_ms || 0) / 1000)}s`} />
        </div>
        <div className="bg-white border border-gray-200 rounded-lg p-4 space-y-2">
          <div className="flex items-center gap-2 text-gray-500">
            <FileText size={16} />
            <p className="text-sm font-medium">Benchmark Document</p>
          </div>
          <p className="text-sm text-gray-600 line-clamp-4 font-mono whitespace-pre-wrap">
            {documentData?.description ?? 'Loading…'}
          </p>
          <Link
            to="/document"
            className="inline-flex items-center text-sm font-semibold text-blue-600 hover:text-blue-700"
          >
            View full document →
          </Link>
        </div>
      </div>

      {/* Summary */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <StatCard icon={<DollarSign className="text-orange-500" />} label="Cost" value={`$${(run.summary?.total_cost_usd || 0).toFixed(4)}`} />
      </div>

      {/* Results by Provider */}
      {Object.entries(run.results_by_provider || {}).map(([key, results]: [string, any]) => (
        <div key={key} className="bg-white rounded-lg border border-gray-200 p-6">
          <h2 className="text-lg font-semibold mb-4">{results.provider_id}/{results.model_id}</h2>
          <div className="mb-4 flex gap-4 text-sm text-gray-500">
            <span>{results.passed}/{results.total_tests} passed</span>
            <span>Avg: {results.avg_latency_ms}ms</span>
            <span>${results.total_cost_usd.toFixed(4)}</span>
          </div>

          {/* Results by Category */}
          {Object.entries(results.results_by_category || {}).map(([catId, catResults]: [string, any]) => (
            <div key={catId} className="mb-4">
              <h3 className="font-medium text-gray-700 mb-2">{catId}</h3>
              <div className="space-y-2">
                {catResults.tests?.map((test: any) => (
                  <Link
                    key={test.test_id}
                    to={`/run/${runId}/test/${test.test_id}?provider=${key}`}
                    className="block border border-gray-200 rounded p-3 hover:bg-gray-50 transition-colors"
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        {test.success ? (
                          <CheckCircle size={16} className="text-green-500" />
                        ) : (
                          <XCircle size={16} className="text-red-500" />
                        )}
                        <span className="font-mono text-sm">{test.test_id}</span>
                      </div>
                      <div className="flex gap-4 text-sm text-gray-500">
                        <span>{test.latency_ms}ms</span>
                        <span>${test.cost_usd.toFixed(6)}</span>
                      </div>
                    </div>
                    {test.error && (
                      <p className="mt-1 text-sm text-red-600 truncate">{test.error.message}</p>
                    )}
                  </Link>
                ))}
              </div>
            </div>
          ))}
        </div>
      ))}
    </div>
  )
}

function StatCard({ icon, label, value }: { icon: React.ReactNode; label: string; value: string | number }) {
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

function StatusBadge({ status }: { status: any }) {
  if (status === 'completed') return <span className="bg-green-100 text-green-700 px-3 py-1 rounded-full text-sm font-medium">Completed</span>
  if (status === 'pending') return <span className="bg-gray-100 text-gray-700 px-3 py-1 rounded-full text-sm font-medium">Pending</span>
  if (typeof status === 'object' && 'running' in status) {
    return (
      <div className="flex items-center gap-2">
        <div className="w-32 bg-gray-200 rounded-full h-2">
          <div className="bg-blue-600 h-2 rounded-full" style={{ width: `${status.running.progress * 100}%` }} />
        </div>
        <span className="text-sm text-gray-500">{Math.round(status.running.progress * 100)}%</span>
      </div>
    )
  }
  if (typeof status === 'object' && 'failed' in status) return <span className="bg-red-100 text-red-700 px-3 py-1 rounded-full text-sm font-medium">Failed</span>
  return null
}
