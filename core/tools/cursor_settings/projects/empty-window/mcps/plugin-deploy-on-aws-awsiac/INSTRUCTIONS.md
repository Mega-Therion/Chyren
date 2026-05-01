# AWS IaC MCP Server

                This server provides tools for AWS Infrastructure as Code development, including CloudFormation template validation, compliance checking, deployment troubleshooting, and AWS CDK documentation access.

                ## Tool Selection Guide

                - Use `validate_cloudformation_template` when: You need to validate CloudFormation template syntax, schema, and resource properties using cfn-lint
                - Use `check_cloudformation_template_compliance` when: You need to validate templates against security and compliance rules using cfn-guard
                - Use `cloudformation_pre_deploy_validation` when: You need instructions for pre-deployment validation using CloudFormation change sets to catch account-level issues
                - Use `troubleshoot_cloudformation_deployment` when: You need to diagnose CloudFormation deployment failures with root cause analysis and CloudTrail integration
                - Use `search_cdk_documentation` when: You need specific CDK construct APIs, properties, or official documentation from AWS CDK knowledge bases
                - Use `search_cdk_samples_and_constructs` when: You need working code examples, implementation patterns, or community constructs
                - Use `read_iac_documentation_page` when: You have specific documentation URLs from search results and need complete content with pagination support
                - Use `search_cloudformation_documentation` when: You need Cloudformation related official documentation, resource type information or template syntax
                - Use `cdk_best_practices` when: You need to generate or review CDK code