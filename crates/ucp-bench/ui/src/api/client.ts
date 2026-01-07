const API_BASE = '/api'

export interface Category {
  id: string
  name: string
  description: string
  command_type: string
  tags: string[]
  enabled_by_default: boolean
  complexity: number
  test_count: number
}

export interface Provider {
  id: string
  name: string
  available: boolean
  models: Model[]
}

export interface Model {
  id: string
  name: string
  context_length: number
}

export interface Suite {
  id: string
  name: string
  description: string
  categories: string[]
  config: SuiteConfig
  created_at: string
}

export interface SuiteConfig {
  matrix: { pairs: ProviderModelPair[] }
  concurrency: number
  execute_commands: boolean
  capture_debug_info: boolean
  capture_document_snapshots: boolean
}

export interface ProviderModelPair {
  provider_id: string
  model_id: string
  enabled: boolean
}

export interface Run {
  run_id: string
  suite_id: string
  started_at: string
  completed_at: string | null
  status: RunStatus
  results_by_provider: Record<string, ProviderResults>
  summary: RunSummary
}

export type RunStatus = 
  | 'pending'
  | { running: { progress: number; current_test: string } }
  | 'completed'
  | { failed: { error: string } }
  | 'cancelled'

export interface ProviderResults {
  provider_id: string
  model_id: string
  results_by_category: Record<string, CategoryResults>
  total_tests: number
  passed: number
  failed: number
  total_cost_usd: number
  avg_latency_ms: number
}

export interface CategoryResults {
  category_id: string
  tests: TestResult[]
  passed: number
  failed: number
  success_rate: number
  avg_latency_ms: number
  total_cost_usd: number
}

export interface TestResult {
  test_id: string
  category_id: string
  provider_id: string
  model_id: string
  executed_at: string
  latency_ms: number
  input_tokens: number
  output_tokens: number
  cost_usd: number
  success: boolean
  parse_success: boolean
  execute_success: boolean
  semantic_score: number
  efficiency_score: number
  error: TestError | null
  context: ExecutionContext
  document_before: DocumentSnapshot | null
  document_after: DocumentSnapshot | null
  diff: DocumentDiff | null
}

export interface TestError {
  category: string
  message: string
  details: Record<string, string> | null
}

export interface ExecutionContext {
  test_description: string
  task_prompt: string
  system_prompt: string
  full_user_prompt: string
  raw_response: string
  extracted_ucl: string
  parsed_command: string | null
  expected_pattern: string | null
  pattern_matched: boolean | null
}

export interface DocumentSnapshot {
  captured_at: string
  block_count: number
  blocks: Record<string, BlockSnapshot>
}

export interface BlockSnapshot {
  id: string
  content_type: string
  content_preview: string
  label: string | null
  parent_id: string | null
  children_count: number
}

export interface DocumentDiff {
  added_blocks: string[]
  removed_blocks: string[]
  modified_blocks: BlockModification[]
  summary: string
}

export interface BlockModification {
  block_id: string
  field: string
  before: string
  after: string
}

export interface RunSummary {
  total_tests: number
  total_passed: number
  total_failed: number
  overall_success_rate: number
  total_cost_usd: number
  total_duration_ms: number
}

export interface ValidationCriteria {
  must_parse: boolean
  must_execute: boolean
  expected_command?: string | null
  target_block_id?: string | null
  forbidden_patterns: string[]
}

export interface TestCase {
  id: string
  command_type: string
  description: string
  prompt: string
  expected_pattern?: string | null
  validation: ValidationCriteria
}

export interface TestDetailResponse {
  test: TestCase
  category?: Category
  document?: { description: string }
}

export interface DocumentResponse {
  ucm: unknown
  description: string
  snapshot: DocumentSnapshot
}

// API functions
export async function fetchCategories(): Promise<Category[]> {
  const res = await fetch(`${API_BASE}/registry/categories`)
  return res.json()
}

export async function fetchProviders(): Promise<Provider[]> {
  const res = await fetch(`${API_BASE}/providers/available`)
  return res.json()
}

export async function fetchSuites(): Promise<Suite[]> {
  const res = await fetch(`${API_BASE}/suites`)
  return res.json()
}

export async function createSuite(data: {
  name: string
  description?: string
  categories: string[]
  config?: Partial<SuiteConfig>
}): Promise<Suite> {
  const res = await fetch(`${API_BASE}/suites`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  })
  return res.json()
}

export async function fetchRuns(): Promise<Run[]> {
  const res = await fetch(`${API_BASE}/runs`)
  return res.json()
}

export async function fetchRun(runId: string): Promise<Run> {
  const res = await fetch(`${API_BASE}/runs/${runId}`)
  return res.json()
}

export async function startRun(suiteId: string, configOverride?: Partial<SuiteConfig>): Promise<{ run_id: string }> {
  const res = await fetch(`${API_BASE}/runs`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ suite_id: suiteId, config_override: configOverride }),
  })
  return res.json()
}

export async function fetchTestResult(runId: string, testId: string): Promise<TestResult> {
  const res = await fetch(`${API_BASE}/runs/${runId}/results/${testId}`)
  return res.json()
}

export async function fetchDocument(): Promise<DocumentResponse> {
  const res = await fetch(`${API_BASE}/document`)
  return res.json()
}

export async function fetchTests(): Promise<TestCase[]> {
  const res = await fetch(`${API_BASE}/tests`)
  return res.json()
}

export async function fetchTest(id: string): Promise<TestDetailResponse> {
  const res = await fetch(`${API_BASE}/tests/${id}`)
  return res.json()
}

// Playground types
export interface PlaygroundDocument {
  id: string
  name: string
  summary: string
  tags: string[]
}

export interface PlaygroundDocumentDetail {
  summary: PlaygroundDocument
  llm_description: string
  snapshot: DocumentSnapshot
  ucm: unknown
}

export interface PlaygroundChatRequest {
  document_id: string
  provider_id: string
  model_id: string
  message: string
  execute_commands: boolean
}

export interface PlaygroundChatResponse {
  message_id: string
  timestamp: string
  user_message: string
  full_prompt: string
  raw_response: string
  extracted_ucl: string
  parsed_commands: string[]
  parse_success: boolean
  execute_success: boolean | null
  document_before: unknown | null
  document_after: unknown | null
  diff: unknown | null
  error: string | null
  latency_ms: number
  input_tokens: number
  output_tokens: number
}

// Playground API functions
export async function fetchPlaygroundDocuments(): Promise<PlaygroundDocument[]> {
  const res = await fetch(`${API_BASE}/playground/documents`)
  return res.json()
}

export async function fetchPlaygroundDocument(id: string): Promise<PlaygroundDocumentDetail> {
  const res = await fetch(`${API_BASE}/playground/documents/${id}`)
  return res.json()
}

export async function sendPlaygroundChat(request: PlaygroundChatRequest): Promise<PlaygroundChatResponse> {
  const res = await fetch(`${API_BASE}/playground/chat`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(request),
  })
  return res.json()
}
