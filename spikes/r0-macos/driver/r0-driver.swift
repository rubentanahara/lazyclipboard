import AppKit
import Carbon
import Foundation

let sentinel = "R0-SENTINEL-7f3a"
let initialLine = "AAABBB"
let caretOffset = 3
let expectedAfterPaste = "AAA" + sentinel + "BBB"
let expectedPanelKeys = ["Character", "Character", "ArrowDown", "ArrowUp", "Escape"]

enum KeyCode {
    static let a: CGKeyCode = 0
    static let b: CGKeyCode = 11
    static let c: CGKeyCode = 8
    static let v: CGKeyCode = 9
    static let returnKey: CGKeyCode = 36
    static let escape: CGKeyCode = 53
    static let shift: CGKeyCode = 56
    static let left: CGKeyCode = 123
    static let right: CGKeyCode = 124
    static let down: CGKeyCode = 125
    static let up: CGKeyCode = 126
}

let eventSource = CGEventSource(stateID: .hidSystemState)
var allowedFrontmostPids: Set<pid_t> = []

var runningSpike: Spike?

func fail(_ message: String) -> Never {
    FileHandle.standardError.write(Data("abort: \(message)\n".utf8))
    runningSpike?.stop()
    exit(2)
}

func frontmostPid() -> pid_t? {
    NSWorkspace.shared.frontmostApplication?.processIdentifier
}

func requireAllowedFrontmost() {
    guard let pid = frontmostPid(), allowedFrontmostPids.contains(pid) else {
        fail("frontmost app \(frontmostPid().map(String.init) ?? "none") is not an allowed target, refusing to post keys")
    }
}

func postKey(_ code: CGKeyCode, down: Bool, flags: CGEventFlags = []) {
    requireAllowedFrontmost()
    let event = CGEvent(keyboardEventSource: eventSource, virtualKey: code, keyDown: down)!
    event.flags = flags
    event.post(tap: .cghidEventTap)
}

func tapKey(_ code: CGKeyCode, flags: CGEventFlags = [], pause: UInt32 = 12_000) {
    postKey(code, down: true, flags: flags)
    usleep(pause)
    postKey(code, down: false, flags: flags)
    usleep(pause)
}

func pressShortcut() {
    tapKey(KeyCode.v, flags: [.maskCommand, .maskAlternate], pause: 40_000)
}

@discardableResult
func shell(_ executable: String, _ arguments: [String]) -> String {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: executable)
    process.arguments = arguments
    let output = Pipe()
    process.standardOutput = output
    process.standardError = Pipe()
    try! process.run()
    let data = output.fileHandleForReading.readDataToEndOfFile()
    process.waitUntilExit()
    return String(decoding: data, as: UTF8.self).trimmingCharacters(in: .whitespacesAndNewlines)
}

func appleScript(_ source: String) -> String {
    shell("/usr/bin/osascript", ["-e", source])
}

func waitUntil(timeout: TimeInterval, _ condition: () -> Bool) -> Bool {
    let deadline = Date().addingTimeInterval(timeout)
    while Date() < deadline {
        if condition() { return true }
        usleep(5_000)
    }
    return condition()
}

func currentLayoutName() -> String {
    let source = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
    let id = TISGetInputSourceProperty(source, kTISPropertyInputSourceID)
    return Unmanaged<CFString>.fromOpaque(id!).takeUnretainedValue() as String
}

struct LoggedEvent {
    let name: String
    let fields: [String: Any]

    var millis: Double { fields["ms"] as? Double ?? -1 }
    var frontmostPid: pid_t? { (fields["frontmost_pid"] as? Int).map { pid_t($0) } }
}

let spikeBundleIdentifier = "io.github.rubentanahara.lazyclipboard.r0macos"

final class Spike {
    let process = Process()
    let logURL: URL
    private let launchedAsBundle: Bool

