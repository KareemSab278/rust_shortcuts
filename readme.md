# RustShortcuts

A small macOS utility written in Rust for launching applications with custom global keyboard shortcuts.

> **Disclaimer:** This currently targets macOS only. It was created as a personal productivity tool to avoid repeatedly opening Spotlight with `Command + Space` and typing the name of an application.

---

# Building & Running

## To build

```bash
# clone the repository (obviously)

clear; cargo check

# fix any errors if any (you're on your own)

# ensure it runs

clear; cargo run

# build the release binary

clear; cargo build --release

# all ok? move along.
```

The release binary will be created at:

```text
target/release/rust_shortcuts
```

The binary is a build artifact. For normal macOS usage, RustShortcuts is packaged as:

```text
RustShortcuts.app
```

---

# Create RustShortcuts.app

RustShortcuts is intended to run as a macOS application rather than as a loose binary.

Create the application directory:

```bash
mkdir -p ~/Applications/RustShortcuts.app/Contents/MacOS
```

Copy the release binary into the application:

```bash
cp target/release/rust_shortcuts \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

Make sure the executable is executable:

```bash
chmod +x \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

---

## Create the application Info.plist

Create:

```bash
micro ~/Applications/RustShortcuts.app/Contents/Info.plist
```

If you don't have `micro`, use:

```bash
nano ~/Applications/RustShortcuts.app/Contents/Info.plist
```

Add:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

<plist version="1.0">
<dict>

    <key>CFBundleExecutable</key>
    <string>rust_shortcuts</string>

    <key>CFBundleIdentifier</key>
    <string>com.kareem.rust-shortcuts</string>

    <key>CFBundleName</key>
    <string>RustShortcuts</string>

    <key>CFBundleDisplayName</key>
    <string>RustShortcuts</string>

    <key>CFBundlePackageType</key>
    <string>APPL</string>

    <key>CFBundleVersion</key>
    <string>1.0.0</string>

    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>

</dict>
</plist>
```

---

# Sign RustShortcuts.app

Because RustShortcuts listens for global keyboard events using `rdev`, macOS privacy permissions are associated with the application's code identity.

For personal use, an ad-hoc signature is sufficient.

First sign the executable:

```bash
codesign --force --sign - \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

Then sign the application bundle:

```bash
codesign --force --sign - \
    ~/Applications/RustShortcuts.app
```

Verify the application:

```bash
codesign --verify --deep --strict --verbose=2 \
    ~/Applications/RustShortcuts.app
```

Inspect the signature:

```bash
codesign -dv --verbose=4 \
    ~/Applications/RustShortcuts.app
```

You should see something similar to:

```text
Identifier=com.kareem.rust-shortcuts
Signature=adhoc
```

> **Important:** Sign the executable first, then sign the `.app` bundle.

The ad-hoc signature is suitable for running RustShortcuts on your own Mac. It is not intended for distributing the application to other Macs.

---

# Shortcuts configuration

The shortcuts are stored in:

```text
~/shortcuts.json
```

View the current configuration:

```bash
cat ~/shortcuts.json
```

Edit it with:

```bash
micro ~/shortcuts.json
```

or:

```bash
nano ~/shortcuts.json
```

Example:

```json
[
  {
    "shortcut": ["command", "option", "t"],
    "program": "Terminal.app"
  }
]
```

This means:

```text
Command + Option + T → Terminal.app
```

Multiple shortcuts can be configured:

```json
[
  {
    "shortcut": ["command", "option", "t"],
    "program": "Terminal.app"
  },
  {
    "shortcut": ["command", "option", "n"],
    "program": "Notes.app"
  }
]
```

The configuration file remains outside the application bundle so it can be edited without rebuilding the application.

> RustShortcuts reads `shortcuts.json` when it starts. After changing the configuration, restart RustShortcuts for the changes to take effect.

---

# Test RustShortcuts manually

Before installing the background service, test the exact executable that the LaunchAgent will use:

