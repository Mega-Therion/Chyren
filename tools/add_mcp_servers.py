
import subprocess
import json

servers = {
    "filesystem": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@anthropic-ai/mcp-server-filesystem", "/home/mega/Chyren"]
    },
    "github": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@anthropic-ai/mcp-server-github"],
        "env": {
            "GITHUB_PERSONAL_ACCESS_TOKEN": "MISSING"
        }
    },
    "neon-main": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@neondatabase/mcp-server-neon"],
        "env": {
            "DATABASE_URL": "MISSING"
        }
    },
    "neon-omega": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@neondatabase/mcp-server-neon"],
        "env": {
            "DATABASE_URL": "MISSING"
        }
    },
    "neon-omega2": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@neondatabase/mcp-server-neon"],
        "env": {
            "DATABASE_URL": "MISSING"
        }
    },
    "neon-catalog": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@neondatabase/mcp-server-neon"],
        "env": {
            "DATABASE_URL": "MISSING"
        }
    },
    "supabase-chyren": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@supabase/mcp-server-supabase", "--url", "https://sgvitxezqrjgjmduoool.supabase.co", "--service-role-key", "MISSING"]
    },
    "supabase-amethyst": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@supabase/mcp-server-supabase", "--url", "https://ozqsgtphovplulokejvv.supabase.co", "--service-role-key", "MISSING"]
    },
    "supabase-proj3": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@supabase/mcp-server-supabase", "--url", "https://eletftuboucrsrnapqoq.supabase.co", "--service-role-key", "MISSING"]
    },
    "cloudflare": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@cloudflare/mcp-server-cloudflare"],
        "env": {
            "CLOUDFLARE_API_TOKEN": "MISSING"
        }
    },
    "redis": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@upstash/mcp-server-redis"],
        "env": {
            "UPSTASH_REDIS_URL": "MISSING",
            "UPSTASH_REDIS_TOKEN": "MISSING"
        }
    },
    "zapier": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-zapier"],
        "env": {
            "ZAPIER_API_KEY": "MISSING"
        }
    },
    "brave-search": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-brave-search"],
        "env": {
            "BRAVE_API_KEY": "MISSING"
        }
    },
    "fetch": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-fetch"]
    },
    "memory": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-memory"]
    },
    "sequential-thinking": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-sequential-thinking"]
    },
    "puppeteer": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-puppeteer"]
    },
    "slack": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-slack"],
        "env": {
            "SLACK_BOT_TOKEN": "MISSING"
        }
    },
    "linear": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-linear"],
        "env": {
            "LINEAR_API_KEY": "MISSING"
        }
    },
    "notion": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-notion"],
        "env": {
            "NOTION_TOKEN": "MISSING"
        }
    },
    "figma": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-figma"],
        "env": {
            "FIGMA_ACCESS_TOKEN": "MISSING"
        }
    },
    "vercel": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-vercel"],
        "env": {
            "VERCEL_TOKEN": "MISSING"
        }
    },
    "stripe": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-stripe"],
        "env": {
            "STRIPE_SECRET_KEY": "MISSING"
        }
    },
    "resend": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-resend"],
        "env": {
            "RESEND_API_KEY": "MISSING"
        }
    },
    "twilio": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-twilio"],
        "env": {
            "TWILIO_ACCOUNT_SID": "MISSING",
            "TWILIO_AUTH_TOKEN": "MISSING"
        }
    },
    "firebase": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-firebase"],
        "env": {
            "FIREBASE_PROJECT_ID": "MISSING",
            "FIREBASE_SERVICE_ACCOUNT_JSON": "MISSING"
        }
    },
    "sentry": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-sentry"],
        "env": {
            "SENTRY_AUTH_TOKEN": "MISSING"
        }
    },
    "datadog": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-datadog"],
        "env": {
            "DATADOG_API_KEY": "MISSING",
            "DATADOG_APP_KEY": "MISSING"
        }
    },
    "openai": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-openai"],
        "env": {
            "OPENAI_API_KEY": "MISSING"
        }
    },
    "huggingface": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-huggingface"],
        "env": {
            "HUGGING_FACE_HUB_TOKEN": "MISSING"
        }
    },
    "docker": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-docker"]
    },
    "kubernetes": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-kubernetes"]
    },
    "terraform": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-terraform"]
    },
    "aws": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-aws"],
        "env": {
            "AWS_ACCESS_KEY_ID": "MISSING",
            "AWS_SECRET_ACCESS_KEY": "MISSING"
        }
    },
    "gcp": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-gcp"],
        "env": {
            "GCP_PROJECT_ID": "MISSING"
        }
    },
    "airtable": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-airtable"],
        "env": {
            "AIRTABLE_API_KEY": "MISSING"
        }
    },
    "context7": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@upstash/context7-mcp"],
        "env": {
            "CONTEXT7_API_KEY": "MISSING"
        }
    },
    "exa": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-exa"],
        "env": {
            "EXA_API_KEY": "MISSING"
        }
    },
    "perplexity": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "mcp-server-perplexity"],
        "env": {
            "PERPLEXITY_API_KEY": "MISSING"
        }
    }
}

for name, config in servers.items():
    json_str = json.dumps(config)
    cmd = ["claude", "mcp", "add-json", name, json_str]
    print(f"Adding {name}...")
    subprocess.run(cmd)
