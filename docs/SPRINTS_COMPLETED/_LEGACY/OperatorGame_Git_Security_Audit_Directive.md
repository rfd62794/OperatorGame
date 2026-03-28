# OperatorGame Git Security Audit & History Purge Directive

**Status:** Pre-Public Release Security Hardening  
**Target:** Remove any leaked secrets, API keys, keystores, credentials from git history  
**Scope:** Full commit history scan, selective history rewrite, final validation

---

## Critical: What to Scan For

Before any public push, the agent must verify NO sensitive data exists in git history:

### 1. Android Keystores & Signing Keys
- `*.jks` files (even if deleted, check history)
- `*.keystore`, `*.p12`, `*.pfx` files
- Pattern: `storePassword=`, `keyPassword=`
- Lines containing keystore paths with actual passwords

**Check command:**
```powershell
git log -p --all | Select-String -Pattern "\.jks|\.keystore|storePassword|keyPassword" -Context 3
```

### 2. API Keys / Tokens
- OpenRouter API keys (format: `sk-*`)
- Firebase keys
- Google Cloud keys
- Pattern: `api[_-]?key\s*[=:]\s*[''"]?[a-zA-Z0-9]{32,}`

**Check command:**
```powershell
git log -p --all | Select-String -Pattern "api.key|sk-|firebase|gcloud" -Context 2
```

### 3. Database Credentials
- Firebase connection strings
- SQL passwords
- Pattern: `password\s*[=:]\s*[''"][^''\"]*[''"]`

**Check command:**
```powershell
git log -p --all | Select-String -Pattern "password|password_hash|db_password" -Context 2
```

### 4. Android NDK / SDK Paths with Credentials
- `local.properties` should NEVER be in history with actual paths
- Pattern: `sdk.dir=`, `ndk.dir=` with personal paths

**Check command:**
```powershell
git log --all --name-status | Select-String -Pattern "local.properties"
```

### 5. Private Keys (PEM/RSA)
- `BEGIN RSA PRIVATE KEY`
- `BEGIN PRIVATE KEY`
- `BEGIN OPENSSH PRIVATE KEY`

**Check command:**
```powershell
git log -p --all | Select-String -Pattern "BEGIN.*PRIVATE KEY|-----"
```

---

## Part A: Run Security Scan (Read-Only)

**Agent Action:**

```powershell
cd C:\Github\OperatorGame

Write-Host "Security Scan: Checking commit history for secrets..." -ForegroundColor Yellow

# Scan 1: Keystores
Write-Host "`n[1/5] Scanning for Android keystores..." -ForegroundColor Cyan
git log -p --all | Select-String -Pattern "\.jks|\.keystore|storePassword|keyPassword" -Context 3

# Scan 2: API Keys
Write-Host "`n[2/5] Scanning for API keys..." -ForegroundColor Cyan
git log -p --all | Select-String -Pattern "api.key|sk-|openrouter|firebase|gcloud" -Context 2

# Scan 3: Passwords
Write-Host "`n[3/5] Scanning for password patterns..." -ForegroundColor Cyan
git log -p --all | Select-String -Pattern "password\s*[=:]\s*[''"]" -Context 2

# Scan 4: Local Properties
Write-Host "`n[4/5] Scanning for local.properties in history..." -ForegroundColor Cyan
git log --all --name-status | Select-String -Pattern "local.properties"

# Scan 5: Private Keys
Write-Host "`n[5/5] Scanning for private keys..." -ForegroundColor Cyan
git log -p --all | Select-String -Pattern "BEGIN.*PRIVATE KEY" -Context 3

Write-Host "`nScan complete. Review output above for any matches." -ForegroundColor Green
```

**Report findings back to Robert.** If ANY secrets found, proceed to Part B. If clean, skip to Part C.

---

## Part B: Purge Secrets from History (If Needed)

**ONLY execute if Part A found secrets.**

### Option B1: Surgical File Removal (Recommended)

If `local.properties` or `.jks` files were committed:

```powershell
# Remove file from entire history (BFG Repo-Cleaner approach)
# Requires BFG: https://rtyley.github.io/bfg-repo-cleaner/

# Download BFG to repo root, then:
java -jar bfg.jar --delete-files local.properties --no-blob-protection C:\Github\OperatorGame

# Or use git filter-branch (slower, but no dependencies):
git filter-branch --tree-filter 'rm -f local.properties' -- --all

# Force push (DESTRUCTIVE - only if repo is private or you own it)
git push origin --force-with-lease --all
```

### Option B2: Rewrite Specific Commit

If a single commit contains secrets:

```powershell
# Find the commit hash containing secrets
git log --oneline | grep -i "commit message containing secret reference"

# Rebase interactively up to that commit
git rebase -i <commit-hash>~1

# Mark the commit as 'edit', save, then:
git rm <secretive-file>
git commit --amend --no-edit
git rebase --continue