    init(executable: String, logURL: URL) {
        self.logURL = logURL
        launchedAsBundle = executable.hasSuffix(".app")
        FileManager.default.createFile(atPath: logURL.path, contents: nil)
        if launchedAsBundle {
            process.executableURL = URL(fileURLWithPath: "/usr/bin/open")
            process.arguments = ["-n", "--stdout", logURL.path, executable]
        } else {
            process.executableURL = URL(fileURLWithPath: executable)
            process.standardOutput = try! FileHandle(forWritingTo: logURL)
        }
        try! process.run()
    }

    var pid: pid_t {
        launchedAsBundle
            ? NSRunningApplication.runningApplications(withBundleIdentifier: spikeBundleIdentifier).first?.processIdentifier ?? 0
            : process.processIdentifier
    }

    func events() -> [LoggedEvent] {
        let text = (try? String(contentsOf: logURL, encoding: .utf8)) ?? ""
        return text.split(separator: "\n").compactMap { line in
            guard let object = try? JSONSerialization.jsonObject(with: Data(line.utf8)) as? [String: Any],
                  let name = object["event"] as? String else { return nil }
            return LoggedEvent(name: name, fields: object["fields"] as? [String: Any] ?? [:])
        }
    }

    func wait(for name: String, after count: Int, timeout: TimeInterval = 3) -> LoggedEvent? {
        var found: LoggedEvent?
        _ = waitUntil(timeout: timeout) {
            found = events().dropFirst(count).first { $0.name == name }
            return found != nil
        }
        return found
    }

    func stop() {
        if launchedAsBundle {
            NSRunningApplication.runningApplications(withBundleIdentifier: spikeBundleIdentifier).forEach { $0.terminate() }
        } else {
            process.terminate()
        }
    }
}

typealias Flavours = [[String: Data]]

func pasteboardFlavours() -> Flavours {
    (NSPasteboard.general.pasteboardItems ?? []).map { item in
        Dictionary(uniqueKeysWithValues: item.types.compactMap { type in
            item.data(forType: type).map { (type.rawValue, $0) }
        })
    }
}

func setPriorText(_ text: String) {
    let pasteboard = NSPasteboard.general
    pasteboard.clearContents()
    let item = NSPasteboardItem()
    item.setString(text, forType: .string)
    item.setString("<b>\(text)</b>", forType: .html)
    pasteboard.writeObjects([item])
}

func setPriorImage() {
    let image = NSImage(size: NSSize(width: 24, height: 24), flipped: false) { rect in
        NSColor.systemRed.setFill()
        rect.fill()
        return true
    }
    NSPasteboard.general.clearContents()
    NSPasteboard.general.writeObjects([image])
}

protocol Target {
    var name: String { get }
    var pid: pid_t { get }
    func prepare()
    func resetLine()
    func content() -> String
    func cleanup()
}

func applicationPid(bundleIdentifier: String) -> pid_t {
    _ = waitUntil(timeout: 10) {
        NSRunningApplication.runningApplications(withBundleIdentifier: bundleIdentifier).first != nil
    }
    return NSRunningApplication.runningApplications(withBundleIdentifier: bundleIdentifier).first!.processIdentifier
}

func activate(_ pid: pid_t) {
    if let bundleIdentifier = NSRunningApplication(processIdentifier: pid)?.bundleIdentifier {
        appleScript("tell application id \"\(bundleIdentifier)\" to activate")
    }
    if !waitUntil(timeout: 3, { frontmostPid() == pid }) { fail("could not bring pid \(pid) to the front") }
}

final class TextEditTarget: Target {
    let name = "TextEdit"
    var pid: pid_t = 0
    private var wasRunning = false

    func prepare() {
        wasRunning = !NSRunningApplication.runningApplications(withBundleIdentifier: "com.apple.TextEdit").isEmpty
        appleScript("tell application \"TextEdit\" to activate")
        pid = applicationPid(bundleIdentifier: "com.apple.TextEdit")
        appleScript("tell application \"TextEdit\" to if (count of documents) = 0 then make new document")
        activate(pid)
        allowedFrontmostPids = [pid]
    }