```bash
~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

If the listener starts successfully, test the configured shortcut:

```text
Command + Option + T
```

Stop the program with:

```text
Ctrl + C
```

If it works manually but not when running in the background, check the macOS permissions and LaunchAgent configuration.

---

# Run as a background process

RustShortcuts uses a macOS **LaunchAgent** so it can run automatically in the logged-in user's GUI session.

This is important because RustShortcuts needs access to global keyboard events.

Create the LaunchAgents directory:

```bash
mkdir -p ~/Library/LaunchAgents
```

Create the LaunchAgent:

```bash
micro ~/Library/LaunchAgents/com.rust-shortcuts.plist
```

If you don't have `micro`:

```bash
nano ~/Library/LaunchAgents/com.rust-shortcuts.plist
```

Add:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

<plist version="1.0">
<dict>

    <key>Label</key>
    <string>com.rust-shortcuts</string>

    <key>ProgramArguments</key>
    <array>
        <string>/Users/YOUR_USERNAME/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts</string>
    </array>

    <key>EnvironmentVariables</key>
    <dict>
        <key>HOME</key>
        <string>/Users/YOUR_USERNAME</string>

        <key>PATH</key>
        <string>/usr/bin:/bin:/usr/sbin:/sbin</string>
    </dict>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <true/>

    <key>StandardOutPath</key>
    <string>/Users/YOUR_USERNAME/Library/Logs/rust-shortcuts.log</string>

    <key>StandardErrorPath</key>
    <string>/Users/YOUR_USERNAME/Library/Logs/rust-shortcuts-error.log</string>

</dict>
</plist>
```

Replace:

```text
YOUR_USERNAME
```

with your macOS username.

Find your username with:

```bash
whoami
```

For example:

```text
kareemsab278
```

means the executable path should be:

```text
/Users/kareemsab278/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

---

# Start the background process

Load the LaunchAgent:

```bash
launchctl bootstrap gui/$(id -u) \
    ~/Library/LaunchAgents/com.rust-shortcuts.plist
```

Start the service:

```bash
launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

Check that it is running:

```bash
launchctl list | grep rust-shortcuts
```

You should see something similar to:

```text
12345    0    com.rust-shortcuts
```

Inspect the complete service:

```bash
launchctl print gui/$(id -u)/com.rust-shortcuts
```

A healthy service should contain:

```text
state = running
```

and:

```text
program = /Users/YOUR_USERNAME/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

The `program` path is important.

It should **not** point to the old development binary:

```text
/Users/YOUR_USERNAME/bin/rust_shortcuts
```

Because the LaunchAgent uses:

```xml
<key>RunAtLoad</key>
<true/>
```

RustShortcuts starts when you log into macOS.

Because it uses:

```xml
<key>KeepAlive</key>
<true/>
```

macOS will attempt to restart it if the process exits.

---

# Quick restart

After changing `shortcuts.json`, rebuilding the application, or changing Rust code, the normal restart command is:

```bash
launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

Then verify:

```bash
launchctl list | grep rust-shortcuts
```

Or:

```bash
launchctl print gui/$(id -u)/com.rust-shortcuts
```

---

# Stop the background process

To stop and unload the LaunchAgent:

```bash
launchctl bootout \
    gui/$(id -u) \
    ~/Library/LaunchAgents/com.rust-shortcuts.plist
```

Start it again with:

```bash
launchctl bootstrap gui/$(id -u) \
    ~/Library/LaunchAgents/com.rust-shortcuts.plist
```

Then:

```bash
launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

> If `bootout` reports `No such process`, the LaunchAgent was not currently loaded. You can simply run `bootstrap` again.

---

# Restart after rebuilding

After making changes to the Rust code:

```bash
cargo build --release
```

Copy the new release binary into the application:

```bash
cp target/release/rust_shortcuts \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

Make it executable:

```bash
chmod +x \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

Because the executable has changed, re-sign it:

```bash
codesign --force --sign - \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

