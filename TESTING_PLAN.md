# Testing Plan: Keyring Credential Management

## Pre-Test Setup

```bash
# Ensure clean state
cd ~/github/rubrix

# Clear any existing keychain entry
# macOS:
security delete-generic-password -s rubrix -a aws_bedrock_token 2>/dev/null || true

# Remove any .env file
rm -f src-tauri/.env

# Clean build
npm install
cd src-tauri && cargo clean && cd ..
```

## Test Suite 1: Development Mode

### Test 1.1: Dev Mode Detection
**Objective**: Verify dev mode is properly detected

```bash
npm run tauri dev
```

**Expected**:
- [ ] App launches successfully
- [ ] Header shows "🔧 DEV MODE" badge
- [ ] Badge is visible and styled correctly

**Pass Criteria**: Dev mode badge appears

---

### Test 1.2: First Time Login
**Objective**: Verify authentication and keychain storage

**Steps**:
1. Run `npm run tauri dev`
2. Click "Generate"
3. Enter credentials when prompted
4. Complete authentication

**Expected**:
- [ ] Login modal appears
- [ ] Authentication succeeds
- [ ] Login modal closes
- [ ] Header shows "🔐 Authenticated" badge
- [ ] Console logs: "INFO: Saved token to keychain"

**Verify keychain** (macOS):
```bash
security find-generic-password -s rubrix -a aws_bedrock_token -w
# Should output your token
```

**Pass Criteria**: Token stored in keychain, authentication works

---

### Test 1.3: App Restart (Cached Token)
**Objective**: Verify keychain token is loaded automatically

**Steps**:
1. Close app (Cmd+Q or exit dev server)
2. Restart: `npm run tauri dev`
3. Click "Generate" immediately

**Expected**:
- [ ] No login modal appears
- [ ] Header shows "🔐 Authenticated" immediately
- [ ] Console logs: "INFO: Loaded token from keychain (length=XXX)"
- [ ] Generation works immediately

**Pass Criteria**: No login required on second run

---

### Test 1.4: Logout Clears Keychain
**Objective**: Verify logout clears both memory and keychain

**Steps**:
1. Run app (already authenticated from Test 1.3)
2. Click "Logout" button in header
3. Close app
4. Restart app
5. Try to generate

**Expected**:
- [ ] After logout: "🔐 Authenticated" badge disappears
- [ ] Console logs: "INFO: Cleared token from keychain"
- [ ] On restart: Login modal appears
- [ ] Keychain is empty

**Verify keychain** (macOS):
```bash
security find-generic-password -s rubrix -a aws_bedrock_token
# Should error: "The specified item could not be found in the keychain"
```

**Pass Criteria**: Keychain cleared, login required again

---

### Test 1.5: .env Override
**Objective**: Verify DEV_AWS_TOKEN is used and cached

**Steps**:
1. Ensure logged out (clear keychain if needed)
2. Create `.env`:
   ```bash
   echo "DEV_AWS_TOKEN=test-token-123" > src-tauri/.env
   ```
3. Restart app
4. Click "Generate"

