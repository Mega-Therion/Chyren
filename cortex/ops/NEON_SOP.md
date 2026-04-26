# Standard Operating Procedure: Neon Database Project Overflow (SOP-001)

## Policy
When any Chyren agent detects a **Database Quota Overflow** (e.g., "exceeded the data transfer quota" or "storage limit reached"), the agent MUST NOT wait for operator intervention to continue technical hardening. Instead, the agent is authorized and directed to scale the infrastructure horizontally by creating a new Neon project and linking it to the Chyren Database Pool.

## Procedure

### 1. Detection
Identify the error signature:
`psycopg2.OperationalError: connection to server ... failed: ERROR: Your project has exceeded the data transfer quota. Upgrade your plan to increase limits.`

### 2. Creation of Overflow Project
If `neonctl` is not available, provide the user with the exact CLI command or create a script to use the Neon API to spin up a new project.
- **Project Name Convention:** `chyren-overflow-[UUID]`
- **Region:** `us-east-1` (match original)
- **Database Name:** `neondb`

### 3. Registration in the Pool
Chyren maintains a pool of active database projects to circumvent hobby-tier limits.
- Add the new connection string to `/home/mega/Chyren/hub/ops/db_pool.json`.
- Each agent must rotate through the pool or use the latest 'current' project for new writes while maintaining read-access to legacy shards.

### 4. Schema Initialization
Every new project in the pool MUST be initialized with the core Chyren schema:
- `family_profiles`
- `chyren_memory_entries`
- `chyren_governance_logs`

### 5. Environment Update
Update the local `.env.local` to point to the latest pool member if it is considered the new 'Primary' for the current session.

## Governance
- **Authority:** Approved by RY as a standard architectural reflex.
- **Audit:** Every project creation event MUST be logged in the `chyren_memory_entries` of the *new* project with the source `infra_scalability_reflex`.

---
*Created by: Antigravity-RY*
*Status: ACTIVE*
