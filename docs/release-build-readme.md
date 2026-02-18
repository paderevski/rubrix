# Catie Release Install Guide (Unsigned Builds)

Catie is not code-signed yet, so Windows and macOS will show warnings. The steps
below explain how to safely install it.

## Windows 10/11 (EXE or MSI)

1. Download `Catie_0.9.0_x64-setup.exe` or `Catie_0.9.0_x64_en-US.msi`.
2. If you see **Windows protected your PC**:
   - Click **More info**.
   - Click **Run anyway**.
3. Follow the installer prompts.
4. If Windows warns during install, confirm you want to proceed.

Optional: verify the file integrity with the SHA256 listed below before running.

## macOS (Apple Silicon)

1. Download `Catie_0.9.0_aarch64.dmg`.
2. Open the .dmg and drag **Catie** to **Applications**.
3. If it will not open, run this in Terminal:
   ```bash
   xattr -cr /Applications/Catie.app
   ```
4. Then open **Catie** again. If macOS still blocks it:
   - Go to **System Settings -> Privacy & Security**.
   - Under **Security**, click **Open Anyway** for Catie.
   - Confirm **Open**.

Optional: verify the file integrity with the SHA256 listed below before opening.

## Linux (Deb / RPM / AppImage)

Pick the format that matches your distro.

### Debian/Ubuntu (.deb)
```bash
sudo dpkg -i catie_0.9.0_amd64.deb
sudo apt-get -f install
```

### Fedora/RHEL (.rpm)
```bash
sudo rpm -i catie-0.9.0-1.x86_64.rpm
```

### AppImage (most distros)
```bash
chmod +x catie_0.9.0_amd64.AppImage
./catie_0.9.0_amd64.AppImage
```

Optional: verify the file integrity with the SHA256 listed below before installing.

## Checksums (SHA256)

- `Catie_0.9.0_x64-setup.exe`
  `d62d5cf130f58345bdbb82b34537bf325e9a1924290707dacd0a9a074fba492f`
- `Catie_0.9.0_x64_en-US.msi`
  `3fcdfd08427330202f95cbb3ef5402dff7c51783d7c79f6010b0e2cfda0219b9`
- `Catie_0.9.0_aarch64.dmg`
  `4d81401dc9f54d045944f23a146f2d090977a333aa7a4f4dbbe18e1fc349a607`
- `catie_0.9.0_amd64.deb`
  `ec8c747109f2e143200f3204b6162f54a6ceabc2b5a0c9bf989888f0a71980c8`
- `catie-0.9.0-1.x86_64.rpm`
  `69fa8fe62ee7d5db42b53213e2c2a7abd36e7aeae438efe084700bf2ff8a29ba`
- `catie_0.9.0_amd64.AppImage`
  `43e7f57f1f707068ea0f61710a9d1952742c367eba15280d898636020f435b5d`
