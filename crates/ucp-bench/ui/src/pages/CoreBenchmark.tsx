import { useState, useEffect } from 'react'
import { 
  Play, 
  Clock, 
  CheckCircle2, 
  XCircle, 
  Loader2,
  ChevronDown,
  ChevronRight,
  Code,
  FileText,
  TreePine,
  RefreshCw
} from 'lucide-react'

interface Category {
  id: string
  name: string
  input_count: number
}

interface Input {
  id: string
  name: string
  description: string
  category: string
  complexity: string
  input: string
}

interface CoreTestResult {
  test_id: string
  input_id: string
  category: string
  benchmark_type: string
  success: boolean
  performance: {
    duration_ns: number
    duration_ms: number
    memory_bytes: number | null
    throughput_ops_sec: number | null
    iterations: number
  }
  validation: {
    valid: boolean
    checks: Array<{
      name: string
      passed: boolean
      expected: string | null
      actual: string | null
    }>
    error_message: string | null
  }
  input_preview: string
  output_preview: string
  structure_preview: string | null
  error_message: string | null
  executed_at: string
}

interface CoreBenchmarkReport {
  report_id: string
  name: string
  started_at: string
  completed_at: string | null
  status: { pending?: true; running?: { progress: number; current_test: string }; completed?: true; failed?: { error: string } }
  metrics: {
    total_tests: number
    passed: number
    failed: number
    success_rate: number
    total_duration_ms: number
    avg_duration_ms: number
    min_duration_ms: number
    max_duration_ms: number
    p50_duration_ms: number
    p95_duration_ms: number
    p99_duration_ms: number
  }
  results: CoreTestResult[]
}

type ViewTab = 'first' | 'second' | 'third'

interface TabConfig {
  label: string
  icon: any
  visible: boolean
}

