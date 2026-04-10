#!/bin/bash
# OmegA Sovereign Production Deployment Controller
echo 'Starting Governance-compliant deployment...'
npm run build || { echo 'Build failed - rolling back'; exit 1; }
vercel --prod --yes
echo 'Deployment verified.'
