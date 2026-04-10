pub fn run() {
    println!(
        r#"
Chrome DevTools Protocol Setup
==============================

tabs-cli communicates with Chrome via the DevTools Protocol.
Chrome must be started with a remote debugging port enabled.

macOS:
  /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222

  Or create an alias in your shell profile:
    alias chrome='/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222'

Linux:
  google-chrome --remote-debugging-port=9222
  # or
  chromium --remote-debugging-port=9222

Windows:
  "C:\Program Files\Google\Chrome\Application\chrome.exe" --remote-debugging-port=9222

Other Chromium browsers:
  Brave:  brave --remote-debugging-port=9222
  Edge:   msedge --remote-debugging-port=9222
  Arc:    (not yet supported via CDP)

To verify it's working:
  curl http://localhost:9222/json

You should see a JSON array of your open tabs.

Configuration:
  Use --port <PORT> to connect to a different port:
    tabs list --port 9223

Tip: Close all Chrome windows first, then relaunch with the flag.
     If Chrome is already running without the flag, you'll need to
     quit it completely and relaunch.
"#
    );
}
