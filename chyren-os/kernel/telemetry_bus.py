import time
import threading
import collections
import json
import logging


class TelemetryBus:
    def __init__(self, max_history=10000):
        self.subscribers = []
        self.event_log = collections.deque(maxlen=max_history)
        self.lock = threading.Lock()
        self.stats = {"total_events": 0, "events_by_type": {}}
        self.logger = logging.getLogger("Medulla.TelemetryBus")

    def broadcast(self, sender: str, event_type: str, data: dict):
        event = {
            "ts": time.time(),
            "sender": sender,
            "type": event_type,
            "data": data,
        }
        with self.lock:
            self.event_log.append(event)
            self.stats["total_events"] += 1
            self.stats["events_by_type"][event_type] = (
                self.stats["events_by_type"].get(event_type, 0) + 1
            )
            subscribers_snapshot = list(self.subscribers)

        for callback in subscribers_snapshot:
            if callback is None:
                continue
            try:
                callback(sender, event_type, data)
            except Exception as e:
                self.logger.exception(
                    "Subscriber %r raised exception on event %r: %s", callback, event_type, e
                )

    def subscribe(self, callback) -> int:
        with self.lock:
            self.subscribers.append(callback)
            token = len(self.subscribers) - 1
        return token

    def unsubscribe(self, token: int):
        with self.lock:
            if 0 <= token < len(self.subscribers):
                self.subscribers[token] = None

    def get_recent_events(self, n: int = 100, event_type: str = None) -> list:
        with self.lock:
            events = list(self.event_log)
        if event_type is not None:
            events = [e for e in events if e["type"] == event_type]
        return events[-n:]

    def get_stats(self) -> dict:
        with self.lock:
            return {
                "total_events": self.stats["total_events"],
                "events_by_type": dict(self.stats["events_by_type"]),
                "subscriber_count": sum(1 for s in self.subscribers if s is not None),
            }

    def get_event_stream(self, event_type: str, since_ts: float) -> list:
        with self.lock:
            events = list(self.event_log)
        return [e for e in events if e["type"] == event_type and e["ts"] >= since_ts]

    def to_json(self, n: int = 50) -> str:
        events = self.get_recent_events(n)
        return json.dumps(events, default=str)
