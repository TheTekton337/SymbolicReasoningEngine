# SymbolicReasoningEngine

A powerful symbolic reasoning engine designed for dynamic knowledge inference and decision-making based on logical rules and facts.

## Overview

The SymbolicReasoningEngine is a flexible and extensible framework that enables symbolic reasoning processes. It allows for the representation and manipulation of knowledge through facts and rules, supporting the evaluation of complex logical expressions. This engine is capable of dynamic inference, making it suitable for a wide range of applications from artificial intelligence solutions to expert systems and educational tools.

## Features

- **Dynamic Knowledge Base**: Manage a growing knowledge base of facts that the engine uses for reasoning.
- **Logical Rule Evaluation**: Define rules with premises and conclusions to drive the inference process.
- **Variable Support**: Utilize variables within rules for dynamic and context-sensitive reasoning.
- **Forward Chaining**: Apply forward chaining logic to automatically derive new facts from existing ones.
- **Extensible Design**: Easily extend the engine to accommodate new types of logical operations or domain-specific optimizations.

## Getting Started

To get started with the SymbolicReasoningEngine, clone this repository and include it in your project. Ensure you have Rust installed on your system to compile and run the engine.

```bash
git clone https://github.com/TheTekton337/SymbolicReasoningEngine.git
cd SymbolicReasoningEngine
cargo build
```

## Usage Example

Here's a simple example of how to use the SymbolicReasoningEngine to define symbols, assert facts, add rules, and perform inference:

```rust
let mut engine = SymbolicReasoningEngine::new();

// Define symbols
let weather_symbol = engine.define_symbol("Weather", "String");
let temp_symbol = engine.define_symbol("Temperature", "Integer");
let activity_symbol = engine.define_symbol("Activity", "String");

// Assert facts: It is not raining, and the temperature is 25 degrees
engine.assert_fact(weather_symbol.clone(), FactValue::Text("NotRaining".to_string()));
engine.assert_variable("CurrentTemperature".to_string(), FactValue::Integer(25));

// Define the rule with nested logical expressions
engine.define_rule(
    LogicalOperator::And(vec![
        LogicalOperator::Or(vec![
            LogicalOperator::AtomicFact(Fact {
                symbol: weather_symbol.clone(),
                value: FactValue::Text("Sunny".to_string()),
            }),
            LogicalOperator::Not(Box::new(LogicalOperator::AtomicFact(Fact {
                symbol: weather_symbol.clone(),
                value: FactValue::Text("Raining".to_string()),
            }))),
        ]),
        LogicalOperator::AtomicFact(Fact {
            symbol: temp_symbol,
            value: FactValue::Comparison(
                Box::new(FactValue::Text("CurrentTemperature".to_string())),
                Comparison::GreaterThan,
                Box::new(FactValue::Integer(20))
            ),
        }),
    ]),
    Fact {
        symbol: activity_symbol.clone(),
        value: FactValue::Text("GoodForOutdoor".to_string()),
    }
);

// Perform forward chaining to infer new facts based on the rules
engine.forward_chaining_with_variables();

// Check for inferred facts
```

## Documentation

For detailed documentation on using the SymbolicReasoningEngine, including all available methods and their usage, please refer to the [doc](doc/) directory in this repository.

## Contributing

We welcome contributions to the SymbolicReasoningEngine project! Please refer to our [Contributing Guide](CONTRIBUTING.md) for more information on how to get involved.

## License

This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file for details.