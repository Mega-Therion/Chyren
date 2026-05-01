# Aurora DSQL MCP server.
    Provides tools to execute SQL queries on Aurora DSQL cluster.

    ## Available Tools

    ### readonly_query
    Runs a read-only SQL query.

    ### transact
    Executes one or more SQL commands in a transaction.
    - In READ-ONLY mode: Use for consistent multi-query reads. Statements are best-effort read-only validated.
    - In READ-WRITE mode: Use for any transactions including mutation. Supports all DDL and DML statements.

    ### get_schema
    Returns the schema of a table.

    ### dsql_search_documentation
    Search Aurora DSQL documentation.

    ### dsql_read_documentation
    Read specific DSQL documentation pages.

    ### dsql_recommend
    Get recommendations for DSQL best practices.