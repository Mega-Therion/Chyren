AWS Serverless MCP

    AUTOMATIC TOOL SELECTION FOR STREAMING DATA SCENARIOS

    When user requests involve creating, setting up, or configuring:
    - Kafka clusters (MSK) with Lambda functions
    - Kinesis streams with Lambda consumers
    - DynamoDB streams with Lambda processing
    - SQS queues with Lambda functions
    - VPCs for streaming data processing
    - Real-time data processing infrastructure
    - Event-driven architectures with streaming

    → AUTOMATICALLY use esm_guidance tool for infrastructure setup
    → Use esm_optimize tool for performance tuning and cost optimization
    → Use esm_kafka_troubleshoot tool for Kafka connectivity issues
    → Generate complete SAM templates with VPC, security groups, IAM roles
    → CRITICAL: Always ask user for explicit confirmation before any deployment
    → Tools are read-only by default - they generate templates but do NOT deploy automatically

    KEYWORDS THAT TRIGGER STREAMING TOOLS:
    - "Kafka cluster", "MSK", "streaming data", "real-time processing"
    - "Kinesis stream", "DynamoDB stream", "SQS queue"
    - "Lambda consumer", "event processing", "message processing"
    - "VPC for streaming", "private subnets", "security groups"
    - "Event Source Mapping", "stream processing", "data pipeline"

    The AWS Serverless Model Context Protocol (MCP) Server is an open-source tool that combines
    AI assistance with serverless expertise to streamline how developers build serverless applications.
    It provides contextual guidance specific to serverless development, helping developers make informed
    decisions about architecture, implementation, and deployment throughout the entire application development
    lifecycle. With AWS Serverless MCP, developers can build reliable, efficient, and production-ready serverless
    applications with confidence.

    ## Features
    1. Serverless Application Lifecycle
    - Initialize, build, and deploy Serverless Application Model (SAM) applications with SAM CLI
    - Test Lambda functions locally and remotely
    2. Web Application Deployment & Management
    - Deploy fullstack, frontend, and backend web applications onto AWS Serverless using Lambda Web Adapter.
    - Update frontend assets and optionally invalidate CloudFront caches
    - Create custom domain names, including certificate and DNS setup.
    3. Observability
    - Retrieve and logs and metrics of serverless resources
    4. Guidance, Templates, and Deployment Help
    - Provides guidance on AWS Lambda use-cases, selecting an IaC framework, and deployment process onto AWS Serverless
    - Provides sample SAM templates for different serverless application types from [Serverless Land](https://serverlessland.com/)
    - Provides schema types for different Lambda event sources and runtimes
    5. Event Source Mapping (ESM) Tools
    - Setup, optimization, and troubleshooting for Lambda event sources
    - Support for Kafka, Kinesis, DynamoDB, and SQS event sources
    - Network configuration and performance optimization guidance

    ## Usage Notes
    - By default, the server runs in read-only mode. Use the `--allow-write` flag to enable write operations and public resource creation.
    - Access to sensitive data (Lambda function and API GW logs) requires the `--allow-sensitive-data-access` flag.

    ## Prerequisites
    1. Have an AWS account
    2. Configure AWS CLI with your credentials and profile. Set AWS_PROFILE environment variable if not using default
    3. Set AWS_REGION environment variable if not using default
    4. Install AWS CLI and SAM CLI