# Force push
git push origin --force-with-lease
```

### Option B3: Remove Sensitive Content from Commit (Text Inline)

If secrets are inline (e.g., `password=secret123` in a config file):

```powershell
git filter-branch --tree-filter '
  find . -type f -name "*.properties" -o -name "*.gradle" | xargs sed -i "s/password=[^[:space:]]*/password=REDACTED/g"
' -- --all

git push origin --force-with-lease --all
```

**WARNING:** Force pushing rewrites history. Only safe on private repos or repos you control. **Coordinate with any collaborators.**

---

## Part C: Final .gitignore Validation

**Agent Action:**

Verify .gitignore blocks future leaks:

```powershell
cd C:\Github\OperatorGame

# Check current .gitignore
Write-Host "Current .gitignore entries (sensitive sections):" -ForegroundColor Yellow
Select-String -Path .gitignore -Pattern "keystore|password|local\.properties|\.jks|api.key|secret"

# If any missing, append:
@"

# Security: Keystores & Signing Keys
*.jks
*.keystore
*.p12
*.pfx
local.properties

# Security: API Keys & Credentials
.env
.env.local
*.key
*.secret
secrets/

"@ | Add-Content .gitignore

git add .gitignore
git commit -m "security: Strengthen .gitignore to prevent credential leakage"
git push origin main
```

---

## Part D: Verify Clean History

**Agent Action:**

```powershell
# Final audit: confirm no secrets remain
Write-Host "Final Security Validation..." -ForegroundColor Yellow

$secretPatterns = @(
    "api[_-]?key",
    "password",
    "BEGIN.*PRIVATE",
    "\.jks",
    "keystore",
    "sk-"
)

$foundIssues = @()
foreach ($pattern in $secretPatterns) {
    $result = git log -p --all | Select-String -Pattern $pattern -Context 1 | Measure-Object | Select-Object -ExpandProperty Count
    if ($result -gt 0) {
        $foundIssues += $pattern
    }
}

if ($foundIssues.Count -eq 0) {
    Write-Host "✅ PASS: No secrets found in commit history" -ForegroundColor Green
} else {
    Write-Host "❌ FAIL: Found patterns still in history: $($foundIssues -join ', ')" -ForegroundColor Red
    Write-Host "Execute Part B cleanup before public push" -ForegroundColor Yellow
}

# Show last 5 commits (should all be public-safe)
Write-Host "`nLast 5 commits:" -ForegroundColor Cyan
git log --oneline -5
```

---

## Part E: Review .gitignore Coverage

**Agent Action:**

Verify these entries exist in `.gitignore`:

```
# Android keystores & signing
*.jks
*.keystore
*.p12
*.pfx
local.properties

# Environment & secrets
.env
.env.local
.env.*.local
secrets/

# Sensitive config
gradle.properties (if contains passwords)

# PowerShell artifacts (from new tooling)
screenshots_*/
ui_coordinates.json

# Build artifacts (prevent accidental binary inclusion)
target/
build/
dist/
*.apk
*.aab
```

If any are missing, append them to `.gitignore` and commit.

---

## Part F: Final Commit & Report

**Agent Action:**

```powershell
cd C:\Github\OperatorGame

# Verify clean working tree
git status

# Expected output: "nothing to commit, working tree clean"
# OR a single commit ready to push

# If changes made, commit:
git add .gitignore
git commit -m "security: Final .gitignore hardening for public release"

# Report to Robert:
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  Security Audit Complete                                   ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "✅ Commit History Scanned: No secrets found"
Write-Host "✅ .gitignore Hardened: Keystores, credentials blocked"
Write-Host "✅ Working Tree Clean: Ready for git push"
Write-Host ""
Write-Host "Current status:"
git log --oneline -3
Write-Host ""
Write-Host "Ready for: git push origin main"
```

---

## Success Criteria

- [ ] Part A: Security scan completed (report findings)
- [ ] Part B: If secrets found, history purged and force-pushed
- [ ] Part C: .gitignore validated and strengthened
- [ ] Part D: Final audit shows zero secrets in history
- [ ] Part E: All sensitive patterns covered in .gitignore
- [ ] Part F: Working tree clean, commits ready for push

---

## Critical Notes for Agent

1. **Do not proceed with `git push origin main` until Part D passes**
2. If Part A finds ANY secrets, report them to Robert BEFORE executing Part B
3. Part B involves force-pushing history — only do this if Robert approves
4. Always run `git status` before final push to verify clean state
5. If uncertain about any finding, ask Robert before taking action

---

## Report Template (For Agent to Return to Robert)

```
╔════════════════════════════════════════════════════════════╗
║  Git Security Audit Report                                 ║
╚════════════════════════════════════════════════════════════╝

SCAN RESULTS:
[Part A findings here - list any patterns found or "CLEAN"]

REMEDIATION TAKEN:
[Part B actions if any, or "Not required - history clean"]

GITIGNORE STATUS:
[List entries added or verified]

FINAL VALIDATION:
[Part D result - PASS or FAIL with details]

LAST 5 COMMITS:
[git log --oneline -5 output]

READY FOR PUSH: [YES/NO]
Next command: git push origin main
```

---

**Execute all parts in order. Do not skip to push without completing Part D validation.**
