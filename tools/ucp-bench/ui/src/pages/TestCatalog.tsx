import { useQuery } from '@tanstack/react-query'
import { fetchTests, TestCase } from '../api/client'
import { Link } from 'react-router-dom'

export default function TestCatalog() {
  const { data: tests, isLoading } = useQuery<TestCase[]>({
    queryKey: ['tests'],
    queryFn: fetchTests,
  })

  const grouped = (tests ?? []).reduce<Record<string, TestCase[]>>((acc, test) => {
    const key = test.command_type
    if (!acc[key]) {
      acc[key] = []
    }
    acc[key].push(test)
    return acc
  }, {})

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Test Catalog</h1>
        <p className="text-gray-500">
          Browse the exact prompts, validation rules, and target blocks that define each benchmark test.
        </p>
      </div>

      {isLoading ? (
        <div className="text-gray-500">Loading tests…</div>
      ) : (
        Object.entries(grouped).map(([category, list]) => (
          <section key={category} className="bg-white border border-gray-200 rounded-xl p-6 space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm uppercase tracking-wide text-gray-500">{category}</p>
                <p className="text-lg font-semibold text-gray-900">{list.length} tests</p>
              </div>
            </div>

            <div className="space-y-4">
              {list.map((test) => (
                <article key={test.id} className="border border-gray-200 rounded-lg p-4 hover:border-blue-300 transition-colors">
                  <div className="flex items-center justify-between gap-4">
                    <div>
                      <p className="text-sm font-mono text-gray-500">{test.id}</p>
                      <h3 className="text-lg font-semibold text-gray-900">{test.description}</h3>
                    </div>
                    <Link
                      to={`/tests/${test.id}`}
                      className="text-sm font-medium text-blue-600 hover:text-blue-700"
                    >
                      View details →
                    </Link>
                  </div>
                  <div className="mt-3">
                    <p className="text-xs font-medium text-gray-500">Prompt</p>
                    <p className="text-sm text-gray-800 whitespace-pre-wrap bg-gray-50 rounded-md p-3 mt-1">{test.prompt}</p>
                  </div>
                  <div className="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
                    <div className="bg-gray-50 rounded-md p-3">
                      <p className="text-xs font-medium text-gray-500">Expected Pattern</p>
                      <p className="text-gray-800 font-mono break-all">
                        {test.expected_pattern ?? '—'}
                      </p>
                    </div>
                    <div className="bg-gray-50 rounded-md p-3">
                      <p className="text-xs font-medium text-gray-500">Validation</p>
                      <ul className="text-gray-800 space-y-1">
                        <li>Must parse: {test.validation.must_parse ? 'Yes' : 'No'}</li>
                        <li>Must execute: {test.validation.must_execute ? 'Yes' : 'No'}</li>
                        {test.validation.target_block_id && (
                          <li className="font-mono text-xs text-gray-600">
                            Target block: {test.validation.target_block_id}
                          </li>
                        )}
                      </ul>
                    </div>
                  </div>
                </article>
              ))}
            </div>
          </section>
        ))
      )}
    </div>
  )
}
