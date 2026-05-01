import hashlib
import json
import time
import hmac
import secrets
import logging


class SwarmAttestation:
    def __init__(self, yettragrammaton_seed: str, quorum_threshold: float = 0.67):
        self.seed = yettragrammaton_seed
        self.quorum_threshold = quorum_threshold
        self.peers = {}
        self.message_log = []
        self.byzantine_suspects = set()
        self.logger = logging.getLogger("Hub.SwarmAttestation")

    def _compute_hmac(self, canonical_data: str) -> str:
        return hmac.new(
            self.seed.encode(), canonical_data.encode(), hashlib.sha256
        ).hexdigest()

    def sign_message(self, message: dict) -> dict:
        canonical = json.dumps(message, sort_keys=True)
        sig = self._compute_hmac(canonical)
        return {
            "data": message,
            "signature": sig,
            "ts": time.time(),
            "signer": "self",
        }

    def verify_message(self, signed_message: dict) -> bool:
        data = signed_message.get("data")
        provided_sig = signed_message.get("signature", "")
        canonical = json.dumps(data, sort_keys=True)
        expected = self._compute_hmac(canonical)
        try:
            valid = hmac.compare_digest(expected, provided_sig)
        except TypeError:
            valid = False
        if not valid:
            self.logger.warning(
                "Signature verification failed for message from signer=%r",
                signed_message.get("signer"),
            )
        return valid

    def register_peer(self, peer_id: str, peer_seed_hash: str):
        self.peers[peer_id] = {"peer_seed_hash": peer_seed_hash, "registered_at": time.time()}
        self.logger.info("Registered peer %r", peer_id)

    def submit_peer_vote(self, peer_id: str, message_hash: str, signed_vote: dict) -> bool:
        if not self.verify_message(signed_vote):
            self.byzantine_suspects.add(peer_id)
            self.logger.warning("Peer %r submitted invalid vote — flagged as Byzantine suspect", peer_id)
            return False
        self.message_log.append({
            "peer_id": peer_id,
            "message_hash": message_hash,
            "vote": signed_vote,
            "recorded_at": time.time(),
        })
        return True

    def check_quorum(self, message_hash: str) -> tuple:
        valid_votes = [
            entry for entry in self.message_log
            if entry["message_hash"] == message_hash
            and entry["peer_id"] not in self.byzantine_suspects
        ]
        total_eligible = max(len(self.peers) - len(self.byzantine_suspects), 1)
        agreement_ratio = len(valid_votes) / total_eligible
        quorum_reached = agreement_ratio >= self.quorum_threshold
        return (quorum_reached, agreement_ratio)

    def get_trust_summary(self) -> dict:
        return {
            "registered_peers": list(self.peers.keys()),
            "byzantine_suspects": list(self.byzantine_suspects),
            "messages_verified": len(self.message_log),
            "quorum_threshold": self.quorum_threshold,
        }