    func resetLine() {
        appleScript("tell application \"TextEdit\" to set text of document 1 to \"\(initialLine)\"")
        activate(pid)
        tapKey(KeyCode.up, flags: .maskCommand)
        for _ in 0..<caretOffset { tapKey(KeyCode.right) }
    }

    func content() -> String {
        appleScript("tell application \"TextEdit\" to get text of document 1")
    }

    func cleanup() {
        appleScript("tell application \"TextEdit\" to close document 1 saving no")
        if !wasRunning { appleScript("tell application \"TextEdit\" to quit") }
    }
}

final class TerminalTarget: Target {
    let name = "Terminal"
    var pid: pid_t = 0
    private var windowId = ""
    private let plainPrompt = "$"

    func prepare() {
        appleScript("tell application \"Terminal\" to activate")
        pid = applicationPid(bundleIdentifier: "com.apple.Terminal")
        windowId = appleScript("tell application \"Terminal\"\ndo script \"exec env PS1='\(plainPrompt) ' bash --noprofile --norc\"\nreturn id of front window\nend tell")
        activate(pid)
        allowedFrontmostPids = [pid]
        if !waitUntil(timeout: 10, { lastNonEmptyLine() == plainPrompt }) { fail("Terminal never showed the plain bash prompt") }
    }

    func resetLine() {
        activate(pid)
        tapKey(KeyCode.c, flags: .maskControl)
        if !waitUntil(timeout: 3, { lastNonEmptyLine() == plainPrompt }) { fail("Terminal prompt did not clear after Ctrl-C") }
        for _ in 0..<caretOffset { tapKey(KeyCode.a, flags: .maskShift) }
        for _ in 0..<(initialLine.count - caretOffset) { tapKey(KeyCode.b, flags: .maskShift) }
        for _ in 0..<(initialLine.count - caretOffset) { tapKey(KeyCode.left) }
        if !waitUntil(timeout: 3, { content() == initialLine }) { fail("Terminal did not echo the reset line, last line \(lastNonEmptyLine().debugDescription)") }
    }

    func content() -> String {
        let line = lastNonEmptyLine()
        return line.hasPrefix(plainPrompt + " ") ? String(line.dropFirst(plainPrompt.count + 1)) : line
    }

    private func lastNonEmptyLine() -> String {
        let screen = appleScript("tell application \"Terminal\" to get contents of selected tab of window id \(windowId)")
        let line = screen.split(separator: "\n").map(String.init).last { !$0.trimmingCharacters(in: .whitespaces).isEmpty } ?? ""
        return line.trimmingCharacters(in: .whitespaces)
    }

    func cleanup() {
        appleScript("tell application \"Terminal\" to close window id \(windowId)")
    }
}

final class SafariTarget: Target {
    let name = "Safari"
    var pid: pid_t = 0
    private let port = 18_977
    private let server = Process()
    private var windowId = ""

