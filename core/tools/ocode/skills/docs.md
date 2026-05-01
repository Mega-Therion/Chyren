---
name: docs
description: Generate documentation for code, APIs, or projects
triggers: ["docs", "documentation", "docstring", "readme", "explain function", "document", "add comments"]
author: ocode
---

# Documentation Skill

You are in documentation mode. Your goal is to write clear, accurate, and useful documentation.

## Principles

1. **Document intent, not mechanics**
   - Don't explain what the code does line-by-line (the code does that)
   - Explain WHY, WHEN, and HOW TO USE it
   - Document contracts: what it expects, what it guarantees

2. **Accuracy over completeness**
   - Wrong documentation is worse than no documentation
   - Read the code before writing docs
   - Run examples to verify they work

3. **Know your audience**
   - API docs: for users of the interface
   - Code comments: for maintainers
   - README: for people new to the project

## Documentation Types

### Docstrings (Python)
```python
def process_payment(amount: float, currency: str, customer_id: str) -> PaymentResult:
    """
    Process a payment for a customer.

    Args:
        amount: Payment amount in the given currency (must be > 0)
        currency: ISO 4217 currency code (e.g., "USD", "EUR")
        customer_id: Unique customer identifier

    Returns:
        PaymentResult with transaction_id and status

    Raises:
        ValueError: If amount <= 0 or currency is invalid
        PaymentError: If the payment processor rejects the payment

    Example:
        result = process_payment(49.99, "USD", "cust_123")
        if result.status == "success":
            print(f"Payment ID: {result.transaction_id}")
    """
```

### JSDoc (JavaScript/TypeScript)
```typescript
/**
 * Process a payment for a customer.
 *
 * @param amount - Payment amount in the given currency (must be > 0)
 * @param currency - ISO 4217 currency code
 * @param customerId - Unique customer identifier
 * @returns Promise resolving to PaymentResult
 * @throws {ValueError} If amount <= 0 or currency is invalid
 *
 * @example
 * const result = await processPayment(49.99, 'USD', 'cust_123');
 */
```

### Rust Doc Comments
```rust
/// Process a payment for a customer.
///
/// # Arguments
/// * `amount` - Payment amount (must be > 0)
/// * `currency` - ISO 4217 currency code
/// * `customer_id` - Unique customer identifier
///
/// # Returns
/// `Ok(PaymentResult)` on success, `Err(PaymentError)` on failure
///
/// # Examples
/// ```rust
/// let result = process_payment(49.99, "USD", "cust_123").await?;
/// ```
pub async fn process_payment(...) -> Result<PaymentResult, PaymentError> {
```

## README Structure
```markdown
# Project Name
One-line description.

## What it does
2-3 sentences explaining the problem it solves.

## Quick Start
[Minimal working example]

## Installation
[Step-by-step install]

## Usage
[Common usage patterns with examples]

## Configuration
[Config options table]

## API Reference
[If applicable]

## Development
[How to run tests, contribute]
```

## Process

1. Read the code/module to document
2. Understand the public interface and contracts
3. Run the code/tests to verify behavior
4. Write documentation starting with the most important parts
5. Add examples for non-obvious usage
6. Verify all examples are correct (run them if possible)

## What NOT to Document
- Don't add comments that just restate the code: `# increment i by 1` before `i += 1`
- Don't document obvious getters/setters
- Don't add "TODO: document this later" — write real docs or nothing
