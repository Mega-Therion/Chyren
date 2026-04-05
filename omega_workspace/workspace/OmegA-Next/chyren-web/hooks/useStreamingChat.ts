import { useState, useCallback } from 'react';

export function useStreamingChat() {
    const [messages, setMessages] = useState<{ role: string; content: string }[]>([]);
    const [isStreaming, setIsStreaming] = useState(false);

    const sendMessage = useCallback(async (prompt: string, history: any[]) => {
        setIsStreaming(true);
        setMessages((prev) => [...prev, { role: 'user', content: prompt }]);
        
        let fullResponse = '';
        setMessages((prev) => [...prev, { role: 'assistant', content: '' }]);

        const response = await fetch('/api/chat/stream', {
            method: 'POST',
            body: JSON.stringify({ prompt, history }),
        });

        const reader = response.body?.getReader();
        const decoder = new TextDecoder();

        if (!reader) return;

        try {
            while (true) {
                const { done, value } = await reader.read();
                if (done) break;

                const chunk = decoder.decode(value, { stream: true });
                const json = JSON.parse(chunk);

                if (json.chunk) {
                    fullResponse += json.chunk;
                    setMessages((prev) => {
                        const next = [...prev];
                        next[next.length - 1].content = fullResponse;
                        return next;
                    });
                }
            }
        } catch (e) {
            console.error('Streaming error', e);
        } finally {
            setIsStreaming(false);
        }
    }, []);

    return { messages, isStreaming, sendMessage };
}
