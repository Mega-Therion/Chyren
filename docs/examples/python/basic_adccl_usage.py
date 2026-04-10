#!/usr/bin/env python3
"""
Basic ADCCL Usage Example (Python)

This example demonstrates how to use Chyren's Python bindings to perform
ADCCL compliance checks and integrate sovereign intelligence verification
into your Python applications.
"""

import asyncio
import logging
from typing import Optional

try:
    from chyren import AdcclGate, AdcclConfig, State, ChyrenError
except ImportError:
    print("Error: Chyren Python bindings not installed.")
    print("Install with: pip install chyren")
    exit(1)

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


async def example_1_basic_check():
    """
    Example 1: Basic ADCCL compliance check
    """
    print("\n=== Example 1: Basic ADCCL Check ===")
    
    # Initialize ADCCL gate with default configuration
    config = AdcclConfig(
        strictness=0.5,
        enable_preflight=True,
        gate_threshold=0.7
    )
    gate = AdcclGate(config)
    
    print(f"✓ ADCCL Gate initialized (strictness: {config.strictness})\n")
    
    # Create a state for a user action
    state = State(action="user_login")
    state.set_context("web_app")
    state.set_metadata("user_id", "user_12345")
    state.set_metadata("ip_address", "192.168.1.100")
    
    # Perform preflight check
    try:
        result = await gate.preflight_check(state)
        
        print("Preflight Result:")
        print(f"  Status: {'✓ PASS' if result.is_approved else '✗ FAIL'}")
        print(f"  Score: {result.score:.2f}")
        print(f"  Chirality: {result.chirality}")
        print(f"  Gate Status: {result.gate_status}\n")
        
        if result.is_approved:
            print("  → Action APPROVED: Proceed with user login")
        else:
            print(f"  → Action BLOCKED: {result.reason}")
            
    except ChyrenError as e:
        logger.error(f"ADCCL check failed: {e}")


async def example_2_risk_levels():
    """
    Example 2: Testing different risk levels
    """
    print("\n=== Example 2: Risk Level Testing ===")
    
    gate = AdcclGate(AdcclConfig(strictness=0.6))
    
    test_actions = [
        ("read_public_data", "Low risk action"),
        ("update_user_profile", "Medium risk action"),
        ("delete_all_user_data", "High risk action"),
        ("export_database", "Critical risk action"),
    ]
    
    for action, description in test_actions:
        state = State(action=action)
        state.set_metadata("description", description)
        
        try:
            result = await gate.preflight_check(state)
            status_icon = "✓" if result.is_approved else "✗"
            
            print(f"\n{description}:")
            print(f"  Action: {action}")
            print(f"  {status_icon} Score: {result.score:.2f}")
            print(f"  Gate: {result.gate_status}")
            
        except ChyrenError as e:
            logger.error(f"Check failed for {action}: {e}")


async def example_3_custom_strictness():
    """
    Example 3: Using custom strictness levels
    """
    print("\n=== Example 3: Custom Strictness Levels ===")
    
    strictness_levels = [0.3, 0.5, 0.8]
    action = "sensitive_data_access"
    
    for strictness in strictness_levels:
        config = AdcclConfig(strictness=strictness)
        gate = AdcclGate(config)
        
        state = State(action=action)
        state.set_context("api_request")
        
        try:
            result = await gate.preflight_check(state)
            
            print(f"\nStrictness {strictness}:")
            print(f"  Approved: {result.is_approved}")
            print(f"  Score: {result.score:.2f}")
            print(f"  Interpretation: ", end="")
            
            if strictness < 0.4:
                print("Permissive (most actions pass)")
            elif strictness < 0.7:
                print("Balanced (moderate filtering)")
            else:
                print("Strict (high security requirements)")
                
        except ChyrenError as e:
            logger.error(f"Check failed at strictness {strictness}: {e}")


async def example_4_chirality_detection():
    """
    Example 4: Demonstrating chirality detection
    """
    print("\n=== Example 4: Chirality Detection ===")
    
    gate = AdcclGate(AdcclConfig())
    
    # Test deterministic operation (RIGHT-HANDED)
    deterministic_state = State(action="hash_calculation")
    deterministic_state.set_metadata("operation_type", "deterministic")
    deterministic_state.set_metadata("algorithm", "SHA-256")
    
    # Test probabilistic operation (LEFT-HANDED)
    probabilistic_state = State(action="ml_inference")
    probabilistic_state.set_metadata("operation_type", "probabilistic")
    probabilistic_state.set_metadata("model", "neural_network")
    
    for state, name in [(deterministic_state, "Deterministic"), (probabilistic_state, "Probabilistic")]:
        try:
            result = await gate.preflight_check(state)
            
            print(f"\n{name} Operation:")
            print(f"  Chirality: {result.chirality}")
            print(f"  Explanation: ", end="")
            
            if result.chirality == "RIGHT-HANDED":
                print("Deterministic/Verifiable outcome")
            elif result.chirality == "LEFT-HANDED":
                print("Probabilistic/Uncertain outcome")
            else:
                print("Neutral/Achiral")
                
        except ChyrenError as e:
            logger.error(f"Check failed for {name}: {e}")


async def example_5_batch_processing():
    """
    Example 5: Batch processing multiple actions
    """
    print("\n=== Example 5: Batch Processing ===")
    
    gate = AdcclGate(AdcclConfig(strictness=0.5))
    
    actions = [
        "user_registration",
        "data_export",
        "password_reset",
        "admin_access",
        "file_upload"
    ]
    
    results = []
    
    # Process all actions
    for action in actions:
        state = State(action=action)
        try:
            result = await gate.preflight_check(state)
            results.append((action, result))
        except ChyrenError as e:
            logger.error(f"Failed to check {action}: {e}")
    
    # Summary report
    approved = sum(1 for _, r in results if r.is_approved)
    blocked = len(results) - approved
    
    print(f"\nBatch Processing Summary:")
    print(f"  Total actions: {len(results)}")
    print(f"  ✓ Approved: {approved}")
    print(f"  ✗ Blocked: {blocked}")
    print(f"  Approval rate: {(approved/len(results)*100):.1f}%\n")
    
    print("Detailed Results:")
    for action, result in results:
        status = "✓" if result.is_approved else "✗"
        print(f"  {status} {action}: {result.score:.2f}")


async def main():
    """
    Run all examples
    """
    print("\n" + "="*50)
    print("Chyren ADCCL Python Examples")
    print("Demonstrating sovereign intelligence verification")
    print("="*50)
    
    try:
        await example_1_basic_check()
        await example_2_risk_levels()
        await example_3_custom_strictness()
        await example_4_chirality_detection()
        await example_5_batch_processing()
        
        print("\n" + "="*50)
        print("All examples completed successfully!")
        print("="*50 + "\n")
        
    except Exception as e:
        logger.error(f"Example execution failed: {e}")
        raise


if __name__ == "__main__":
    # Run the async examples
    asyncio.run(main())
