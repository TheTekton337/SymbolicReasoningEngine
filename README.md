# SymbolicReasoningEngine

A powerful symbolic reasoning engine designed for dynamic knowledge inference and decision-making based on logical rules and facts.

## Overview

The SymbolicReasoningEngine is a flexible and extensible framework that enables symbolic reasoning processes. It allows for the representation and manipulation of knowledge through facts and rules, supporting the evaluation of complex logical expressions. This engine is capable of dynamic inference, making it suitable for a wide range of applications from artificial intelligence solutions to expert systems and educational tools.

## Features

- **Dynamic Knowledge Base**: Manage a growing knowledge base of facts that the engine uses for reasoning.
- **Logical Rule Evaluation**: Define rules with premises and conclusions to drive the inference process.
- **Variable Support**: (WIP) Utilize variables within rules for dynamic and context-sensitive reasoning.
- **Backward Chaining**: Apply backward chaining logic to search for matching goals within specified rules.
- **Forward Chaining**: Apply forward chaining logic to automatically derive new facts from existing ones.
- **Extensible Design**: Easily extend the engine to accommodate new types of logical operations or domain-specific optimizations.

## Getting Started

To get started with the SymbolicReasoningEngine, clone this repository and include it in your project. Ensure you have Rust installed on your system to compile and run the engine.

```bash
git clone https://github.com/TheTekton337/SymbolicReasoningEngine.git
cd SymbolicReasoningEngine
cargo build
```

## Usage Examples

Here's a simple example of how to use the SymbolicReasoningEngine to define symbols, assert facts, add rules, and perform inference with forward chaining:

```rust
let mut engine = SymbolicReasoningEngine::new();

// Define symbols
let weather_symbol = engine.define_symbol("Weather", "String");
let activity_symbol = engine.define_symbol("Activity", "String");

// Assert the fact: It is sunny
engine.assert_fact(weather_symbol.clone(), FactValue::Text("Sunny".to_string()));

// Define the rule: If it is sunny, then it's a good day for outdoor activity
engine.define_rule(
    LogicalOperator::AtomicFact(Fact {
        symbol: weather_symbol.clone(),
        value: FactValue::Text("Sunny".to_string()),
    }),
    Fact {
        symbol: activity_symbol.clone(),
        value: FactValue::Text("Outdoor".to_string()),
    },
);

// Perform forward chaining to infer new facts based on the rules
engine.forward_chaining();

// Check if the new fact (good day for outdoor activity) is added to the knowledge base
assert!(engine.facts.contains(&Fact {
    symbol: activity_symbol,
    value: FactValue::Text("Outdoor".to_string()),
}), "The engine did not infer that it's a good day for outdoor activity when it's sunny.");
```

Here is a simple example of how to use the SymbolicReasoningEngine to define symbols, assert facts, add rules, and perform inference via backward chaining:

```rust
let mut engine = SymbolicReasoningEngine::new();

// Define symbols
let weather = engine.define_symbol("Weather", "String");
let temperature = engine.define_symbol("Temperature", "Integer");

// Assert known facts into the engine's knowledge base
engine.assert_fact(temperature.clone(), FactValue::Integer(25));
engine.assert_fact(weather.clone(), FactValue::Text(String::from("Sunny")));

// Rule 1: If temperature > 20, then it's warm
let warm = engine.define_symbol("warm", "Boolean");
engine.define_rule(
    LogicalOperator::GreaterThan(
        Box::new(ComparableValue::Symbol(temperature.clone())),
        Box::new(ComparableValue::Direct(FactValue::Integer(20)))
    ),
    // Fact::new(warm.clone(), FactValue::Boolean(true))
    Fact::new(warm.clone(), FactValue::Text("Warm".into()))
);

// Rule 2: If it's warm and sunny, then it's a good day for a picnic
let picnic_day = engine.define_symbol("picnic_advisable", "Boolean");
engine.define_rule(
    LogicalOperator::And(vec![
        LogicalOperator::AtomicFact(Fact::new(warm.clone(), FactValue::Text("Warm".into()))),
        LogicalOperator::AtomicFact(Fact::new(weather.clone(), FactValue::Text(String::from("Sunny"))))
    ]),
    Fact::new(picnic_day.clone(), FactValue::Boolean(true))
);

// Specify the goal: To determine if it's a picnic day
let goal = Fact::new(picnic_day, FactValue::Boolean(true));
let is_picnic_day = engine.specify_goal(&goal);

// Assert that the engine successfully determines it's a beach day through backward chaining
assert!(is_picnic_day, "The engine should successfully determine it's a picnic day through backward chaining.");
```

## Documentation

For detailed documentation on using the SymbolicReasoningEngine, including all available methods and their usage, please refer to the [doc](doc/) directory in this repository.

## Contributing

We welcome contributions to the SymbolicReasoningEngine project! Please refer to our [Contributing Guide](CONTRIBUTING.md) for more information on how to get involved.

## License

This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file for details.