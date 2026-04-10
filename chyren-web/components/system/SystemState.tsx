import React, { useState } from 'react';

export const SystemState = () => {
    const [state] = useState({
        aeon_stage: 'Operational',
        provider_health: 'All Spokes Online',
        memory_load: '462MB',
    });

    return (
        <div className="p-4 border border-zinc-700 rounded-lg bg-zinc-900 text-zinc-300 font-mono text-xs">
            <h2 className="text-zinc-100 font-bold mb-2">Ω_t STATE VECTOR</h2>
            <div>Stage: {state.aeon_stage}</div>
            <div>Health: {state.provider_health}</div>
            <div>Memory: {state.memory_load}</div>
        </div>
    );
};