**Expected**:
- [ ] No login modal appears
- [ ] Console logs: "INFO: Using DEV_AWS_TOKEN from environment"
- [ ] Console logs: "INFO: Saved token to keychain"
- [ ] Generation attempts (may fail if token is invalid, that's ok)

**Verify keychain**:
```bash
security find-generic-password -s rubrix -a aws_bedrock_token -w
# Should output: test-token-123
```

**Steps to verify persistence**:
1. Delete .env: `rm src-tauri/.env`
2. Restart app
3. Click "Generate"

**Expected**:
- [ ] Token still works (loaded from keychain now)
- [ ] Console logs: "INFO: Loaded token from keychain"

**Pass Criteria**: .env token cached to keychain successfully

---

### Test 1.6: Mock Mode Fallback
**Objective**: Verify mock mode works when no credentials available

**Steps**:
1. Logout (clear keychain)
2. Remove .env: `rm src-tauri/.env`
3. Unset env var: `unset AWS_BEARER_TOKEN_BEDROCK`
4. Restart app
5. Click "Generate" (but cancel login modal)

**Expected**:
- [ ] Login modal appears
- [ ] Click "Cancel" to dismiss
- [ ] Try generate again without logging in
- [ ] Console logs: "INFO: Falling back to mock mode in dev"
- [ ] Mock questions appear (deterministic test data)

**Pass Criteria**: Mock mode activates without credentials

---

## Test Suite 2: Release Mode

### Test 2.1: Release Build
**Objective**: Verify release mode behavior

```bash
npm run tauri build
```

**Expected**:
- [ ] Build completes successfully
- [ ] Binary created in `src-tauri/target/release/`

**Pass Criteria**: Build succeeds

---

### Test 2.2: Release Mode Detection
**Objective**: Verify no dev mode features in release

**Steps**:
1. Run release binary: `./src-tauri/target/release/Rubrix`
2. Observe UI

**Expected**:
- [ ] App launches
- [ ] NO "🔧 DEV MODE" badge
- [ ] App looks normal

**Pass Criteria**: Dev mode badge absent

---

### Test 2.3: Release Authentication Required
**Objective**: Verify authentication always required in release

**Steps**:
1. Ensure you have a valid token in keychain (from dev tests)
2. Run release binary
3. Try to generate

**Expected**:
- [ ] Login modal appears immediately
- [ ] Keychain is NOT checked (even though token exists)
- [ ] Must authenticate to proceed

**Pass Criteria**: Authentication required despite keychain token

---

### Test 2.4: Release Token Not Persisted
**Objective**: Verify token cleared on app close

**Steps**:
1. Run release binary
2. Authenticate successfully
3. Generate questions (verify works)
4. Close app
5. Restart release binary
6. Try to generate

**Expected**:
- [ ] First run: Authentication works
- [ ] First run: Generation works
- [ ] Second run: Login modal appears again
- [ ] Token not persisted

**Pass Criteria**: Must re-authenticate every session in release

---

### Test 2.5: Release Logout
**Objective**: Verify logout in release mode

**Steps**:
1. Run release binary
2. Authenticate
3. Click "Logout"
4. Hover over logout button before clicking

**Expected**:
- [ ] Tooltip shows "Clears session" (not "Clears keychain cache")
- [ ] After logout: "🔐 Authenticated" badge disappears
- [ ] Can't generate without re-authenticating

**Pass Criteria**: Logout works, tooltip correct

---

### Test 2.6: Release No Keychain Access
**Objective**: Verify keychain never accessed in release

**Steps**:
1. Check terminal output while running release binary
2. Look for keychain-related logs

**Expected**:
- [ ] No "Loaded token from keychain" logs
- [ ] No "Saved token to keychain" logs
- [ ] No keychain errors

**Pass Criteria**: No keychain interaction in release

---

## Test Suite 3: Security Validation

### Test 3.1: No Secrets in Binary
**Objective**: Verify no tokens bundled in release binary

```bash
# macOS/Linux
strings src-tauri/target/release/Rubrix | grep -i "token\|secret\|aws" | grep -v "AWS_BEARER" | head -20

# Should not show any actual token values
```

**Expected**:
- [ ] No actual token strings found
- [ ] Only variable names like "DEV_AWS_TOKEN" might appear
- [ ] No hardcoded credentials

**Pass Criteria**: No credentials in binary

---

### Test 3.2: .env Not Bundled
**Objective**: Verify .env file not included in release

```bash
# Create a test .env with distinctive content
echo "SECRET_TEST_VALUE=should-not-be-bundled" > src-tauri/.env

# Rebuild
npm run tauri build

# Search binary
strings src-tauri/target/release/Rubrix | grep "SECRET_TEST_VALUE"
# Should return nothing

# Clean up
rm src-tauri/.env
```

**Expected**:
- [ ] Test string not found in binary

**Pass Criteria**: .env not bundled

---

### Test 3.3: Gitignore Protection
**Objective**: Verify .env files can't be committed

```bash
# Create .env with test content
echo "DEV_AWS_TOKEN=test" > src-tauri/.env

# Check git status
git status --short

# Should NOT show src-tauri/.env
```

**Expected**:
- [ ] `.env` file not listed in `git status`
- [ ] File is gitignored

**Pass Criteria**: .env is gitignored

---

## Test Suite 4: Edge Cases

### Test 4.1: Keychain Access Denied
**Objective**: Handle keychain errors gracefully

**Steps** (macOS):
1. Try to deny keychain access via System Preferences if possible
2. Or simulate by checking logs when keychain fails

**Expected**:
- [ ] App doesn't crash
- [ ] Falls back to next token source
- [ ] Console logs: "WARN: Keychain access error"

**Pass Criteria**: Graceful degradation

---

### Test 4.2: Malformed Token
**Objective**: Handle invalid tokens

**Steps**:
1. Create .env with invalid token:
   ```bash
   echo "DEV_AWS_TOKEN=invalid" > src-tauri/.env
   ```
2. Restart app
3. Try to generate

**Expected**:
- [ ] Token is used (logged)
- [ ] API request fails gracefully
- [ ] Error message shown to user
- [ ] App doesn't crash

**Pass Criteria**: Invalid token handled gracefully

---

### Test 4.3: Concurrent Keychain Access
**Objective**: Handle multiple keychain operations

**Steps**:
1. Login
2. Immediately logout
3. Immediately login again

**Expected**:
- [ ] No race conditions
- [ ] All operations complete
- [ ] Final state is correct (authenticated)

**Pass Criteria**: No crashes or inconsistent state

---

## Test Suite 5: Cross-Platform (Optional)

### Test 5.1: Windows Credential Manager
**Platform**: Windows

**Steps**:
1. Run dev build on Windows
2. Authenticate
3. Open Credential Manager
4. Search for "rubrix"

**Expected**:
- [ ] Entry exists with service "rubrix"
- [ ] Token stored encrypted

---

### Test 5.2: Linux Secret Service
**Platform**: Linux

**Steps**:
1. Install `secret-tool`: `sudo apt install libsecret-tools`
2. Run dev build
3. Authenticate
4. Check: `secret-tool search service rubrix`

**Expected**:
- [ ] Entry found
- [ ] Token stored

---

## Regression Tests

### Regression 1: Existing Functionality
**Objective**: Verify nothing broke

**Tests**:
- [ ] Question generation still works
- [ ] Export to QTI works
- [ ] Edit questions works
- [ ] Regenerate question works
- [ ] Bank editor works
- [ ] Subject/topic selection works

---

### Regression 2: Lambda Auth Still Works
**Objective**: Verify Lambda authentication unchanged

**Steps**:
1. Clear all cached tokens
2. Authenticate with real credentials
3. Verify questions generate correctly

**Expected**:
- [ ] Lambda auth flow identical to before
- [ ] API key retrieved correctly
- [ ] Questions generate successfully

---

## Success Criteria

### Must Pass (Critical)
- ✅ Test 1.1: Dev mode detection
- ✅ Test 1.2: First time login
- ✅ Test 1.3: Cached token on restart
- ✅ Test 1.4: Logout clears keychain
- ✅ Test 2.2: Release mode detection
- ✅ Test 2.3: Release requires auth
- ✅ Test 2.4: Release token not persisted
- ✅ Test 3.1: No secrets in binary
- ✅ Test 3.3: Gitignore protection
- ✅ Regression 1: Existing functionality
- ✅ Regression 2: Lambda auth works

### Should Pass (Important)
- ⚠️ Test 1.5: .env override
- ⚠️ Test 1.6: Mock mode fallback
- ⚠️ Test 2.5: Release logout
- ⚠️ Test 3.2: .env not bundled
- ⚠️ Test 4.1: Keychain errors handled
- ⚠️ Test 4.2: Invalid token handled

### Nice to Have (Optional)
- 💡 Test 4.3: Concurrent access
- 💡 Test 5.1: Windows credential manager
- 💡 Test 5.2: Linux secret service

## Test Results Template

```
Date: ____________
Tester: ____________
Platform: macOS / Windows / Linux
Build: Debug / Release

Test Results:
[ ] Test 1.1: Dev mode detection
[ ] Test 1.2: First time login
[ ] Test 1.3: Cached token on restart
[ ] Test 1.4: Logout clears keychain
[ ] Test 1.5: .env override
[ ] Test 1.6: Mock mode fallback
[ ] Test 2.1: Release build
[ ] Test 2.2: Release mode detection
[ ] Test 2.3: Release authentication required
[ ] Test 2.4: Release token not persisted
[ ] Test 2.5: Release logout
[ ] Test 2.6: Release no keychain access
[ ] Test 3.1: No secrets in binary
[ ] Test 3.2: .env not bundled
[ ] Test 3.3: Gitignore protection
[ ] Regression 1: Existing functionality
[ ] Regression 2: Lambda auth works

Issues Found:
_______________________________________
_______________________________________
_______________________________________

Overall Status: PASS / FAIL / NEEDS WORK
```

## Automated Testing (Future)

Consider adding:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_mode_detection() {
        // In debug build, should be true
        assert_eq!(cfg!(debug_assertions), true);
    }

    #[test]
    fn test_token_resolution_hierarchy() {
        // Test token priority order
    }
}
```

## Performance Benchmarks

- Keychain read: < 100ms
- Keychain write: < 100ms
- Auth flow total: < 2s (network dependent)

## Documentation Verification

- [ ] README updated
- [ ] CHANGELOG updated
- [ ] CREDENTIALS.md complete
- [ ] QUICKSTART_CREDENTIALS.md clear
- [ ] DEV_RELEASE_STRATEGY.md accurate
- [ ] All links work
- [ ] Code examples correct
