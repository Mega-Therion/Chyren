import time

class TelemetryBus:
    def __init__(self):
        self.subscribers = []

    def broadcast(self, sender, event_type, data):
        """Broadcasts telemetry data to all registered components."""
        for callback in self.subscribers:
            callback(sender, event_type, data)

    def subscribe(self, callback):
        self.subscribers.append(callback)