function getTabConfig(benchmarkType: string): [TabConfig, TabConfig, TabConfig] {
  const type = benchmarkType.toLowerCase()

  if (type.includes('markdown_md') || type.includes('markdown_parse')) {
    return [
      { label: 'Markdown Input', icon: FileText, visible: true },
      { label: 'UCM Output', icon: TreePine, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  if (type.includes('markdown_render')) {
    return [
      { label: 'UCM Input', icon: TreePine, visible: true },
      { label: 'Markdown Output', icon: FileText, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  if (type.includes('ucl_parsing')) {
    return [
      { label: 'UCL Input', icon: FileText, visible: true },
      { label: 'Parsed Commands', icon: Code, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  if (type.includes('document_ops')) {
    return [
      { label: 'Initial State', icon: FileText, visible: true },
      { label: 'After Operation', icon: TreePine, visible: true },
      { label: 'Diff', icon: Code, visible: true },
    ]
  }

  if (type.includes('normalization')) {
    return [
      { label: 'Original Text', icon: FileText, visible: true },
      { label: 'Normalized', icon: TreePine, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  if (type.includes('json_content')) {
    return [
      { label: 'JSON Input', icon: FileText, visible: true },
      { label: 'Parsed Content', icon: TreePine, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  if (type.includes('table_ops')) {
    return [
      { label: 'Table Input', icon: FileText, visible: true },
      { label: 'Table Structure', icon: TreePine, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  if (type.includes('code_block')) {
    return [
      { label: 'Code Input', icon: FileText, visible: true },
      { label: 'Parsed Block', icon: TreePine, visible: true },
      { label: 'Output', icon: Code, visible: false },
    ]
  }

  // Default fallback
  return [
    { label: 'Input', icon: FileText, visible: true },
    { label: 'Structure', icon: TreePine, visible: true },
    { label: 'Output', icon: Code, visible: true },
  ]
}

function ResultDetailView({ result, viewTab, setViewTab }: {
  result: CoreTestResult
  viewTab: ViewTab
  setViewTab: (tab: ViewTab) => void
}) {
  const [tab1, tab2, tab3] = getTabConfig(result.benchmark_type)
  const tabs = [tab1, tab2, tab3].filter(t => t.visible)

  return (
    <div className="bg-white rounded-lg border border-gray-200">
      <div className="flex items-center border-b border-gray-200">
        {tabs.map((tab, idx) => {
          const tabKey = idx === 0 ? 'first' : idx === 1 ? 'second' : 'third'
          const Icon = tab.icon
          return (
            <button
              key={tabKey}
              onClick={() => setViewTab(tabKey as ViewTab)}
              className={`flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
                viewTab === tabKey
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              <Icon size={16} />
              {tab.label}
            </button>
          )
        })}
      </div>

      <div className="p-4">
        <div className="flex items-center justify-between mb-4">
          <h4 className="font-medium text-gray-900">{result.test_id}</h4>
          <div className="flex items-center gap-2 text-sm">
            {result.success ? (
              <span className="px-2 py-1 bg-green-100 text-green-700 rounded">Passed</span>
            ) : (
              <span className="px-2 py-1 bg-red-100 text-red-700 rounded">Failed</span>
            )}
          </div>
        </div>

        {/* Validation Checks */}
        {result.validation.checks.length > 0 && (
          <div className="mb-4">
            <h5 className="text-sm font-medium text-gray-700 mb-2">Validation</h5>
            <div className="flex flex-wrap gap-2">
              {result.validation.checks.map((check, idx) => (
                <span
                  key={idx}
                  className={`px-2 py-1 text-xs rounded ${
                    check.passed
                      ? 'bg-green-100 text-green-700'
                      : 'bg-red-100 text-red-700'
                  }`}
                >
                  {check.passed ? '✓' : '✗'} {check.name}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Performance */}
        <div className="mb-4">
          <h5 className="text-sm font-medium text-gray-700 mb-2">Performance</h5>
          <div className="grid grid-cols-2 gap-2 text-sm">
            <div>
              <span className="text-gray-500">Duration:</span>{' '}
              <span className="font-mono">{result.performance.duration_ms.toFixed(3)}ms</span>
            </div>
            <div>
              <span className="text-gray-500">Iterations:</span>{' '}
              <span className="font-mono">{result.performance.iterations}</span>
            </div>
            {result.performance.throughput_ops_sec && (
              <div>
                <span className="text-gray-500">Throughput:</span>{' '}
                <span className="font-mono">{result.performance.throughput_ops_sec.toFixed(0)} ops/s</span>
              </div>
            )}
          </div>
        </div>

        {/* Content based on tab */}
        <div className="bg-gray-50 rounded-lg p-4 font-mono text-sm overflow-x-auto">
          {viewTab === 'first' && (
            <pre className="whitespace-pre-wrap">{result.input_preview}</pre>
          )}
          {viewTab === 'second' && (
            <pre className="whitespace-pre-wrap">
              {result.structure_preview || 'No structure information available'}
            </pre>
          )}
          {viewTab === 'third' && (
            <pre className="whitespace-pre-wrap">{result.output_preview}</pre>
          )}
        </div>

        {result.error_message && (
          <div className="mt-4 p-3 bg-red-50 border border-red-200 rounded-lg">
            <h5 className="text-sm font-medium text-red-800 mb-1">Error</h5>
            <p className="text-sm text-red-700">{result.error_message}</p>
          </div>
        )}
      </div>
    </div>
  )
}

export default function CoreBenchmark() {
  const [categories, setCategories] = useState<Category[]>([])
  const [selectedCategories, setSelectedCategories] = useState<string[]>([])
  const [inputs, setInputs] = useState<Input[]>([])
  const [iterations, setIterations] = useState(100)
  const [benchmarkName, setBenchmarkName] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [currentReport, setCurrentReport] = useState<CoreBenchmarkReport | null>(null)
  const [history, setHistory] = useState<CoreBenchmarkReport[]>([])
  const [selectedResult, setSelectedResult] = useState<CoreTestResult | null>(null)
  const [viewTab, setViewTab] = useState<ViewTab>('first')
  const [expandedCategories, setExpandedCategories] = useState<Set<string>>(new Set())

  useEffect(() => {
    fetchCategories()
    fetchInputs()
    fetchHistory()
  }, [])

  useEffect(() => {
    if (currentReport && currentReport.status.running) {
      const interval = setInterval(() => {
        fetchCurrentRun(currentReport.report_id)
      }, 1000)
      return () => clearInterval(interval)
    }
  }, [currentReport])

  const fetchCategories = async () => {
    try {
      const res = await fetch('/api/core/categories')
      const data = await res.json()
      setCategories(data)
      setSelectedCategories(data.map((c: Category) => c.id))
    } catch (e) {
      console.error('Failed to fetch categories:', e)
    }
  }

  const fetchInputs = async () => {
    try {
      const res = await fetch('/api/core/inputs')
      const data = await res.json()
      setInputs(data)
    } catch (e) {
      console.error('Failed to fetch inputs:', e)
    }
  }

  const fetchHistory = async () => {
    try {
      const res = await fetch('/api/core/history')
      const data = await res.json()
      setHistory(data)
    } catch (e) {
      console.error('Failed to fetch history:', e)
    }
  }

  const fetchCurrentRun = async (reportId: string) => {
    try {
      const res = await fetch(`/api/core/runs/${reportId}`)
      const data = await res.json()
      setCurrentReport(data)
      if (data.status.completed || data.status.failed) {
        setIsRunning(false)
        fetchHistory()
      }
    } catch (e) {
      console.error('Failed to fetch run:', e)
    }
  }

  const startBenchmark = async () => {
    if (selectedCategories.length === 0) return
    
    setIsRunning(true)
    try {
      const res = await fetch('/api/core/runs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: benchmarkName || `Core Benchmark ${new Date().toLocaleString()}`,
          categories: selectedCategories,
          iterations,
        }),
      })
      const data = await res.json()
      fetchCurrentRun(data.report_id)
    } catch (e) {
      console.error('Failed to start benchmark:', e)
      setIsRunning(false)
    }
  }

  const loadReport = async (reportId: string) => {
    try {
      const res = await fetch(`/api/core/runs/${reportId}`)
      const data = await res.json()
      setCurrentReport(data)
      setSelectedResult(null)
    } catch (e) {
      console.error('Failed to load report:', e)
    }
  }

  const toggleCategory = (categoryId: string) => {
    setSelectedCategories(prev =>
      prev.includes(categoryId)
        ? prev.filter(c => c !== categoryId)
        : [...prev, categoryId]
    )
  }

  const toggleExpandCategory = (categoryId: string) => {
    setExpandedCategories(prev => {
      const next = new Set(prev)
      if (next.has(categoryId)) {
        next.delete(categoryId)
      } else {
        next.add(categoryId)
      }
      return next
    })
  }

  const getStatusIcon = (status: CoreBenchmarkReport['status']) => {
    if (!status) return null
    if (status.pending) return <Clock className="w-4 h-4 text-gray-400" />
    if (status.running) return <Loader2 className="w-4 h-4 text-blue-500 animate-spin" />
    if (status.completed) return <CheckCircle2 className="w-4 h-4 text-green-500" />
    if (status.failed) return <XCircle className="w-4 h-4 text-red-500" />
    return null
  }

  const inputsByCategory = inputs.reduce((acc, input) => {
    if (!acc[input.category]) acc[input.category] = []
    acc[input.category].push(input)
    return acc
  }, {} as Record<string, Input[]>)

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Core Benchmarking</h1>
          <p className="text-gray-500">Benchmark parsing, normalization, and document operations</p>
        </div>
        <button
          onClick={fetchHistory}
          className="flex items-center gap-2 px-3 py-2 text-sm text-gray-600 hover:text-gray-900"
        >
          <RefreshCw size={16} />
          Refresh
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Configuration Panel */}
        <div className="lg:col-span-1 space-y-4">
          <div className="bg-white rounded-lg border border-gray-200 p-4">
            <h2 className="font-semibold text-gray-900 mb-4">Configuration</h2>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Benchmark Name
                </label>
                <input
                  type="text"
                  value={benchmarkName}
                  onChange={(e) => setBenchmarkName(e.target.value)}
                  placeholder="Optional name"
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Iterations: {iterations}
                </label>
                <input
                  type="range"
                  min="10"
                  max="1000"
                  step="10"
                  value={iterations}
                  onChange={(e) => setIterations(Number(e.target.value))}
                  className="w-full"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Categories
                </label>
                <div className="space-y-2 max-h-64 overflow-y-auto">
                  {categories.map(cat => (
                    <div key={cat.id} className="border border-gray-200 rounded-lg">
                      <div className="flex items-center gap-2 p-2">
                        <button
                          onClick={() => toggleExpandCategory(cat.id)}
                          className="text-gray-400 hover:text-gray-600"
                        >
                          {expandedCategories.has(cat.id) ? (
                            <ChevronDown size={16} />
                          ) : (
                            <ChevronRight size={16} />
                          )}
                        </button>
                        <label className="flex items-center gap-2 flex-1 cursor-pointer">
                          <input
                            type="checkbox"
                            checked={selectedCategories.includes(cat.id)}
                            onChange={() => toggleCategory(cat.id)}
                            className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                          />
                          <span className="text-sm font-medium text-gray-700">{cat.name}</span>
                          <span className="text-xs text-gray-400">({cat.input_count})</span>
                        </label>
                      </div>
                      {expandedCategories.has(cat.id) && inputsByCategory[cat.id] && (
                        <div className="border-t border-gray-100 p-2 pl-8 space-y-1">
                          {inputsByCategory[cat.id].map(input => (
                            <div key={input.id} className="text-xs text-gray-500">
                              • {input.name} ({input.complexity})
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>

              <button
                onClick={startBenchmark}
                disabled={isRunning || selectedCategories.length === 0}
                className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isRunning ? (
                  <>
                    <Loader2 size={16} className="animate-spin" />
                    Running...
                  </>
                ) : (
                  <>
                    <Play size={16} />
                    Run Benchmark
                  </>
                )}
              </button>
            </div>
          </div>

          {/* History */}
          <div className="bg-white rounded-lg border border-gray-200 p-4">
            <h2 className="font-semibold text-gray-900 mb-4">History</h2>
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {history.length === 0 ? (
                <p className="text-sm text-gray-500">No benchmarks run yet</p>
              ) : (
                history.map(report => (
                  <button
                    key={report.report_id}
                    onClick={() => loadReport(report.report_id)}
                    className={`w-full text-left p-2 rounded-lg border transition-colors ${
                      currentReport?.report_id === report.report_id
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-200 hover:border-gray-300'
                    }`}
                  >
                    <div className="flex items-center gap-2">
                      {getStatusIcon(report.status)}
                      <span className="text-sm font-medium text-gray-900 truncate">
                        {report.name}
                      </span>
                    </div>
                    <div className="text-xs text-gray-500 mt-1">
                      {new Date(report.started_at).toLocaleString()}
                    </div>
                    <div className="text-xs text-gray-500">
                      {report.metrics ? `${report.metrics.passed}/${report.metrics.total_tests} passed` : 'Loading...'}
                      {report.metrics && ` • ${report.metrics.avg_duration_ms.toFixed(2)}ms avg`}
                    </div>
                  </button>
                ))
              )}
            </div>
          </div>
        </div>

        {/* Results Panel */}
        <div className="lg:col-span-2 space-y-4">
          {currentReport ? (
            <>
              {/* Summary */}
              <div className="bg-white rounded-lg border border-gray-200 p-4">
                <div className="flex items-center justify-between mb-4">
                  <h2 className="font-semibold text-gray-900">{currentReport.name}</h2>
                  {getStatusIcon(currentReport.status)}
                </div>

                {currentReport.status.running && (
                  <div className="mb-4">
                    <div className="flex items-center justify-between text-sm text-gray-600 mb-1">
                      <span>Running: {currentReport.status.running.current_test}</span>
                      <span>{Math.round(currentReport.status.running.progress * 100)}%</span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2">
                      <div
                        className="bg-blue-600 h-2 rounded-full transition-all"
                        style={{ width: `${currentReport.status.running.progress * 100}%` }}
                      />
                    </div>
                  </div>
                )}

                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <div className="text-center p-3 bg-gray-50 rounded-lg">
                    <div className="text-2xl font-bold text-gray-900">
                      {currentReport.metrics?.total_tests || 0}
                    </div>
                    <div className="text-xs text-gray-500">Total Tests</div>
                  </div>
                  <div className="text-center p-3 bg-green-50 rounded-lg">
                    <div className="text-2xl font-bold text-green-600">
                      {currentReport.metrics?.passed || 0}
                    </div>
                    <div className="text-xs text-gray-500">Passed</div>
                  </div>
                  <div className="text-center p-3 bg-red-50 rounded-lg">
                    <div className="text-2xl font-bold text-red-600">
                      {currentReport.metrics?.failed || 0}
                    </div>
                    <div className="text-xs text-gray-500">Failed</div>
                  </div>
                  <div className="text-center p-3 bg-blue-50 rounded-lg">
                    <div className="text-2xl font-bold text-blue-600">
                      {currentReport.metrics?.avg_duration_ms?.toFixed(2) || '0.00'}ms
                    </div>
                    <div className="text-xs text-gray-500">Avg Duration</div>
                  </div>
                </div>

                {currentReport.status?.completed && currentReport.metrics && (
                  <div className="mt-4 grid grid-cols-3 gap-4 text-sm">
                    <div>
                      <span className="text-gray-500">P50:</span>{' '}
                      <span className="font-medium">{currentReport.metrics.p50_duration_ms.toFixed(2)}ms</span>
                    </div>
                    <div>
                      <span className="text-gray-500">P95:</span>{' '}
                      <span className="font-medium">{currentReport.metrics.p95_duration_ms.toFixed(2)}ms</span>
                    </div>
                    <div>
                      <span className="text-gray-500">P99:</span>{' '}
                      <span className="font-medium">{currentReport.metrics.p99_duration_ms.toFixed(2)}ms</span>
                    </div>
                  </div>
                )}
              </div>

              {/* Results List */}
              <div className="bg-white rounded-lg border border-gray-200">
                <div className="p-4 border-b border-gray-200">
                  <h3 className="font-semibold text-gray-900">Test Results</h3>
                </div>
                <div className="divide-y divide-gray-100 max-h-96 overflow-y-auto">
                  {currentReport.results.map(result => (
                    <button
                      key={result.test_id}
                      onClick={() => setSelectedResult(result)}
                      className={`w-full text-left p-3 hover:bg-gray-50 transition-colors ${
                        selectedResult?.test_id === result.test_id ? 'bg-blue-50' : ''
                      }`}
                    >
                      <div className="flex items-center gap-2">
                        {result.success ? (
                          <CheckCircle2 className="w-4 h-4 text-green-500" />
                        ) : (
                          <XCircle className="w-4 h-4 text-red-500" />
                        )}
                        <span className="font-medium text-gray-900">{result.test_id}</span>
                        <span className="text-xs text-gray-400 ml-auto">
                          {result.performance.duration_ms.toFixed(3)}ms
                        </span>
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        {result.category} • {result.benchmark_type}
                        {result.performance.throughput_ops_sec && (
                          <> • {result.performance.throughput_ops_sec.toFixed(0)} ops/s</>
                        )}
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              {/* Detail View */}
              {selectedResult && (
                <ResultDetailView
                  result={selectedResult}
                  viewTab={viewTab}
                  setViewTab={setViewTab}
                />
              )}
            </>
          ) : (
            <div className="bg-white rounded-lg border border-gray-200 p-8 text-center">
              <div className="text-gray-400 mb-4">
                <Play size={48} className="mx-auto" />
              </div>
              <h3 className="text-lg font-medium text-gray-900 mb-2">No Benchmark Selected</h3>
              <p className="text-gray-500">
                Configure and run a benchmark, or select one from history.
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
