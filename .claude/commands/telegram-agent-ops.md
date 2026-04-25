---
name: telegram-agent-ops
description: Provision, validate, and maintain the Telegram-facing portions of the agent mesh. Use this skill when asked to "deploy bot to Telegram", "check bot status", "update bot token", or "manage Telegram webhooks".
---

# Telegram Agent Ops

## Purpose
This skill manages the gateway between OmegA and Telegram. It ensures that the agent is accessible, responsive, and secure within the Telegram ecosystem.

## Core Capabilities
1.  **Deployment**: Provision new bots via BotFather API/CLI.
2.  **Configuration**: Manage webhooks, polling, and command lists.
3.  **Monitoring**: Health checks for the bridge service (e.g., `omega-telegram-bridge`).
4.  **Security**: Rotate tokens and manage access lists for authorized users.

## Workflows

### 1. Bot Deployment/Update
**Trigger**: "Deploy new Telegram bot", "Update bot commands"

1.  **Token Validation**: Verify current token.
2.  **Update Commands**: Send command list to BotFather.
3.  **Webhook Setup**: Set webhook endpoint to the gateway URL.
4.  **Verification**: Ping the bot to verify response.

### 2. Status Check
**Trigger**: "Is the Telegram bot online?", "Check bot health"

1.  **Get Me**: Call `getMe` API.
2.  **Log Check**: Verify recent interactions in `logs/telegram.log`.
3.  **Status**: Report availability and last interaction time.

---
**Resources**
-   `references/bot_commands.md`: Canonical list of bot commands (/start, /debug, /audit, etc.).
