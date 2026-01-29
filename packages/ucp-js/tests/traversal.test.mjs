import test from 'node:test';
import assert from 'node:assert/strict';
import { TraversalEngine } from '../dist/traversal.js';

function buildDocument(rootId, layout) {
  const blocks = new Map();
  for (const [id, spec] of Object.entries(layout)) {
    blocks.set(id, {
      id,
      content: spec.content ?? '',
      children: [...(spec.children ?? [])],
      edges: (spec.edges ?? []).map(target => ({ target, edgeType: spec.edgeType ?? 'references' })),
      metadata: spec.metadata ?? {},
    });
  }
  if (!blocks.has(rootId)) {
    blocks.set(rootId, {
      id: rootId,
      content: '',
      children: [],
      edges: [],
      metadata: {},
    });
  }
  return {
    root: rootId,
    blocks,
  };
}

test('findPaths discovers sibling paths via shared parent', () => {
  const doc = buildDocument('root', {
    root: { children: ['intro_overview', 'intro_details'] },
    intro_overview: { children: [] },
    intro_details: { children: [] },
  });

  const engine = new TraversalEngine();
  const paths = engine.findPaths(doc, 'intro_overview', 'intro_details');

  assert.ok(paths.length > 0, 'should return at least one path');
  assert.ok(
    paths.some(
      path =>
        path[0] === 'intro_overview' &&
        path.at(-1) === 'intro_details' &&
        path.includes('root'),
    ),
    'path should travel through the shared parent',
  );
});

test('navigate throws for unknown starting block IDs', () => {
  const doc = buildDocument('root', {
    root: { children: [] },
  });
  const engine = new TraversalEngine();

  assert.throws(() => engine.navigate(doc, 'missing_block'), /Block not found/);
});
