# TODO

## Git Workflow Improvement

### Current workflow:
- Using `release` branch as a deployment trigger
- Manually syncing `release` to `main` with `git reset --hard main` + force push
- Works but unconventional

### Recommended improvement: Switch to tag-based releases

**Benefits:**
- More standard Git practice
- Tags create permanent, immutable snapshots
- Cleaner history
- No force-pushing needed

**Steps to implement:**

1. **Update GitHub Actions workflow** (`.github/workflows/release.yml` or similar):
   ```yaml
   on:
     push:
       tags:
         - 'v*'  # Triggers on tags like v0.6.1, v0.7.0, etc.
   ```

2. **Create releases with tags instead of branch:**
   ```bash
   git checkout main
   git tag v0.6.2
   git push origin v0.6.2
   ```

3. **Optional: Use GitHub CLI for full releases:**
   ```bash
   gh release create v0.6.2 --generate-notes
   ```

4. **Once working, delete release branch:**
   ```bash
   git branch -d release
   git push origin --delete release
   ```

### Alternative: Keep current workflow
If you prefer to keep the current setup:
- It's fine for solo/small projects
- Just remember: `git reset --hard main` then `git push --force-with-lease origin release`
- Accept that it's a simple "build trigger branch" pattern
