import { useQuery } from '@tanstack/react-query'
import { useMemo, useState } from 'react'
import { fetchDocument, DocumentSnapshot, BlockSnapshot } from '../api/client'

type TabId = 'ucm' | 'llm' | 'blocks'

export default function DocumentViewer() {
  const { data, isLoading, error } = useQuery({
    queryKey: ['document'],
    queryFn: fetchDocument,
  })
  const [activeTab, setActiveTab] = useState<TabId>('ucm')
  const tabs: { id: TabId; label: string; description: string }[] = [
    { id: 'ucm', label: 'UCM JSON', description: 'Canonical graph definition used for execution' },
    { id: 'llm', label: 'LLM Prompt View', description: 'Token-optimized adjacency list sent to the model' },
    { id: 'blocks', label: 'Block Index', description: 'Sorted preview of every block ID/label/type' },
  ]

  if (isLoading) {
    return <div className="text-gray-500">Loading document…</div>
  }

  if (error || !data) {
    return <div className="text-red-600">Unable to load document snapshot.</div>
  }

  const ucmJson = useMemo(() => JSON.stringify(data.ucm, null, 2), [data.ucm])
  const blocks = useMemo(() => sortBlocks(data.snapshot), [data.snapshot])

  return (
    <div className="space-y-6">
      <header className="flex items-start justify-between">
        <div>
          <p className="text-sm uppercase tracking-wide text-gray-500">Benchmark Document</p>
          <h1 className="text-2xl font-bold text-gray-900">Machine Learning Tutorial</h1>
          <p className="text-gray-500 mt-1">
            Canonical document used for all test cases. Use this view to inspect structure, block IDs, and previews before running benchmarks.
          </p>
        </div>
        <div className="text-right">
          <p className="text-sm text-gray-500">Blocks</p>
          <p className="text-3xl font-mono text-gray-900">{data.snapshot.block_count}</p>
        </div>
      </header>

      <section className="bg-white border border-gray-200 rounded-xl">
        <div className="border-b border-gray-200 flex">
          {tabs.map((tab) => {
            const isActive = activeTab === tab.id
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`px-4 py-3 text-sm font-medium border-b-2 transition ${
                  isActive
                    ? 'text-blue-600 border-blue-500 bg-white'
                    : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                {tab.label}
              </button>
            )
          })}
        </div>

        <div className="p-6 space-y-3">
          <p className="text-sm text-gray-500">{tabs.find((t) => t.id === activeTab)?.description}</p>

          {activeTab === 'ucm' && (
            <pre className="text-xs font-mono text-gray-900 whitespace-pre bg-gray-50 border border-gray-200 rounded-lg p-4 max-h-[520px] overflow-auto">
              {ucmJson}
            </pre>
          )}

          {activeTab === 'llm' && (
            <pre className="text-sm whitespace-pre-wrap leading-relaxed text-gray-100 bg-gray-900 rounded-lg p-4 max-h-[520px] overflow-auto">
              {data.description}
            </pre>
          )}

          {activeTab === 'blocks' && (
            <div className="overflow-auto rounded-lg border border-gray-200">
              <table className="min-w-full text-sm">
                <thead className="bg-gray-50 text-gray-500 uppercase text-xs tracking-wide">
                  <tr>
                    <th className="text-left px-4 py-3 font-semibold">Block ID</th>
                    <th className="text-left px-4 py-3 font-semibold">Label</th>
                    <th className="text-left px-4 py-3 font-semibold">Type</th>
                    <th className="text-left px-4 py-3 font-semibold">Preview</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {blocks.map((block) => (
                    <tr key={block.id} className="hover:bg-gray-50">
                      <td className="px-4 py-3 font-mono text-xs text-gray-700">{block.id}</td>
                      <td className="px-4 py-3 text-gray-900">{block.label ?? '—'}</td>
                      <td className="px-4 py-3 text-gray-600">{block.content_type}</td>
                      <td className="px-4 py-3 text-gray-600">{block.content_preview}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </section>
    </div>
  )
}

function sortBlocks(snapshot: DocumentSnapshot): BlockSnapshot[] {
  return Object.values(snapshot.blocks).sort((a, b) => a.id.localeCompare(b.id))
}
