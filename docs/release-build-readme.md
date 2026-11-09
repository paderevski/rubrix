# Catie Release Install Guide (Unsigned Builds)

Catie is not code-signed yet, so Windows and macOS will show warnings. The steps below explain how to install it. Download installers from the [latest release](https://github.com/paderevski/rubrix/releases/latest); file names include the version number (shown as `<version>` below).

## Windows 10/11 (EXE or MSI)

1. Download `Catie_<version>_x64-setup.exe` or `Catie_<version>_x64_en-US.msi`.
2. If you see **Windows protected your PC**, click **More info**, then **Run anyway**.
3. Follow the installer prompts.

## macOS

1. Download `Catie_<version>_aarch64.dmg` (Apple Silicon) or the x64 `.dmg` (Intel) if provided.
2. Open the `.dmg` and drag **Catie** to **Applications**.
3. If it will not open, run:
   ```bash
   xattr -cr /Applications/Catie.app
   ```
4. Open **Catie** again. If macOS still blocks it, go to **System Settings → Privacy & Security**, click **Open Anyway** for Catie, and confirm.

## Linux

### Debian/Ubuntu (.deb)
```bash
sudo dpkg -i catie_<version>_amd64.deb
sudo apt-get -f install
```

### Fedora/RHEL (.rpm)
```bash
sudo rpm -i catie-<version>-1.x86_64.rpm
```

### AppImage
```bash
chmod +x catie_<version>_amd64.AppImage
./catie_<version>_amd64.AppImage
```

## Verifying downloads (optional)

Compare the file's SHA256 with the value you were given for that release:

```bash
shasum -a 256 <file>        # macOS / Linux
certutil -hashfile <file> SHA256   # Windows
```

## After installing

Log in with the username and password provided by your administrator. See [Getting Started](getting-started.md).