Then re-sign the application:

```bash
codesign --force --sign - \
    ~/Applications/RustShortcuts.app
```

Verify the application:

```bash
codesign --verify --deep --strict --verbose=2 \
    ~/Applications/RustShortcuts.app
```

Restart the LaunchAgent:

```bash
launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

You do **not** need to recreate the LaunchAgent every time the Rust code changes.

---

# Logs

The LaunchAgent writes standard output to:

```text
~/Library/Logs/rust-shortcuts.log
```

Errors are written to:

```text
~/Library/Logs/rust-shortcuts-error.log
```

View the normal log:

```bash
cat ~/Library/Logs/rust-shortcuts.log
```

Follow it live:

```bash
tail -f ~/Library/Logs/rust-shortcuts.log
```

View errors:

```bash
cat ~/Library/Logs/rust-shortcuts-error.log
```

Follow errors live:

```bash
tail -f ~/Library/Logs/rust-shortcuts-error.log
```

---

# macOS permissions

RustShortcuts uses `rdev` to listen for global keyboard events.

macOS may require permissions before keyboard events can be received.

Go to:

**System Settings → Privacy & Security → Input Monitoring**

Add:

```text
RustShortcuts.app
```

from:

```text
~/Applications/RustShortcuts.app
```

and enable it.

Also check:

**System Settings → Privacy & Security → Accessibility**

Add:

```text
RustShortcuts.app
```

and enable it.

If macOS asks you to allow RustShortcuts to control or access your Mac, allow it.

### Input Monitoring is especially important

If RustShortcuts starts successfully but does not receive keyboard events, Input Monitoring is one of the first things to check.

For example, the logs may show:

```text
Starting rdev...
Starting keyboard listener...
```

while no keyboard events are received.

---

# Important: rebuilding can affect permissions

macOS associates privacy permissions with an application's code identity.

When the Rust executable inside the application bundle is replaced and re-signed, macOS may no longer consider it the same code that previously received permission.

If RustShortcuts worked previously but stops receiving keyboard events after rebuilding:

1. Open:

   **System Settings → Privacy & Security → Input Monitoring**

2. Remove the existing RustShortcuts entry.

3. Open:

   **System Settings → Privacy & Security → Accessibility**

4. Remove the existing RustShortcuts entry.

5. Re-add:

```text
~/Applications/RustShortcuts.app
```

6. Enable the permissions.

7. Restart RustShortcuts:

```bash
launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

---

# Troubleshooting the LaunchAgent

Check whether the service exists:

```bash
launchctl list | grep rust-shortcuts
```

Inspect it:

```bash
launchctl print gui/$(id -u)/com.rust-shortcuts
```

Check the process directly:

```bash
ps aux | grep '[r]ust_shortcuts'
```

The process should show the application executable:

```text
/Users/YOUR_USERNAME/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts
```

If `launchctl print` shows:

```text
state = running
```

but the shortcut doesn't work, do not assume that the keyboard listener is working.

Check the logs:

```bash
tail -f ~/Library/Logs/rust-shortcuts.log
```

and:

```bash
tail -f ~/Library/Logs/rust-shortcuts-error.log
```

---

# Troubleshooting TCC / Input Monitoring

macOS's privacy system is called **TCC**.

You can inspect TCC activity with:

```bash
sudo log stream --predicate 'subsystem == "com.apple.TCC"' --info
```

Then restart RustShortcuts:

