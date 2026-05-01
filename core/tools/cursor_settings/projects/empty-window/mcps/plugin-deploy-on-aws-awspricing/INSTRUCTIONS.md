This server provides two primary functionalities:

    # USE CASE 1: AWS SERVICE CATALOG & PRICING DISCOVERY
    Access AWS service catalog information and pricing details through a structured workflow:

    1. Discovery Workflow:
       - get_pricing_service_codes: Retrieve all available AWS service codes (starting point)
       - get_pricing_service_attributes: Get filterable attributes for a specific service
       - get_pricing_attribute_values: Get possible values for a specific attribute
       - get_pricing: Get actual pricing data with optional filters
       - get_price_list_urls: Get bulk pricing data files in multiple formats (CSV, JSON) for historical pricing analysis

    2. Example Discovery Flow:
       ```
       # Get all service codes to find the one you need
       service_codes = get_pricing_service_codes()

       # Get available attributes for filtering EC2 pricing
       attributes = get_pricing_service_attributes('AmazonEC2')

       # Get all possible instance types for EC2
       instance_types = get_pricing_attribute_values('AmazonEC2', 'instanceType')

       # Get pricing for specific instance types in a region
       filters = [{"Field": "instanceType", "Value": "t3.medium", "Type": "EQUALS"}]
       pricing = get_pricing('AmazonEC2', 'us-east-1', filters)

       # Get bulk pricing data files for historical analysis
       price_list = get_price_list_urls('AmazonEC2', 'us-east-1')
       # Returns: {'arn': '...', 'urls': {'csv': 'https://...', 'json': 'https://...'}}

       # If alternatives are applicable to the use case, retrieve their pricing data (e.g., CloudFrontPlans for AmazonCloudFront)
       for alt in pricing['alternatives']: get_pricing(alt)
       ```

    # USE CASE 2: COST ANALYSIS REPORT GENERATION
    Generate comprehensive cost reports for AWS services by following these steps:

    1. Data Gathering: Invoke get_pricing() to fetch data via AWS Pricing API

    2. Service-Specific Analysis:
       - For Bedrock Services: MUST also use get_bedrock_patterns()
       - This provides critical architecture patterns, component relationships, and cost considerations
       - Especially important for Knowledge Base, Agent, Guardrails, and Data Automation services

    3. Report Generation:
       - MUST generate cost analysis report using retrieved data via generate_cost_report()
       - The report includes sections for:
         * Service Overview
         * Architecture Pattern (for Bedrock services)
         * Assumptions
         * Limitations and Exclusions
         * Cost Breakdown
         * Cost Scaling with Usage
         * AWS Well-Architected Cost Optimization Recommendations

    4. Output:
       Return to user:
       - Detailed cost analysis report in markdown format
       - Source of the data (web scraping, API, or websearch)
       - List of attempted data retrieval methods

    ACCURACY GUIDELINES:
    - When uncertain about service compatibility or pricing details, EXCLUDE them rather than making assumptions
    - For database compatibility, only include CONFIRMED supported databases
    - For model comparisons, always use the LATEST models rather than specific named ones
    - Add clear disclaimers about what is NOT included in calculations
    - PROVIDING LESS INFORMATION IS BETTER THAN GIVING WRONG INFORMATION
    - For Bedrock Knowledge Base, ALWAYS account for OpenSearch Serverless minimum OCU requirements (2 OCUs, $345.60/month minimum)
    - For Bedrock Agent, DO NOT double-count foundation model costs (they're included in agent usage)

    IMPORTANT: For report generation, steps MUST be executed in the exact order specified. Each step must be attempted
    before moving to the next fallback mechanism. The report is particularly focused on
    serverless services and pay-as-you-go pricing models.