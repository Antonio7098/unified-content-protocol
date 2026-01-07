import { useQuery } from '@tanstack/react-query'
import { useParams, useSearchParams, Link } from 'react-router-dom'
import { fetchTestResult } from '../api/client'
import { ArrowLeft, CheckCircle, XCircle } from 'lucide-react'

export default function TestDetails() {
  const { runId, testId } = useParams<{ runId: string; testId: string }>()
  const [searchParams] = useSearchParams()
  const provider = searchParams.get('provider') || ''

  const { data: result, isLoading } = useQuery({
    queryKey: ['testResult', runId, testId, provider],
    queryFn: () => fetchTestResult(runId!, testId!),
  })

  if (isLoading) return <div className="text-gray-500">Loading...</div>
  if (!result) return <div className="text-red-500">Test result not found</div>

  return (
    <div className="space-y-6 max-w-4xl">
      <div className="flex items-center gap-4">
        <Link to={`/run/${runId}`} className="text-gray-500 hover:text-gray-700">
          <ArrowLeft size={20} />
        </Link>
        <div>
          <h1 className="text-2xl font-bold text-gray-900">{result.test_id}</h1>
          <p className="text-gray-500">{result.provider_id}/{result.model_id}</p>
        </div>
        {result.success ? (
          <CheckCircle className="text-green-500 ml-auto" size={24} />
        ) : (
          <XCircle className="text-red-500 ml-auto" size={24} />
        )}
      </div>

      {/* Metrics */}
      <div className="grid grid-cols-4 gap-4">
        <MetricCard label="Latency" value={`${result.latency_ms}ms`} />
        <MetricCard label="Input Tokens" value={result.input_tokens} />
        <MetricCard label="Output Tokens" value={result.output_tokens} />
        <MetricCard label="Cost" value={`$${result.cost_usd.toFixed(6)}`} />
      </div>

      {/* Error */}
      {result.error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <h3 className="font-medium text-red-800 mb-1">Error: {result.error.category}</h3>
          <p className="text-red-700">{result.error.message}</p>
        </div>
      )}

      {/* Context */}
      <Section title="Test Description">
        <p className="text-gray-700">{result.context.test_description}</p>
      </Section>

      <Section title="Task Prompt">
        <p className="text-gray-700 whitespace-pre-wrap">{result.context.task_prompt}</p>
      </Section>

      <Section title="Full User Prompt">
        <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm">
          {result.context.full_user_prompt}
        </pre>
      </Section>

      <Section title="Raw LLM Response">
        <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm">
          {result.context.raw_response}
        </pre>
      </Section>

      <Section title="Extracted UCL">
        <pre className="bg-blue-900 text-blue-100 p-4 rounded-lg overflow-x-auto text-sm font-mono">
          {result.context.extracted_ucl}
        </pre>
      </Section>

      {result.context.expected_pattern && (
        <Section title="Expected Pattern">
          <code className="bg-gray-100 px-2 py-1 rounded">{result.context.expected_pattern}</code>
          {result.context.pattern_matched !== null && (
            <span className={`ml-2 ${result.context.pattern_matched ? 'text-green-600' : 'text-red-600'}`}>
              {result.context.pattern_matched ? '✓ Matched' : '✗ Not matched'}
            </span>
          )}
        </Section>
      )}

      {/* Document Snapshots */}
      {result.document_before && (
        <Section title="Document Before">
          <div className="text-sm text-gray-500 mb-2">{result.document_before.block_count} blocks</div>
          <div className="space-y-1">
            {Object.values(result.document_before.blocks).slice(0, 10).map((block: any) => (
              <div key={block.id} className="bg-gray-50 p-2 rounded text-sm">
                <span className="font-mono text-gray-500">{block.id}</span>
                <span className="ml-2 text-gray-700">{block.content_preview}</span>
              </div>
            ))}
          </div>
        </Section>
      )}

      {result.diff && (
        <Section title="Document Changes">
          <p className="text-gray-600 mb-2">{result.diff.summary}</p>
          {result.diff.added_blocks.length > 0 && (
            <div className="mb-2">
              <span className="text-green-600 font-medium">Added:</span>
              {result.diff.added_blocks.map((id: string) => (
                <span key={id} className="ml-2 font-mono text-sm bg-green-100 px-1 rounded">{id}</span>
              ))}
            </div>
          )}
          {result.diff.removed_blocks.length > 0 && (
            <div className="mb-2">
              <span className="text-red-600 font-medium">Removed:</span>
              {result.diff.removed_blocks.map((id: string) => (
                <span key={id} className="ml-2 font-mono text-sm bg-red-100 px-1 rounded">{id}</span>
              ))}
            </div>
          )}
          {result.diff.modified_blocks.length > 0 && (
            <div>
              <span className="text-yellow-600 font-medium">Modified:</span>
              {result.diff.modified_blocks.map((mod: any) => (
                <div key={mod.block_id} className="mt-1 ml-4 text-sm">
                  <span className="font-mono">{mod.block_id}</span>
                  <div className="bg-red-50 p-1 rounded mt-1">- {mod.before}</div>
                  <div className="bg-green-50 p-1 rounded">+ {mod.after}</div>
                </div>
              ))}
            </div>
          )}
        </Section>
      )}

      {/* Scores */}
      <Section title="Scores">
        <div className="flex gap-8">
          <div>
            <span className="text-gray-500">Semantic Score:</span>
            <span className="ml-2 font-medium">{(result.semantic_score * 100).toFixed(0)}%</span>
          </div>
          <div>
            <span className="text-gray-500">Efficiency Score:</span>
            <span className="ml-2 font-medium">{(result.efficiency_score * 100).toFixed(0)}%</span>
          </div>
        </div>
      </Section>
    </div>
  )
}

function MetricCard({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="bg-white rounded-lg border border-gray-200 p-4">
      <p className="text-sm text-gray-500">{label}</p>
      <p className="text-xl font-bold">{value}</p>
    </div>
  )
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="bg-white rounded-lg border border-gray-200 p-6">
      <h2 className="text-lg font-semibold mb-3">{title}</h2>
      {children}
    </div>
  )
}
