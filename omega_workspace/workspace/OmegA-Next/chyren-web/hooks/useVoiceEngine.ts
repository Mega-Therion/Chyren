import { useState, useCallback } from 'react';

export function useVoiceEngine() {
    const [isListening, setIsListening] = useState(false);
    const [transcript, setTranscript] = useState('');

    const startListening = useCallback(() => {
        setIsListening(true);
        console.log("Voice Engine: Microphone active.");
    }, []);

    const stopListening = useCallback(() => {
        setIsListening(false);
        console.log("Voice Engine: Microphone inactive.");
    }, []);

    return { isListening, transcript, startListening, stopListening };
}
