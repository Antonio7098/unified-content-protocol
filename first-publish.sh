#!/bin/bash
set -e

CRATES=(
  "ucm-core"
  "ucp-observe" 
  "ucp-translator-markdown"
  "ucp-translator-html"
  "ucm-engine"
  "ucl-parser"
  "ucp-llm"
  "ucp-agent"
  "ucp-cli"
)

echo "================================"
echo "UCP First-Time Publishing"
echo "================================"
echo ""
echo "This will publish all 9 crates to crates.io"
echo "Press Ctrl+C to cancel, or wait 3 seconds to continue..."
sleep 3

echo ""
echo "Publishing all crates..."
echo ""

for crate in "${CRATES[@]}"; do
  echo "--------------------------------"
  echo "Publishing: $crate"
  echo "--------------------------------"
  
  if cargo publish -p "$crate" 2>&1; then
    echo "✅ $crate published successfully"
  else
    echo "❌ $crate failed to publish"
    echo "Check error above. If the crate already exists, that's OK."
    echo "Continuing to next crate..."
  fi
  
  if [ "$crate" != "ucp-cli" ]; then
    echo ""
    echo "Waiting 30 seconds for crates.io to index..."
    sleep 30
  fi
  echo ""
done

echo "================================"
echo "Publishing Complete!"
echo "================================"
echo ""
echo "Verify your crates:"
for crate in "${CRATES[@]}"; do
  echo "  https://crates.io/crates/$crate"
done
echo ""
echo "Next steps:"
echo "  1. Visit the URLs above to confirm crates exist"
echo "  2. Set up GitHub Actions secret: CARGO_REGISTRY_TOKEN"
echo "  3. Push this branch to GitHub"
echo "  4. Future releases: git push origin v0.1.11"