```bash
launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

Look for messages mentioning:

```text
kTCCServiceListenEvent
```

or:

```text
Failed to match existing code requirement
```

A message such as:

```text
Failed to match existing code requirement
```

can indicate that macOS's previously stored permission no longer matches the current code identity of the RustShortcuts executable.

If this happens:

1. Remove RustShortcuts from **Input Monitoring**.
2. Remove RustShortcuts from **Accessibility**.
3. Re-sign the application.
4. Add `RustShortcuts.app` again.
5. Enable the permissions.
6. Restart the LaunchAgent.

---

# Troubleshooting a service that repeatedly restarts

`KeepAlive` means `launchd` will try to restart RustShortcuts if it exits.

Therefore, it is possible to see:

```text
state = running
```

even though the process has recently been terminated and restarted.

Inspect the service:

```bash
launchctl print gui/$(id -u)/com.rust-shortcuts
```

Look for:

```text
runs = ...
```

and:

```text
last terminating signal = ...
```

For example:

```text
last terminating signal = Terminated: 15
```

means the previous process was terminated with `SIGTERM`.

If `runs` keeps increasing unexpectedly, inspect the logs:

```bash
tail -f ~/Library/Logs/rust-shortcuts.log
```

and:

```bash
tail -f ~/Library/Logs/rust-shortcuts-error.log
```

---

# Recommended development workflow

Once everything is installed, the normal workflow is:

```bash
# edit Rust code

cargo check

# test manually if needed

cargo run

# build release

cargo build --release

# update the application

cp target/release/rust_shortcuts \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts

# ensure executable

chmod +x \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts

# sign executable

codesign --force --sign - \
    ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts

# sign app

codesign --force --sign - \
    ~/Applications/RustShortcuts.app

# verify

codesign --verify --deep --strict --verbose=2 \
    ~/Applications/RustShortcuts.app

# restart background service

launchctl kickstart -k \
    gui/$(id -u)/com.rust-shortcuts
```

Eventually, this can be automated into an `install.sh` script so rebuilding and installing RustShortcuts only requires:

```bash
./install.sh
```

The script should handle:

1. `cargo build --release`
2. Creating/updating `RustShortcuts.app`
3. Copying the binary
4. Setting executable permissions
5. Creating/updating `Info.plist`
6. Signing the executable
7. Signing the application
8. Verifying the signature
9. Installing/updating the LaunchAgent
10. Restarting the background process

---

# Final application structure

After installation, the important files should look like:

```text
~/Applications/
└── RustShortcuts.app/
    └── Contents/
        ├── Info.plist
        └── MacOS/
            └── rust_shortcuts

~/Library/LaunchAgents/
└── com.rust-shortcuts.plist

~/Library/Logs/
├── rust-shortcuts.log
└── rust-shortcuts-error.log

~/shortcuts.json
```

The intended architecture is:

```text
macOS
   │
   ├── LaunchAgent
   │       │
   │       ▼
   │   RustShortcuts.app
   │       │
   │       ▼
   │   rust_shortcuts
   │       │
   │       ▼
   │      rdev
   │       │
   │       ▼
   │  Global keyboard events
   │
   └── Command + Option + T
               │
               ▼
          Terminal.app
```

RustShortcuts is designed to run silently in the background, start automatically when the user logs into macOS, and restart automatically if the process exits.



## changing the rust code for a new build:
# 1. Build the new release
cargo build --release

# 2. Stop the currently running LaunchAgent
launchctl kickstart -k gui/$(id -u)/com.rust-shortcuts

# 3. Copy the new binary into the app
cp target/release/rust_shortcuts \
  ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts

# 4. Make sure it's executable
chmod +x \
  ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts

# 5. Re-sign the changed executable
codesign --force --sign - \
  ~/Applications/RustShortcuts.app/Contents/MacOS/rust_shortcuts

# 6. Re-sign the app bundle
codesign --force --sign - \
  ~/Applications/RustShortcuts.app

# 7. Verify the app
codesign --verify --deep --strict --verbose=2 \
  ~/Applications/RustShortcuts.app

# 8. Restart the LaunchAgent
launchctl kickstart -k gui/$(id -u)/com.rust-shortcuts

# 9. Confirm it's running
launchctl print gui/$(id -u)/com.rust-shortcuts | \
  grep -E "state|program|runs|last terminating"

# 10. Confirm the actual process
ps aux | grep '[r]ust_shortcuts'