    func prepare() {
        server.executableURL = URL(fileURLWithPath: "/usr/bin/python3")
        server.arguments = [URL(fileURLWithPath: #filePath).deletingLastPathComponent().appendingPathComponent("textarea_server.py").path, String(port)]
        try! server.run()
        _ = waitUntil(timeout: 5) { !state().isEmpty }
        windowId = appleScript("tell application \"Safari\"\nactivate\nmake new document with properties {URL:\"http://127.0.0.1:\(port)/\"}\nreturn id of front window\nend tell")
        pid = applicationPid(bundleIdentifier: "com.apple.Safari")
        activate(pid)
        allowedFrontmostPids = [pid]
        if !waitUntil(timeout: 5, { state()["focused"] as? Bool == true }) { fail("Safari textarea never reported focus") }
    }

    func resetLine() {
        activate(pid)
        shell("/usr/bin/curl", ["-s", "-X", "POST", "http://127.0.0.1:\(port)/reset"])
        let ready = waitUntil(timeout: 3) {
            let current = state()
            return current["value"] as? String == initialLine && current["caret"] as? Int == caretOffset && current["focused"] as? Bool == true
        }
        if !ready { fail("Safari textarea did not reset and keep focus") }
    }

    func content() -> String {
        usleep(120_000)
        return state()["value"] as? String ?? ""
    }

    func cleanup() {
        appleScript("tell application \"Safari\" to close (first window whose id is \(windowId))")
        server.terminate()
    }

    private func state() -> [String: Any] {
        let body = shell("/usr/bin/curl", ["-s", "--max-time", "1", "http://127.0.0.1:\(port)/state"])
        return (try? JSONSerialization.jsonObject(with: Data(body.utf8)) as? [String: Any]) ?? [:]
    }
}

func makeTarget(_ name: String) -> Target {
    switch name {
    case "TextEdit": return TextEditTarget()
    case "Terminal": return TerminalTarget()
    case "Safari": return SafariTarget()
    default: fail("unknown target \(name)")
    }
}

let discardLimit = 8
var tallies: [String: (passed: Int, runs: Int)] = [:]
var failureDetails: [String] = []
var pendingRecords: [(key: String, passed: Bool, detail: String)] = []
var discardedIterations = 0

func record(_ criterion: String, _ target: String, _ passed: Bool, _ detail: @autoclosure () -> String) {
    pendingRecords.append(("\(criterion) \(target)", passed, passed ? "" : detail()))
}

func commitIteration() {
    for result in pendingRecords {
        let current = tallies[result.key] ?? (0, 0)
        tallies[result.key] = (current.passed + (result.passed ? 1 : 0), current.runs + 1)
        if !result.passed { failureDetails.append("\(result.key): \(result.detail)") }
    }
    pendingRecords = []
}

func discardIteration() {
    pendingRecords = []
    discardedIterations += 1
    if discardedIterations > discardLimit { fail("too much foreign keyboard input, keep hands off the Mac during the run") }
}

func foreignInputSeen(spike: Spike, since mark: Int, expectedKeys: [String]) -> Bool {
    let keys = spike.events().dropFirst(mark).filter { $0.name == "panel_key" }.compactMap { $0.fields["key"] as? String }
    return keys.count > expectedKeys.count || !Set(keys).isSubset(of: Set(expectedKeys))
}

func settle(spike: Spike) {
    usleep(500_000)
    tapKey(KeyCode.escape)
    usleep(500_000)
}

func printSummary() {
    commitIteration()
    print("discarded iterations (foreign keyboard input): \(discardedIterations)")
    print("| Check | Passed |\n| --- | --- |")
    for key in tallies.keys.sorted() {
        let tally = tallies[key]!
        print("| \(key) | \(tally.passed)/\(tally.runs) |")
    }
    for detail in failureDetails { print("FAIL \(detail)") }
}

func openPanel(spike: Spike, target: Target) -> LoggedEvent? {
    activate(target.pid)
    let mark = spike.events().count
    pressShortcut()
    return spike.wait(for: "panel_ready", after: mark)
}

func checkOpenedWithoutLeak(spike: Spike, target: Target, iteration: Int) {
    guard let ready = openPanel(spike: spike, target: target) else {
        record("R0-1 opens", target.name, false, "no panel_ready on iteration \(iteration)")
        return
    }
    record("R0-1 opens", target.name, true, "")
    usleep(200_000)
    let strict = ready.frontmostPid == target.pid && frontmostPid() == target.pid
    record("R0-3 strict focus while open", target.name, strict, "frontmost \(String(describing: ready.frontmostPid)) then \(String(describing: frontmostPid())) expected \(target.pid)")
    let leaked = target.content()
    record("R0-1 no chord character leaks", target.name, leaked == initialLine, "content \(leaked.debugDescription)")
}

func checkPanelKeys(spike: Spike, target: Target) {
    let mark = spike.events().count
    tapKey(KeyCode.a)
    tapKey(KeyCode.b)
    tapKey(KeyCode.down)
    tapKey(KeyCode.up)
    tapKey(KeyCode.escape)
    let hidden = spike.wait(for: "panel_hidden", after: mark)
    let received = spike.events().dropFirst(mark).filter { $0.name == "panel_key" }.compactMap { $0.fields["key"] as? String }
    record("R0-2 panel receives keys", target.name, received == expectedPanelKeys && hidden != nil, "received \(received)")
    record("R0-3 strict focus after hide", target.name, frontmostPid() == target.pid, "frontmost \(String(describing: frontmostPid()))")
}

func checkPaste(spike: Spike, target: Target, iteration: Int, shiftHeld: Bool, imagePrior: Bool) {
    imagePrior ? setPriorImage() : setPriorText("prior-\(iteration)")
    let prior = pasteboardFlavours()
    guard openPanel(spike: spike, target: target) != nil else {
        record("R0-4 paste", target.name, false, "panel did not open on iteration \(iteration)")
        return
    }
    let mark = spike.events().count
    if shiftHeld {
        postKey(KeyCode.shift, down: true, flags: .maskShift)
        usleep(20_000)
        postKey(KeyCode.returnKey, down: true, flags: .maskShift)
        postKey(KeyCode.returnKey, down: false, flags: .maskShift)
        usleep(300_000)
        postKey(KeyCode.shift, down: false)
    } else {
        tapKey(KeyCode.returnKey)
    }
    let done = spike.wait(for: "paste_done", after: mark, timeout: 5)
    usleep(150_000)
    let label = shiftHeld ? "R0-8 paste with shift held" : "R0-4 paste at original caret"
    let pasted = target.content()
    record(label, target.name, done != nil && pasted == expectedAfterPaste, "content \(pasted.debugDescription)")
    let restored = pasteboardFlavours() == prior
    record(imagePrior ? "R0-5 image restored" : "R0-5 text restored", target.name, restored, "clipboard differs from prior")
}

func runChecks(spike: Spike, target: Target, iterations: Int, shiftHeld: Bool, imagePrior: Bool, keyChecks: Bool) {
    let expectedKeys = (keyChecks ? expectedPanelKeys : []) + (shiftHeld ? ["Shift", "Enter"] : ["Enter"])
    for iteration in 1...iterations {
        var disturbed = false
        repeat {
            let mark = spike.events().count
            target.resetLine()
            if keyChecks {
                checkOpenedWithoutLeak(spike: spike, target: target, iteration: iteration)
                checkPanelKeys(spike: spike, target: target)
            }
            checkPaste(spike: spike, target: target, iteration: iteration, shiftHeld: shiftHeld, imagePrior: imagePrior)
            disturbed = foreignInputSeen(spike: spike, since: mark, expectedKeys: expectedKeys)
            if disturbed {
                discardIteration()
                settle(spike: spike)
            } else {
                commitIteration()
            }
        } while disturbed
    }
}

func fullscreenChecks(spike: Spike, iterations: Int) {
    let target = TextEditTarget()
    target.prepare()
    setTextEditFullScreen(true)
    sleep(2)
    let isFullscreen = appleScript("tell application \"System Events\" to tell process \"TextEdit\" to get value of attribute \"AXFullScreen\" of \(textEditDocumentWindow)")
    if isFullscreen != "true" { fail("TextEdit did not enter a fullscreen Space, AXFullScreen is \(isFullscreen)") }
    for iteration in 1...iterations {
        guard let ready = openPanel(spike: spike, target: target) else {
            record("R0-7 panel over fullscreen", target.name, false, "no panel_ready on iteration \(iteration)")
            continue
        }
        usleep(300_000)
        let spikePid = spike.pid
        shell("/usr/sbin/screencapture", ["-x", spike.logURL.deletingLastPathComponent().appendingPathComponent("fullscreen-\(iteration).png").path])
        let onScreen = panelIsOnScreen(ownerPid: spikePid)
        record("R0-7 panel over fullscreen", target.name, onScreen && ready.frontmostPid == target.pid, "onscreen \(onScreen) frontmost \(String(describing: ready.frontmostPid))")
        tapKey(KeyCode.escape)
        usleep(300_000)
    }
    setTextEditFullScreen(false)
    sleep(2)
    target.cleanup()
}

let textEditDocumentWindow = "(first window whose subrole is \"AXStandardWindow\")"

func setTextEditFullScreen(_ enabled: Bool) {
    appleScript("tell application \"System Events\" to tell process \"TextEdit\" to set value of attribute \"AXFullScreen\" of \(textEditDocumentWindow) to \(enabled)")
}

func panelIsOnScreen(ownerPid: pid_t) -> Bool {
    let windows = CGWindowListCopyWindowInfo([.optionOnScreenOnly], kCGNullWindowID) as? [[String: Any]] ?? []
    return windows.contains { ($0[kCGWindowOwnerPID as String] as? Int) == Int(ownerPid) && ($0[kCGWindowAlpha as String] as? Double ?? 0) > 0 }
}

func permissionChecks(spike: Spike, iterations: Int) {
    let target = TextEditTarget()
    target.prepare()
    target.resetLine()
    let startedUntrusted = spike.events().first { $0.name == "started" }?.fields["accessibility_trusted"] as? Bool == false
    record("R0-9 app starts without Accessibility", target.name, startedUntrusted, "app reports trusted")
    for _ in 1...iterations {
        guard let ready = openPanel(spike: spike, target: target) else {
            record("R0-9 permission state shown", target.name, false, "no panel_ready")
            continue
        }
        record("R0-9 permission state shown", target.name, ready.fields["permission_shown"] as? Bool == true, "permission section hidden")
        let mark = spike.events().count
        tapKey(KeyCode.returnKey)
        let blocked = spike.wait(for: "paste_blocked", after: mark)
        record("R0-9 paste refused with a reason", target.name, blocked != nil, "no paste_blocked event")
        tapKey(KeyCode.escape)
        usleep(300_000)
    }
    target.cleanup()
}

func p95Summary(spike: Spike) {
    let samples = spike.events().filter { $0.name == "panel_ready" }.map(\.millis).sorted()
    guard !samples.isEmpty else { return }
    let nearestRank = Int((Double(samples.count) * 0.95).rounded(.up)) - 1
    print("panel_ready samples \(samples.count) p95 \(String(format: "%.1f", samples[nearestRank])) ms max \(String(format: "%.1f", samples.last!)) ms median \(String(format: "%.1f", samples[samples.count / 2])) ms")
}

let arguments = CommandLine.arguments
guard arguments.count >= 5 else {
    fail("usage: r0-driver <spike-binary> <log-file> <scenario> <target> [iterations]")
}
let spike = Spike(executable: arguments[1], logURL: URL(fileURLWithPath: arguments[2]))
runningSpike = spike
defer { spike.stop() }
_ = waitUntil(timeout: 10) { spike.events().contains { $0.name == "started" } }
let scenario = arguments[3]
let targetName = arguments[4]
let iterations = arguments.count > 5 ? Int(arguments[5])! : 20
print("layout \(currentLayoutName()) scenario \(scenario) target \(targetName) iterations \(iterations)")
appleScript("display notification \"R0 smoke run in progress, keep hands off the keyboard and mouse\" with title \"lazyclipboard R0\"")

switch scenario {
case "full", "shift", "image", "layout":
    let target = makeTarget(targetName)
    target.prepare()
    runChecks(spike: spike, target: target, iterations: iterations, shiftHeld: scenario == "shift", imagePrior: scenario == "image", keyChecks: scenario == "full")
    target.cleanup()
case "fullscreen":
    fullscreenChecks(spike: spike, iterations: iterations)
case "permission":
    permissionChecks(spike: spike, iterations: iterations)
default:
    fail("unknown scenario \(scenario)")
}
printSummary()
p95Summary(spike: spike)
