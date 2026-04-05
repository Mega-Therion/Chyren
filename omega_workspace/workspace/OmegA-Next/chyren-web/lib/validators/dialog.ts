export function DialogLoopDetector(history: any[]): boolean {
    // Detect repetition loops in conversation history
    if (history.length < 3) return false;
    const last = history[history.length - 1];
    const prev = history[history.length - 2];
    return last.content === prev.content;
}
