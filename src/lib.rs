use std::collections::HashMap;

/// Represents a symbol in the symbolic reasoning engine.
///
/// A symbol is a basic unit of meaning, identified by a name and associated with a type.
/// Symbols are used to construct facts and define the relationship between facts and rules within the engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    name: String,
    symbol_type: String,
}

impl Symbol {
    pub fn new(name: &str, symbol_type: &str) -> Self {
        Self {
            name: name.to_string(),
            symbol_type: symbol_type.to_string(),
        }
    }
}

/// Represents a fact in the symbolic reasoning engine.
///
/// A fact associates a symbol with a specific value, contributing to the knowledge base of the engine.
/// Facts are used in conjunction with rules to infer new information or make decisions based on the engine's current state.
#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    symbol: Symbol,
    value: FactValue, // Simplified to boolean for this example
}

impl Fact {
    pub fn new(symbol: Symbol, value: FactValue) -> Self {
        Self { symbol, value }
    }
}

/// Represents the possible values that can be associated with a symbol in a fact.
///
/// This enumeration covers basic data types such as integers, floats, booleans, and text strings,
/// allowing for a wide range of information to be represented and manipulated within the engine.
#[derive(Debug, Clone, PartialEq)]
pub enum FactValue {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Text(String),
}

/// Represents a value that can be compared within the rule engine, encapsulating different types of comparable values.
///
/// This enum is used to abstract the various ways a value can be represented or referenced in the context of rule evaluation, especially in conditions that involve
/// comparisons. It allows for direct value comparisons, references to values through symbols, or the use of symbol names as strings. This flexibility is crucial for
/// supporting complex rule definitions and evaluations.
///
/// Variants:
/// - `Direct(FactValue)`: Represents a directly provided value, such as an integer or a float, wrapped in the `FactValue` enum. This variant is used when the value
///   to be compared is explicitly given.
/// - `Symbol(Symbol)`: Represents a reference to a value by using a `Symbol`, which is a more structured way to refer to facts within the rule engine. This variant
///   is used when the value for comparison is to be fetched from the current set of facts based on a symbol.
/// - `SymbolName(String)`: Similar to the `Symbol` variant, but uses a plain string to refer to the symbol name. This variant allows for a more flexible or dynamic
///   reference to values within the rule engine, especially when symbol names are generated or not known at compile time.
///
/// This enum is integral to the rule engine's ability to evaluate conditions involving comparisons, ensuring flexibility and robustness in how values are specified
/// and retrieved for comparison operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ComparableValue {
    Direct(FactValue),
    Symbol(Symbol),
    SymbolName(String),
}

/// Represents the logical operators used to construct complex logical expressions within the symbolic reasoning engine.
///
/// This enum allows for the building of nested logical expressions that can combine multiple conditions using logical
/// operators such as AND, OR, and NOT. These expressions form the basis of rule premises and conclusions, enabling
/// sophisticated reasoning based on the accumulated facts and the current state of variable bindings in the engine.
///
/// Variants:
/// - `And`: Represents a logical AND operation, requiring all contained expressions to evaluate to true for the
///   overall expression to be considered true. Used to combine multiple conditions that must all be satisfied.
/// - `Or`: Represents a logical OR operation, requiring at least one of the contained expressions to evaluate to true
///   for the overall expression to be considered true. Used to specify alternative conditions that can satisfy a rule.
/// - `Not`: Represents a logical NOT operation, negating the truth value of the contained expression. Used to invert
///   the evaluation of a condition, specifying that a particular situation or condition must not be true.
/// - `AtomicFact`: Directly associates a fact (a symbol-value pair) with the logical expression, allowing for the direct
///   inclusion of specific facts within logical operations. This enables conditions to be directly tied to concrete
///   pieces of knowledge within the engine's domain.
///
/// LogicalOperator enables the symbolic reasoning engine to evaluate complex conditions involving multiple
/// facts and variables, providing a flexible mechanism for defining the logic that drives inference and decision-making.
/// These operators are essential for expressing dependencies and relationships between facts, underpinning the
/// engine's ability to reason about and interpret the data it manages.
#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And(Vec<LogicalOperator>),
    Or(Vec<LogicalOperator>),
    Not(Box<LogicalOperator>),
    AtomicFact(Fact),
    GreaterThan(Box<ComparableValue>, Box<ComparableValue>),
    LessThan(Box<ComparableValue>, Box<ComparableValue>),
    EqualTo(Box<ComparableValue>, Box<ComparableValue>),
    NotEqualTo(Box<ComparableValue>, Box<ComparableValue>),
    GreaterThanOrEqualTo(Box<ComparableValue>, Box<ComparableValue>),
    LessThanOrEqualTo(Box<ComparableValue>, Box<ComparableValue>),
}

/// Represents a rule within the symbolic reasoning engine.
///
/// A rule is a fundamental construct in the engine that defines a logical relationship between a set of conditions
/// (the premise) and a conclusion. Rules are used to infer new facts or make decisions based on the current state
/// of the knowledge base and variable bindings. When the conditions specified in the premise are met, the conclusion
/// of the rule is considered to be true, potentially leading to the assertion of new facts or the triggering of
/// further logical operations.
///
/// The `Rule` struct encapsulates this logic, containing both the premise and conclusion as logical expressions.
/// The premise is represented as a `LogicalOperator`, which can combine multiple conditions using logical
/// operators like AND, OR, and NOT. The conclusion is represented as a `Fact`, specifying the outcome or assertion
/// that should be made when the premise evaluates to true.
///
/// Attributes:
/// - `premise`: A `LogicalOperator` representing the combination of conditions that must be satisfied for the rule
///   to be applied. This can involve complex nested logical expressions involving facts and variables.
/// - `conclusion`: A `Fact` that specifies what should be considered true or asserted within the engine's knowledge
///   base when the premise is satisfied. The conclusion contributes to the engine's dynamic knowledge, influencing
///   subsequent reasoning and decision-making.
///
/// Rules play a critical role in the engine's operation, allowing for the dynamic evolution of the knowledge base
/// through logical inference based on defined conditions and relationships. They enable the engine to model and
/// reason about complex scenarios, facilitating sophisticated decision-making processes.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Rule {
    premise: LogicalOperator,
    conclusion: Fact,
}

/// Represents the core of the symbolic reasoning engine.
///
/// This struct encapsulates the main functionality of the engine, providing mechanisms for managing a knowledge base,
/// asserting facts and variables, and applying rules to infer new information or make decisions based on logical conditions.
/// The engine operates on a collection of symbols, facts, and rules, utilizing logical operators to evaluate complex
/// expressions that drive the inference process.
///
/// The `SymbolicReasoningEngine` leverages a combination of structured data representations (`Symbol`, `Fact`, `Rule`, etc.)
/// and algorithms for logical evaluation (such as forward chaining) to simulate reasoning processes. This enables the
/// application of the engine across various domains where decision-making and inference are required, from artificial
/// intelligence applications to expert systems and beyond.
///
/// Attributes:
/// - `facts`: A collection of `Fact` instances representing the current state of knowledge within the engine. Facts
///   are assertions about symbols and their values that the engine considers to be true.
/// - `rules`: A collection of `Rule` instances that define the logical relationships and conditions under which new
///   facts can be inferred or actions can be taken. Rules form the basis of the engine's reasoning capabilities.
/// - `variable_bindings`: A mapping of variable names to their `FactValue` instances. This allows the engine to handle
///   dynamic values and conditions within rules and logical expressions, enhancing the engine's flexibility and
///   applicability to real-world scenarios.
///
/// Usage:
/// The engine is used by first defining the necessary symbols, facts, and rules that represent the domain of interest.
/// Variables can be asserted to introduce dynamic elements into the reasoning process. The engine then applies the rules
/// against the current set of facts, using logical evaluation to infer new facts or make decisions based on the defined
/// logical conditions. This process can be repeated or iterated as new information becomes available or as the state of
/// the system evolves.
///
/// Example:
/// ```
/// // let mut engine = SymbolicReasoningEngine::new();
/// // engine.assert_fact(...); // Assert initial facts
/// // engine.assert_variable(...); // Define variables and their bindings
/// // engine.add_rule(...); // Define rules
/// // engine.forward_chaining_with_variables(); // Perform inference to update the knowledge base
/// ```
///
/// The `SymbolicReasoningEngine` is designed to be extensible, allowing for the integration of additional functionality
/// and optimizations to suit specific requirements or to enhance its reasoning capabilities.
#[allow(dead_code)]
pub struct SymbolicReasoningEngine {
    symbols: HashMap<String, Symbol>,
    facts: Vec<Fact>,
    rules: Vec<Rule>,
    variable_bindings: HashMap<String, FactValue>,
    debug: bool,
}

/// Implementation block for `SymbolicReasoningEngine`.
///
/// This implementation provides the methods necessary for operating the symbolic reasoning engine, including
/// the management of facts, rules, and variable bindings. It encapsulates the logic for asserting facts and variables,
/// adding rules, and executing the inference process to derive new information based on the engine's current state.
///
/// Methods within this implementation facilitate the dynamic interaction with the engine's knowledge base, allowing
/// users to construct, query, and manipulate the domain-specific logic represented within. The core functionalities
/// include the ability to:
///
/// - Assert facts (`assert_fact`) about the domain, adding them to the engine's knowledge base.
/// - Define variables (`assert_variable`) with specific values, enhancing the flexibility of rule evaluation.
/// - Add logical rules (`add_rule`) that dictate the inference process based on conditions and conclusions.
/// - Execute the inference process (`infer`), applying the defined rules to the current set of facts and variables,
///   leading to the derivation of new facts or decisions.
///
/// Additional utility methods may be included to support querying the knowledge base, inspecting the current state of
/// variables and facts, and other operations that facilitate the analysis and debugging of the engine's behavior.
///
/// The `SymbolicReasoningEngine` aims to provide a comprehensive framework for symbolic reasoning, supporting a wide
/// range of applications from automated decision-making systems to educational tools for logic and reasoning.
#[allow(dead_code)]
impl SymbolicReasoningEngine {
    /// Creates a new instance of the symbolic reasoning engine, initializing an empty knowledge base,
    /// an empty set of rules, and no variable bindings.
    ///
    /// Returns a `SymbolicReasoningEngine` instance ready for the definition of symbols, facts, rules, and variables.
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            facts: Vec::new(),
            rules: Vec::new(),
            variable_bindings: HashMap::new(),
            debug: false
        }
    }

    /// Enables the debugging mode for the rule engine.
    ///
    /// This method sets the internal `debug` flag of the rule engine to `true`, activating the output of debug messages across the engine. When debug mode is enabled,
    /// calls to `print_debug` will result in messages being printed to the console, providing insights into the engine's operations, decision-making processes, and
    /// flow of execution. This feature is beneficial for development, testing, and troubleshooting, allowing developers to track how rules are evaluated and facts
    /// are managed.
    ///
    /// Enabling debug mode is a crucial tool for developers to understand the inner workings of the rule engine, diagnose problems, and ensure that the logic
    /// of rule evaluation and fact assertion behaves as expected.
    fn enable_debug(&mut self) {
        self.debug = true;
    }

    /// Prints a debug message to the console if debugging is enabled for the rule engine.
    ///
    /// This method is a utility function used throughout the rule engine to output debug information. It checks the internal `debug` flag
    /// of the rule engine instance before deciding to print the message. This allows for verbose output during development or troubleshooting
    /// without cluttering the output in production environments where debugging is turned off.
    ///
    /// # Arguments
    /// * `message` - A reference to a `str` that contains the debug message to be printed.
    ///
    /// This method simplifies the process of inserting debug statements throughout the rule engine's codebase, allowing for easy activation or deactivation
    /// of these messages based on the `debug` flag. It's particularly useful for tracking the flow of execution, inspecting variable states, and diagnosing
    /// issues with rule evaluation or fact assertion.
    fn print_debug(&self, message: &str) {
        if self.debug {
            println!("{}", message);
        }
    }

    /// Defines a new symbol within the symbolic reasoning engine's context.
    ///
    /// This method is responsible for creating and registering a new symbol, which represents a fundamental
    /// element of the knowledge domain being modeled by the engine. Symbols are key to constructing facts
    /// and rules, serving as identifiers that link values or conditions within the engine's logic.
    ///
    /// Each symbol is uniquely identified by its name, and associated with a specific type that determines
    /// the kind of values it can hold or be compared against in facts and rules. Common types include "Integer",
    /// "Float", "Boolean", and "Text", among others that the engine supports.
    ///
    /// # Arguments
    /// * `name` - A string slice (`&str`) representing the unique name of the symbol. This name is used to
    ///   reference the symbol in facts, rules, and logical expressions.
    /// * `symbol_type` - A string slice (`&str`) specifying the type of the symbol. The type influences how
    ///   the symbol can be used in the engine, particularly in comparisons and evaluations.
    ///
    /// # Returns
    /// Returns a `Symbol` instance representing the newly defined symbol. This instance can be used directly
    /// in the construction of facts and rules, integrating the symbol into the engine's reasoning processes.
    ///
    /// # Example
    /// ```
    /// //let mut engine = SymbolicReasoningEngine::new();
    /// //let temperature_symbol = engine.define_symbol("temperature", "Integer");
    /// // Now, "temperature" can be used as a symbol in facts and rules, associated with integer values.
    /// ```
    ///
    /// # Panics
    /// This method may panic if an attempt is made to define a symbol with a name that already exists within
    /// the engine's context, ensuring that symbol names remain unique and unambiguous.
    ///
    /// It is essential to define symbols before using them in facts and rules, as they establish the basic
    /// vocabulary for expressing the knowledge and logic encapsulated by the engine.
    fn define_symbol(&mut self, name: &str, symbol_type: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            symbol_type: symbol_type.to_string(),
        };
        if self.find_symbol(name) {
            panic!("Symbol already defined");
        }
        self.symbols.insert(name.to_string(), symbol.clone());
        symbol
    }

    /// Checks if a symbol with the specified name exists within the rule engine's symbol table.
    ///
    /// This method is used to determine whether a symbol (representing a fact or a variable) has been defined within the engine's knowledge base.
    /// It's a straightforward lookup operation that aids in verifying the presence of symbols before they are used in rule evaluations or when
    /// adding new facts or rules that reference these symbols.
    ///
    /// # Arguments
    /// * `name` - A reference to a `str` representing the name of the symbol to look for.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the symbol is found within the rule engine's symbol table; otherwise, it returns `false`.
    ///
    /// This method is essential for ensuring that references to symbols in rules and facts are valid, thereby preventing the rule engine from attempting
    /// to evaluate rules or facts with undefined symbols.
    fn find_symbol(&mut self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Asserts or updates a variable with a specified value in the engine's context.
    ///
    /// This method is responsible for binding a variable name to a value within the engine's variable bindings.
    /// It should be called prior to asserting facts that depend on these variables to ensure that the variable
    /// values are correctly set and can be resolved during rule evaluation and fact matching.
    ///
    /// Variables asserted using this method can be used within `FactValue::Comparison` or directly as `FactValue`
    /// in facts, enabling dynamic behavior in rule evaluation based on variable values.
    ///
    /// # Arguments
    /// * `var_name` - A `String` representing the name of the variable to be asserted or updated. This name
    ///   is used as the key to store the variable's value in the engine's bindings and must be unique within
    ///   the context of a rule evaluation session.
    /// * `value` - A `FactValue` representing the value to be bound to the variable. This value can be of any
    ///   supported `FactValue` type (e.g., `Integer`, `Float`, `Boolean`, `Text`).
    ///
    /// # Usage
    /// ```
    /// // Example usage within the symbolic reasoning engine
    /// // engine.assert_variable("temperature", FactValue::Integer(25));
    /// // Now, the engine knows that "temperature" is bound to the value 25
    /// // This variable can then be used in asserting facts or within rule conditions
    /// ```
    ///
    /// # Note
    /// Attempting to assert a variable that already exists in the bindings will update its value. This allows
    /// for dynamic adjustments of variable values based on new information or changing conditions within the
    /// engine's execution context.
    fn assert_variable(&mut self, var_name: String, value: FactValue) {
        // Insert or update the variable's value in the bindings
        self.variable_bindings.insert(var_name, value);
        self.print_debug("Variable asserted/updated in the bindings.");
    }

    /// Asserts a new fact into the engine's knowledge base.
    ///
    /// This method allows for the addition of a new fact, composed of a symbol and its associated value,
    /// to the symbolic reasoning engine. Facts represent concrete pieces of knowledge or assertions about
    /// the domain being modeled by the engine. Once asserted, facts can be used in conjunction with rules
    /// to drive the inference process, enabling the engine to derive new information or make decisions
    /// based on the logical structure defined by the rules and the current state of the knowledge base.
    ///
    /// # Arguments
    /// * `symbol` - The `Symbol` that identifies the fact. This symbol acts as a label or key for the fact,
    ///   categorizing it within a specific domain of knowledge (e.g., "temperature", "humidity").
    /// * `value` - The `FactValue` associated with the symbol. This value represents the actual data or
    ///   assertion being made about the symbol (e.g., an integer value of 25 for "temperature").
    ///
    /// # Examples
    /// ```
    /// // Assuming an instance `engine` of `SymbolicReasoningEngine` and a symbol for "temperature"
    /// //let temperature_symbol = engine.define_symbol("temperature", "Integer");
    /// // Assert the fact that the temperature is 25 degrees
    /// //engine.assert_fact(temperature_symbol, FactValue::Integer(25));
    /// ```
    ///
    /// By asserting facts, users of the engine can populate the knowledge base with relevant information
    /// required for logical evaluation and reasoning. This mechanism supports dynamic updates to the
    /// engine's understanding of the domain, reflecting changes in conditions or the discovery of new
    /// information.
    fn assert_fact(&mut self, symbol: Symbol, value: FactValue) {
        // Proceed to add the fact to the engine's knowledge base
        self.facts.push(Fact { symbol, value });
    }

    /// A placeholder method to determine or retrieve the value of a variable.
    /// This will involve looking up the value from a data source,
    /// user input, or another mechanism based on the application's requirements.
    ///
    /// # Arguments
    /// * `var_name` - The name of the variable whose value is to be determined.
    ///
    /// # Returns
    /// The determined `FactValue` for the variable.
    pub fn determine_variable_value(&self, var_name: &str) -> Option<&FactValue> {
        // Placeholder implementation
        // The actual implementation would depend on how variable values are provided or determined
        self.variable_bindings.get(var_name)
    }

    /// Defines and adds a new rule to the engine's set of logical rules.
    ///
    /// This method creates a rule based on a logical premise and a corresponding conclusion. Rules are central
    /// to the engine's inference process, allowing it to derive new facts or make decisions based on the
    /// evaluation of premises against the current state of the knowledge base and variable bindings. When the
    /// conditions described in the premise are met, the rule's conclusion is asserted as a new fact, expanding
    /// the knowledge base or influencing subsequent reasoning processes.
    ///
    /// # Arguments
    /// * `premise` - A `LogicalOperator` that represents the condition or set of conditions that must be satisfied
    ///   for the rule to be applied. The premise can involve complex logical constructs, including combinations of
    ///   AND, OR, and NOT operations, potentially nested to represent intricate logical relationships.
    /// * `conclusion` - A `Fact` that specifies the outcome or assertion to be made when the premise of the rule
    ///   evaluates to true. This fact is added to the knowledge base, representing new information derived from
    ///   applying the rule.
    ///
    /// # Examples
    /// ```
    /// // Assuming an instance `engine` of `SymbolicReasoningEngine`
    /// // Define symbols and facts as prerequisites
    /// //let temperature_symbol = engine.define_symbol("temperature", "Integer");
    /// //let hot_weather_symbol = engine.define_symbol("weather", "Text");
    ///
    /// // Define a rule stating that if the temperature is greater than 30 degrees, it is considered hot weather
    /// //engine.define_rule(
    /// //    LogicalOperator::GreaterThan(
    /// //        Fact::new(temperature_symbol.clone(), FactValue::Integer(30))
    /// //    ),
    /// //    Fact::new(hot_weather_symbol.clone(), FactValue::Text(String::from("Hot")))
    /// //);
    /// ```
    ///
    /// This example demonstrates defining a rule that interprets high temperature as an indication of hot weather.
    /// Such rules enable the engine to reason about the domain, making logical inferences that enrich its understanding
    /// and guide its decision-making processes.
    fn define_rule(&mut self, premise: LogicalOperator, conclusion: Fact) {
        let rule = Rule { premise, conclusion };
        self.rules.push(rule);
    }

    /// Retrieves a reference to a `Fact` from the knowledge base using a given symbol.
    ///
    /// This method searches the knowledge base for a `Fact` that matches the provided `Symbol`. If a matching `Fact` is found,
    /// it returns a reference to that `Fact`. This is particularly useful for operations that need to resolve symbols to their
    /// corresponding facts, such as when evaluating conditions in rules or when resolving `ComparableValue::Symbol` values.
    ///
    /// # Arguments
    /// * `symbol` - The `Symbol` for which to find the corresponding `Fact` in the knowledge base.
    ///
    /// # Returns
    /// * `Option<&Fact>` - A reference to the matching `Fact` if found; otherwise, `None`.
    ///
    /// This method is essential for the rule engine's ability to dynamically access facts in the knowledge base using symbols.
    /// It facilitates the translation of symbolic references into concrete data, enabling the evaluation of rules and logical expressions
    /// that depend on the current state of the knowledge base.
    fn get_fact_from_symbol(&self, symbol: Symbol) -> Option<&Fact> {
        self.facts.iter().find_map(|known_fact| {
            if known_fact.symbol == symbol {
                Some(known_fact)
            } else {
                None
            }
        })
    }

    /// Retrieves the value associated with a given fact from the knowledge base.
    ///
    /// # Arguments
    /// * `fact` - A reference to the `Fact` whose value is to be retrieved.
    ///
    /// # Returns
    /// * An `Option<f64>` representing the numerical value of the fact for comparison purposes.
    ///   Returns `None` if the fact is not found or if its value cannot be represented as a number.
    fn get_fact_value(&self, fact: &Fact) -> Option<f64> {
        self.facts.iter().find_map(|known_fact| {
            if known_fact.symbol == fact.symbol {
                match known_fact.value {
                    FactValue::Integer(value) => Some(value as f64),
                    FactValue::Float(value) => Some(value),
                    // Handle other FactValue variants that can be represented as a number or return None
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    /// Evaluates whether a given logical expression, serving as a rule's premise, is true based on the current knowledge base and variable bindings.
    ///
    /// This method is a key component of the engine's inference mechanism, allowing it to determine if the conditions
    /// specified by a rule's premise are satisfied. It recursively evaluates logical expressions, which can be composed
    /// of simple facts, combinations of logical operators (AND, OR, NOT), or comparisons involving variables. The evaluation
    /// takes into account the facts currently asserted in the knowledge base as well as the values of any variables
    /// referenced within the expression. A premise is considered true if its logical evaluation against the known facts
    /// and variable states yields a positive result, enabling the associated rule's conclusion to be potentially asserted.
    ///
    /// # Arguments
    /// * `expression` - A reference to a `LogicalOperator` representing the logical expression to be evaluated. This
    ///   expression forms the premise of a rule and can include nested logical structures for complex condition evaluation.
    ///
    /// # Returns
    /// * `bool` - A boolean value indicating whether the premise is true (`true`) or false (`false`) given the current
    ///   state of the engine's knowledge base and variable bindings.
    ///
    /// # Examples
    /// ```
    /// // Assuming an instance `engine` of `SymbolicReasoningEngine`
    /// // and previously defined symbols and variable bindings
    ///
    /// // Define a logical expression as a premise
    /// //let expression = LogicalOperator::And(vec![
    /// //    LogicalOperator::Fact(Fact::new(weather_symbol, FactValue::Text(String::from("Sunny")))),
    /// //    LogicalOperator::GreaterThan(
    /// //        Fact::new(temperature_symbol, FactValue::Integer(25))
    /// //    )
    /// //]);
    ///
    /// // Evaluate the expression to see if the conditions are met
    /// //let is_true = engine.is_premise_true(&expression);
    /// //assert_eq!(is_true, true); // Assumes the knowledge base supports this evaluation
    /// ```
    ///
    /// The `is_premise_true` method facilitates dynamic and flexible rule evaluation within the engine, supporting
    /// a wide range of logical operations and conditions. Its ability to accurately assess premises enables the engine
    /// to perform sophisticated reasoning and inference, adapting to changes in the knowledge base and variable states.
    fn is_premise_true(&self, expression: &LogicalOperator) -> bool {
        self.print_debug(&format!("Starting evaluation of expression: {:?}", expression));
        let result = match expression {
            LogicalOperator::And(expressions) => {
                // AND: All expressions must be true
                expressions.iter().all(|expr| {
                    let res = self.is_premise_true(expr);
                    self.print_debug(&format!("AND expr: {:?}, result: {}", expr, res));
                    res
                })
            },
            LogicalOperator::Or(expressions) => {
                // OR: At least one expression must be true
                expressions.iter().any(|expr| {
                    let res = self.is_premise_true(expr);
                    self.print_debug(&format!("OR expr: {:?}, result: {}", expr, res));
                    res
                })
            },
            LogicalOperator::Not(expression) => {
                // NOT: The expression must not be true
                let res = !self.is_premise_true(expression);
                self.print_debug(&format!("NOT expr: {:?}, result: {}", expression, res));
                res
            },
            LogicalOperator::AtomicFact(fact) => {
                // Directly evaluate the fact against the known facts
                let res = self.facts.contains(fact);
                self.print_debug(&format!("Fact evaluation: {:?}, result: {}", fact, res));
                res
            },
            LogicalOperator::GreaterThan(left, right) => {
                if self.compare_values(left, right, |a, b| a > b) {
                    true
                } else {
                    false
                }
            },
            LogicalOperator::LessThan(left, right) => {
                if self.compare_values(left, right, |a, b| a < b) {
                    true
                } else {
                    false
                }
            },
            LogicalOperator::EqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a == b) {
                    true
                } else {
                    false
                }
            },
            LogicalOperator::NotEqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a != b) {
                    true
                } else {
                    false
                }
            },
            LogicalOperator::GreaterThanOrEqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a >= b) {
                    true
                } else {
                    false
                }
            },
            LogicalOperator::LessThanOrEqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a <= b) {
                    true
                } else {
                    false
                }
            },
        };
        self.print_debug(&format!("Expression evaluation completed: {:?}, result: {}", expression, result));
        result
    }

    /// Executes the forward chaining inference process over the current set of rules and facts.
    ///
    /// Forward chaining is a key reasoning strategy in the symbolic reasoning engine, where the engine iteratively
    /// applies rules to the known facts in the knowledge base to infer new facts. This method scans through all the
    /// rules in the engine, evaluating their premises against the existing facts and variable bindings. When a premise
    /// is found to be true, the conclusion of the rule is asserted as a new fact, potentially triggering further inferences
    /// as the knowledge base is updated with new information.
    ///
    /// This process continues until no new facts can be inferred from the current set of rules, indicating a stable state
    /// where all applicable rules have been exhausted. Forward chaining is particularly effective in systems where the
    /// knowledge base is incrementally expanded through the application of rules, facilitating dynamic reasoning based on
    /// the evolving state of facts and variables.
    ///
    /// # Usage
    /// The method does not require any arguments and operates directly on the engine's internal state, making it simple
    /// to initiate the inference process. It is typically called after setting up the initial knowledge base and defining
    /// the relevant rules and variables.
    ///
    /// ```
    /// //let mut engine = SymbolicReasoningEngine::new();
    /// // Assuming the engine has been populated with facts, rules, and variables
    /// //engine.forward_chaining();
    /// // After calling forward_chaining, the knowledge base may be expanded with new inferred facts.
    /// ```
    ///
    /// # Note
    /// The effectiveness and efficiency of forward chaining can depend on the complexity and number of rules, the size
    /// of the knowledge base, and the specificity of rule premises. It is important to design rules with clear and
    /// relevant premises to ensure productive and meaningful inferences.
    fn forward_chaining(&mut self) {
        let mut new_facts = Vec::new();

        for rule in &self.rules {
            if self.is_premise_true(&rule.premise) {
                if !self.facts.contains(&rule.conclusion) {
                    new_facts.push(rule.conclusion.clone());
                }
            }
        }

        // Add all new facts to the knowledge base
        self.facts.extend(new_facts);
    }

    /// Executes the forward chaining inference process, considering both static facts and dynamic variables.
    ///
    /// This enhanced forward chaining method extends the basic forward chaining approach by incorporating the evaluation
    /// of variables alongside facts in the inference process. It allows the engine to apply rules not only based on the
    /// current static facts in the knowledge base but also taking into account the values of variables at the time of
    /// evaluation. This dynamic approach enables the engine to handle more complex scenarios where the applicability of
    /// rules may depend on conditions that change over time or are contingent upon variable states.
    ///
    /// The method iterates over all defined rules, evaluating their premises with respect to both the facts and the
    /// current bindings of variables. If a rule's premise evaluates to true, its conclusion is asserted as a new fact,
    /// which may in turn update the state of the knowledge base and influence the evaluation of subsequent rules. This
    /// process is repeated until no new facts can be inferred, ensuring that the knowledge base reflects all derivable
    /// information given the initial facts, rules, and variable states.
    ///
    /// # Usage
    /// Like `forward_chaining`, this method operates on the engine's internal state and does not require arguments. It
    /// should be invoked after initializing the knowledge base, rules, and variable bindings to drive the inference process.
    ///
    /// ```
    /// //let mut engine = SymbolicReasoningEngine::new();
    /// // Populate the engine with initial facts, rules, and variable bindings
    /// //engine.forward_chaining_with_variables();
    /// // The knowledge base may now include new facts inferred considering both static facts and variable states.
    /// ```
    ///
    /// # Note
    /// Utilizing variables within the forward chaining process introduces additional complexity, as the engine must
    /// resolve variable values during rule evaluation. This requires careful management of variable bindings to ensure
    /// accurate and meaningful inferences. Designing rules and premises that effectively leverage variables can
    /// significantly enhance the engine's reasoning capabilities, enabling it to adapt to a wider range of dynamic
    /// conditions and scenarios.
    fn forward_chaining_with_variables(&mut self) {
        let mut new_facts_added = true;

        while new_facts_added {
            new_facts_added = false;
            let mut conclusions_to_add = Vec::new();

            for rule in self.rules.clone() {
                let new_fact = self.apply_rule_conclusion(&rule.conclusion);
                match self.match_rule(&rule.premise) {
                    Some(bindings) => {
                        self.variable_bindings.extend(bindings);
                        conclusions_to_add.push(new_fact);
                    },
                    _ => {}
                };
            }

            for new_fact in conclusions_to_add {
                if !self.facts.contains(&new_fact) {
                    self.facts.push(new_fact);
                    new_facts_added = true;
                }
            }
        }
    }

    // Separated the logic to apply the rule's conclusion into its own method to avoid borrowing issues
    fn apply_rule_conclusion(&self, conclusion: &Fact) -> Fact {
        conclusion.clone()
    }

    /// Evaluates the premise of a rule and determines if it matches the current state of the knowledge base, considering variable bindings.
    ///
    /// This method plays a critical role in the inference process of the symbolic reasoning engine by evaluating the logical
    /// expressions that constitute the premises of rules. It determines whether these premises are satisfied given the current
    /// facts and variable bindings within the engine. If the premise is satisfied, the method returns a mapping of any variables
    /// involved in the premise to their resolved values, facilitating the application of the rule's conclusion with accurate,
    /// context-specific data.
    ///
    /// The method supports complex logical expressions involving AND, OR, NOT operators, and direct fact comparisons, allowing
    /// for the evaluation of sophisticated rule premises. Variable values are resolved against the current bindings to ensure
    /// that dynamic conditions are accurately assessed.
    ///
    /// # Arguments
    /// * `premise` - A reference to a `LogicalOperator` representing the logical expression that forms the premise of a rule.
    ///
    /// # Returns
    /// * `Option<HashMap<String, FactValue>>` - An optional mapping of variable names to their resolved values if the premise
    ///   is satisfied. Returns `None` if the premise is not satisfied, indicating that the rule should not be applied.
    ///
    /// # Examples
    /// ```
    /// // Assuming an instance `engine` of `SymbolicReasoningEngine`
    /// // and a rule premise that involves temperature and weather conditions
    /// //let premise = LogicalOperator::And(vec![
    /// //    LogicalOperator::Fact(Fact::new(temperature_symbol, FactValue::GreaterThan(FactValue::Integer(20)))),
    /// //    LogicalOperator::Fact(Fact::new(weather_symbol, FactValue::Text(String::from("Sunny"))))
    /// //]);
    ///
    /// // Evaluate the rule's premise
    /// //if let Some(bindings) = engine.match_rule(&premise) {
    ///     // Premise is satisfied, proceed with applying the rule's conclusion
    ///     // Bindings contain any variable resolutions involved in the premise
    /// //} else {
    ///     // Premise is not satisfied, rule is not applied
    /// //}
    /// ```
    ///
    /// The `match_rule` method enables the engine to dynamically assess rule premises against the evolving knowledge base,
    /// supporting conditional logic and variable-based reasoning within the rule evaluation framework.
    fn match_rule(&self, premise: &LogicalOperator) -> Option<HashMap<String, FactValue>> {
        self.evaluate_logical_expression(premise, &self.variable_bindings, false, &mut None)
    }

    /// Evaluates a logical expression against the current knowledge base and provided variable bindings.
    ///
    /// This method is a cornerstone of the engine's reasoning capabilities, allowing for the evaluation of complex
    /// logical expressions that may include combinations of AND, OR, NOT operators, direct fact comparisons, and
    /// variable references. It determines the truth of the expression based on the facts currently known to the engine
    /// and the context provided by any existing variable bindings. If the expression evaluates to true, the method
    /// returns an updated set of variable bindings that includes any variables involved in the expression with their
    /// resolved values. This is particularly useful for rules that involve variables, as it allows for the dynamic
    /// application of rule conclusions based on the specific conditions met at the time of evaluation.
    ///
    /// # Arguments
    /// * `expression` - A reference to a `LogicalOperator` representing the logical expression to be evaluated.
    /// * `existing_bindings` - A reference to a `HashMap` containing any existing variable bindings. These bindings
    ///   represent the current state of variables within the engine's context and are used to resolve variable
    ///   references within the expression.
    /// * `use_backward_chaining` - A reference to a bool flagging the use of recursive rule searches in fact evaluation for backward chaining
    /// * `visited` - A mutable reference to an optional vector tracking visited goals to prevent cycles. Backward chaining must provide this argument.
    ///
    /// # Returns
    /// * `Option<HashMap<String, FactValue>>` - An optional mapping of variable names to their resolved values
    ///   if the expression evaluates to true, indicating that the conditions represented by the expression are
    ///   satisfied. Returns `None` if the expression evaluates to false, indicating that the conditions are not met.
    ///
    /// # Examples
    /// ```
    /// // Assuming an instance `engine` of `SymbolicReasoningEngine`
    /// // Define a logical expression involving variables and facts
    /// //let expression = LogicalOperator::And(vec![
    /// //    LogicalOperator::Variable("temperature".to_string(), FactValue::GreaterThan(FactValue::Integer(20))),
    /// //    LogicalOperator::Fact(Fact::new(weather_symbol, FactValue::Text(String::from("Sunny"))))
    /// //]);
    ///
    /// // Evaluate the expression with initial variable bindings
    /// //let initial_bindings = HashMap::new(); // Assume this contains any initial variable states
    /// //if let Some(bindings) = engine.evaluate_logical_expression(&expression, &initial_bindings) {
    ///     // Expression is true, proceed with rule application or further logic
    ///     // Bindings now include resolved variable values based on the expression evaluation
    /// //} else {
    ///     // Expression is false, conditions are not met
    /// //}
    /// ```
    ///
    /// The `evaluate_logical_expression` method enables nuanced and conditional logic to be applied within the engine,
    /// supporting the evaluation of rules and conditions that reflect the complex dynamics of the domain being modeled.
    fn evaluate_logical_expression(&self, expression: &LogicalOperator, existing_bindings: &HashMap<String, FactValue>, use_backward_chaining: bool, visited: &mut Option<&mut Vec<Fact>>) -> Option<HashMap<String, FactValue>> {
        match expression {
            LogicalOperator::And(expressions) => {
                let mut combined_bindings = existing_bindings.clone();
                for expr in expressions {
                    if let Some(bindings) = self.evaluate_logical_expression(expr, &combined_bindings, use_backward_chaining, visited) {
                        combined_bindings = bindings; // Instead of extending, replace to avoid duplication
                    } else {
                        return None; // Short-circuit on the first false expression
                    }
                }
                Some(combined_bindings)
            },
            LogicalOperator::Or(expressions) => expressions.iter()
                .find_map(|expr| self.evaluate_logical_expression(expr, existing_bindings, use_backward_chaining, visited)),
            LogicalOperator::Not(expression) => match self.evaluate_logical_expression(expression, existing_bindings, use_backward_chaining, visited) {
                None => Some(existing_bindings.clone()), // NOT expression is true if inner is false
                Some(_) => None, // NOT expression is false if inner is true
            },
            LogicalOperator::AtomicFact(fact) => {
                if use_backward_chaining {
                    for known_fact in &self.facts {
                        if self.match_fact(fact, known_fact) {
                            return Some(existing_bindings.clone());
                        }
                    }
                    match visited {
                        Some(visited_facts) => {
                            if self.search_for_rules(fact, visited_facts) {
                                return Some(existing_bindings.clone());
                            }
                        },
                        _ => panic!("Backward chaining calls to evaluate_logical_expression must provide visited rules")
                    }
                } else {
                    for known_fact in &self.facts {
                        if self.match_fact(fact, known_fact) {
                            return Some(existing_bindings.clone());
                        }
                    }
                }
                None // Fact does not match any known facts
            },
            LogicalOperator::GreaterThan(left, right) => {
                if self.compare_values(left, right, |a, b| a > b) {
                    Some(existing_bindings.clone())
                } else {
                    None
                }
            },
            LogicalOperator::LessThan(left, right) => {
                if self.compare_values(left, right, |a, b| a < b) {
                    Some(existing_bindings.clone())
                } else {
                    None
                }
            },
            LogicalOperator::EqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a == b) {
                    Some(existing_bindings.clone())
                } else {
                    None
                }
            },
            LogicalOperator::NotEqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a != b) {
                    Some(existing_bindings.clone())
                } else {
                    None
                }
            },
            LogicalOperator::GreaterThanOrEqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a >= b) {
                    Some(existing_bindings.clone())
                } else {
                    None
                }
            },
            LogicalOperator::LessThanOrEqualTo(left, right) => {
                if self.compare_values(left, right, |a, b| a <= b) {
                    Some(existing_bindings.clone())
                } else {
                    None
                }
            },
        }
    }

    /// Compares two `ComparableValue` instances using a specified comparison function.
    ///
    /// This method takes two `ComparableValue` references, `left` and `right`, and a comparison function `comparison`.
    /// It resolves the actual numerical values of both `left` and `right` by calling `resolve_comparable_value` on each.
    /// Then, it applies the `comparison` function to these numerical values.
    ///
    /// The `comparison` function is a higher-order function that takes two `f64` values and returns a `bool` indicating
    /// the result of the comparison (e.g., greater than, less than, equal to, etc.).
    ///
    /// # Arguments
    /// * `left` - A reference to the first `ComparableValue` to compare.
    /// * `right` - A reference to the second `ComparableValue` to compare.
    /// * `comparison` - A function that defines the type of comparison to perform between the two values.
    ///   It must accept two `f64` arguments and return a `bool` indicating the result of the comparison.
    ///
    /// # Returns
    /// * `bool` - The result of applying the `comparison` function to the resolved values of `left` and `right`.
    fn compare_values(
        &self,
        left: &ComparableValue,
        right: &ComparableValue,
        comparison: fn(f64, f64) -> bool
    ) -> bool {
        let left_value = self.resolve_comparable_value(left);
        let right_value = self.resolve_comparable_value(right);

        comparison(left_value, right_value)
    }

    /// Resolves a `ComparableValue` to its numerical representation as an `f64`.
    ///
    /// This method interprets a `ComparableValue` (which can represent either a direct numerical value or a symbol referring to a fact)
    /// and returns its numerical value as an `f64`. This is useful for comparing numerical values within the rule engine, especially when
    /// evaluating conditions that involve numerical comparisons.
    ///
    /// # Arguments
    /// * `value` - A reference to the `ComparableValue` to be resolved.
    ///
    /// # Returns
    /// * `f64` - The numerical representation of the input `ComparableValue`.
    ///
    /// # Panics
    /// This method panics if:
    /// - The `ComparableValue` is of type `Direct` with a non-numeric `FactValue` (neither `Integer` nor `Float`).
    /// - The `ComparableValue` is of type `Symbol`, and the symbol cannot be found in the knowledge base.
    /// - The `ComparableValue` is of type `Symbol`, and the corresponding `FactValue` is not numeric.
    /// - The `ComparableValue` type is not supported by this method.
    ///
    /// This method is crucial for the operation of the rule engine, allowing it to perform numeric comparisons on facts
    /// and fact values, which are essential for making logical inferences based on the rules defined within the engine.
    fn resolve_comparable_value(&self, value: &ComparableValue) -> f64 {
        match value {
            ComparableValue::Direct(fact_value) => match fact_value {
                FactValue::Integer(val) => *val as f64,
                FactValue::Float(val) => *val,
                _ => panic!("Unsupported FactValue type from ComparableValue::Direct for conversion to f64"),
            },
            ComparableValue::Symbol(symbol) => {
                let fact = self.get_fact_from_symbol(symbol.clone())
                    .expect("Symbol not found in knowledge base");
                let fact_value = self.get_fact_value(fact);
                match fact_value {
                    Some(val) => val,
                    _ => panic!("Unsupported FactValue type from ComparableValue::Symbol for conversion to f64"),
                }
            },
            ComparableValue::SymbolName(symbol_name) => {
                let symbol = self.symbols.get(symbol_name).expect("Symbol not found in knowledge base");
                let fact = self.get_fact_from_symbol(symbol.clone())
                    .expect("Symbol not found in knowledge base");
                let fact_value = self.get_fact_value(fact);
                match fact_value {
                    Some(val) => val,
                    _ => panic!("Unsupported FactValue type from ComparableValue::SymbolName for conversion to f64"),
                }
            }
        }
    }

    /// Resolves the value of a specified variable based on the provided bindings.
    ///
    /// This method is crucial for the dynamic evaluation of rules and logical expressions within the symbolic reasoning engine.
    /// It attempts to find the current value of a variable by looking it up in the provided set of variable bindings, which map
    /// variable names to their respective values. This lookup process enables the engine to work with dynamic data, where the
    /// value of a variable can influence the outcome of rule evaluations and logical operations.
    ///
    /// The method supports scenarios where variables are used within rules to represent data that may change over time or
    /// across different contexts. By resolving variables to their current values at the time of evaluation, the engine ensures
    /// that its reasoning processes are based on the most up-to-date information available.
    ///
    /// # Arguments
    /// * `var_name` - A string slice (`&str`) representing the name of the variable whose value is to be resolved.
    /// * `bindings` - A reference to a `HashMap` containing the current variable bindings. This map provides the context
    ///   for resolving the variable's value, associating variable names with their current `FactValue`s.
    ///
    /// # Returns
    /// * `Option<FactValue>` - An optional `FactValue` representing the resolved value of the variable if it exists within
    ///   the provided bindings. Returns `None` if the variable name is not found, indicating that the variable has not been
    ///   bound to a value within the current context.
    ///
    /// # Examples
    /// ```
    /// // Assuming a set of variable bindings
    /// //let mut bindings = HashMap::new();
    /// //bindings.insert("temperature".to_string(), FactValue::Integer(25));
    ///
    /// // Resolve the value of the "temperature" variable
    /// //let temperature_value = engine.resolve_variable_value("temperature", &bindings);
    ///
    /// //assert_eq!(temperature_value, Some(FactValue::Integer(25))); // The variable is successfully resolved
    /// ```
    ///
    /// The `resolve_variable_value` method plays a key role in enabling flexible and dynamic rule evaluation, supporting
    /// the engine's ability to adapt its reasoning based on variable data.
    /// TODO: Add support for variable resolution within facts themselves.
    fn resolve_variable_value(&self, var_name: &str, bindings: &HashMap<String, FactValue>) -> Option<FactValue> {
        bindings.get(var_name).cloned()
    }

    /// Matches a single fact against known facts, considering variable bindings and comparisons.
    ///
    /// # Arguments
    /// * `fact` - A reference to the fact being matched.
    /// * `known_fact` - A reference to a known fact for comparison.
    /// * `bindings` - A mutable reference to a HashMap representing variable bindings.
    ///
    /// # Returns
    /// True if the fact matches the known fact based on direct equality or comparison operations; false otherwise.
    fn match_fact(&self, fact: &Fact, known_fact: &Fact) -> bool {
        // Ensure symbols match before evaluating values.
        if fact.symbol != known_fact.symbol {
            // self.print_debug("Symbols do not match.");
            return false;
        }

        self.print_debug(&format!("Matching fact: {:?} against known fact: {:?}", fact, known_fact));

        match (&fact.value, &known_fact.value) {
            // Direct value comparison
            (FactValue::Integer(l), FactValue::Integer(r)) => l == r,
            (FactValue::Float(l), FactValue::Float(r)) => l == r,
            (FactValue::Boolean(l), FactValue::Boolean(r)) => l == r,
            (FactValue::Text(l), FactValue::Text(r)) => l == r,
            // Default case for non-matching types or unsupported comparisons
            _ => {
                self.print_debug("Fact types do not match or comparison not supported.");
                false
            },
        }
    }

    /// Initiates the backward chaining process to try and satisfy a specified goal.
    ///
    /// # Arguments
    /// * `goal` - The goal the engine attempts to satisfy, represented as a `Fact`.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the engine successfully satisfies the goal using backward chaining,
    ///   or `false` if the goal cannot be satisfied with the current set of rules and facts.
    pub fn specify_goal(&mut self, goal: &Fact) -> bool {
        let mut visited = Vec::new(); // Used to track visited rules for cycle detection
        self.search_for_rules(goal, &mut visited)
    }

    /// Attempts to satisfy a specified goal by recursively searching for and applying rules.
    ///
    /// This method forms the core of the backward chaining logic, searching for rules that have conclusions
    /// matching the goal. For each found rule, it then attempts to satisfy all of the rule's premises through
    /// further backward chaining. Cycle detection is performed to prevent infinite recursion.
    ///
    /// # Arguments
    /// * `goal` - The goal the system is trying to satisfy, represented as a `Fact`.
    /// * `visited` - A mutable reference to a vector tracking visited goals to prevent cycles.
    ///
    /// # Returns
    /// * `bool` - True if the goal can be satisfied through backward chaining, false otherwise.
    fn search_for_rules(&self, goal: &Fact, visited: &mut Vec<Fact>) -> bool {
        // Step 1: Detect cycle
        if self.detect_cycle(goal, visited) {
            println!("Cycle detected for goal: {:?}", goal);
            // Cycle detected, return false to prevent infinite recursion
            return false;
        }

        // Step 2: Check if the goal is already a known fact
        for known_fact in &self.facts {
            let value = known_fact.value.clone();
            if value == goal.value {
                return true;
            }
        }

        visited.push(goal.clone()); // Add the current goal to the visited list

        // Step 2: Search for rules that could lead to the goal
        let applicable_rules = self.rules.iter().filter(|rule| {
            // Check if the rule's conclusion matches the goal
            &rule.conclusion == goal
        });

        // Step 3: Attempt to satisfy the conditions of each applicable rule
        for rule in applicable_rules {
            // Recursively apply backward chaining on the rule's conditions
            if let Some(_) = self.evaluate_logical_expression(&rule.premise, &self.variable_bindings, true, &mut Some(visited)) {
                return true;
            }
        }

        visited.pop(); // Clean up to allow revisiting this goal from different paths

        // If no rules lead to satisfying the goal, return false
        false
    }

    /// Detects cycles within the rule evaluation process to prevent infinite recursion.
    ///
    /// This method checks if the current goal or premise has already been visited during
    /// the backward chaining process. If a goal or premise is encountered more than once,
    /// it indicates a potential cycle in the rule dependencies, which could lead to infinite
    /// recursion if not handled properly.
    ///
    /// # Arguments
    /// * `current` - The current goal or premise being evaluated, represented as a `Fact`.
    /// * `visited` - A reference to a vector containing all previously visited goals and premises
    ///   in the current chain of reasoning. This vector tracks the path taken during the backward
    ///   chaining process and is used to identify cycles.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if a cycle is detected (i.e., the `current` goal or premise has
    ///   already been visited), indicating that further evaluation should be halted to prevent
    ///   infinite recursion. Returns `false` if no cycle is detected, allowing the evaluation to
    ///   proceed.
    ///
    /// # Examples
    /// ```
    /// // Assuming an instance `engine` of `SymbolicReasoningEngine`
    /// // and a setup where "goal_fact" might cause a cyclic rule dependency
    /// //let mut visited = Vec::new();
    /// //let goal_fact = Fact::new(Symbol::new("example_symbol", "Integer"), FactValue::Integer(1));
    ///
    /// // Initially, no cycle is detected
    /// //assert_eq!(engine.detect_cycle(&goal_fact, &visited), false);
    ///
    /// // Add the goal_fact to the visited list
    /// //visited.push(goal_fact.clone());
    ///
    /// // Now, detecting the cycle should return true
    /// //assert_eq!(engine.detect_cycle(&goal_fact, &visited), true);
    /// ```
    fn detect_cycle(&self, current: &Fact, visited: &[Fact]) -> bool {
        visited.contains(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fact_value_types() {
        let symbol = Symbol { name: "Temperature".to_string(), symbol_type: "Integer".to_string() };

        // Integer Value
        let int_fact = Fact { symbol: symbol.clone(), value: FactValue::Integer(30) };
        assert_eq!(int_fact.value, FactValue::Integer(30));

        // Float Value
        let float_fact = Fact { symbol: symbol.clone(), value: FactValue::Float(25.5) };
        assert_eq!(float_fact.value, FactValue::Float(25.5));

        // Boolean Value
        let bool_fact = Fact { symbol: symbol.clone(), value: FactValue::Boolean(true) };
        assert_eq!(bool_fact.value, FactValue::Boolean(true));

        // Text Value
        let text_fact = Fact { symbol, value: FactValue::Text("Warm".to_string()) };
        assert_eq!(text_fact.value, FactValue::Text("Warm".to_string()));
    }

    #[test]
    fn apply_simple_rule_without_variables() {
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
    }

    // TODO: Refactor variable handling.
    // #[test]
    // fn apply_rule_with_variables() {
    //     let mut engine = SymbolicReasoningEngine::new();
    //
    //     // Define symbols
    //     let location_symbol = engine.define_symbol("Location", "String");
    //     let temp_symbol = engine.define_symbol("Temperature", "Integer");
    //     let condition_symbol = engine.define_symbol("Condition", "String");
    //
    //     // Assert the fact: Location 'Desert' has a temperature of 30 degrees
    //     engine.assert_fact(location_symbol.clone(), FactValue::Text("Desert".to_string()));
    //     engine.assert_fact(temp_symbol.clone(), FactValue::Integer(30));
    //
    //     // Define the rule: If a location's temperature is above 25, it's considered hot
    //     engine.define_rule(
    //         LogicalOperator::GreaterThan(
    //             Box::new(ComparableValue::Symbol(temp_symbol.clone())),
    //             Box::new(ComparableValue::Direct(FactValue::Integer(20)))
    //         ),
    //         Fact {
    //             symbol: condition_symbol.clone(),
    //             value: FactValue::Text("Hot".to_string()),
    //         }
    //     );
    //
    //     // Simulate matching a variable within the rule's premise to the known facts
    //     // and applying the conclusion based on this match
    //     engine.forward_chaining_with_variables();
    //
    //     // Check if the new fact (the location is hot) is added to the knowledge base
    //     assert!(engine.facts.iter().any(|fact|
    //         fact.symbol == condition_symbol && fact.value == FactValue::Text("Hot".to_string())
    //     ), "The engine did not correctly apply the rule with variables to infer the location is hot.");
    // }

    #[test]
    fn no_new_facts_generated() {
        let mut engine = SymbolicReasoningEngine::new();

        // Define symbols
        let weather_symbol = engine.define_symbol("Weather", "String");
        let recommendation_symbol = engine.define_symbol("Recommendation", "String");

        // Assert known facts: It is raining, and an umbrella is recommended
        engine.assert_fact(weather_symbol.clone(), FactValue::Text("Rainy".to_string()));
        engine.assert_fact(recommendation_symbol.clone(), FactValue::Text("Umbrella".to_string()));

        // Define the rule: If it is raining, then an umbrella is recommended
        engine.define_rule(
            LogicalOperator::AtomicFact(Fact {
                symbol: weather_symbol,
                value: FactValue::Text("Rainy".to_string()),
            }),
            Fact {
                symbol: recommendation_symbol,
                value: FactValue::Text("Umbrella".to_string()),
            }
        );

        // Perform forward chaining to infer new facts based on the rules
        let initial_fact_count = engine.facts.len();
        engine.forward_chaining();

        // Check if no new facts were added to the knowledge base
        let final_fact_count = engine.facts.len();
        assert_eq!(initial_fact_count, final_fact_count, "No new facts should have been added, but the fact count changed.");
    }

    #[test]
    fn handle_nested_logical_expressions() {
        let mut engine = SymbolicReasoningEngine::new();
        engine.enable_debug();

        // Define symbols
        let weather_symbol = engine.define_symbol("Weather", "String");
        let temp_symbol = engine.define_symbol("Temperature", "Integer");
        let activity_symbol = engine.define_symbol("Activity", "String");

        // Assert facts: It is not raining, and the temperature is 25 degrees
        engine.assert_fact(weather_symbol.clone(), FactValue::Text("NotRaining".to_string()));
        engine.assert_fact(temp_symbol.clone(), FactValue::Integer(25));

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
                LogicalOperator::GreaterThan(
                    Box::new(ComparableValue::Symbol(temp_symbol.clone())),
                    Box::new(ComparableValue::Direct(FactValue::Integer(20)))
                )
            ]),
            Fact {
                symbol: activity_symbol.clone(),
                value: FactValue::Text("GoodForOutdoor".to_string()),
            }
        );

        // Perform forward chaining to infer new facts based on the rules
        engine.forward_chaining_with_variables();

        // println!("Defined symbols: {:?}", engine.symbols);
        // println!("Asserted facts: {:?}", engine.facts);
        // println!("Asserted variables: {:?}", engine.variable_bindings);

        // Check if the new fact (good day for outdoor activities) is added to the knowledge base
        assert!(engine.facts.iter().any(|fact|
            fact.symbol == activity_symbol && fact.value == FactValue::Text("GoodForOutdoor".to_string())
        ), "The engine did not infer that it's a good day for outdoor activities based on the weather conditions and temperature.");
    }

    #[test]
    #[should_panic(expected = "Symbol already defined")]
    fn test_adding_duplicate_symbol_panics() {
        let mut engine = SymbolicReasoningEngine::new();

        // Define a symbol "temperature" of type "Integer"
        let symbol_name = "temperature";
        let symbol_type = "Integer";
        engine.define_symbol(symbol_name, symbol_type);

        // Attempt to define the same symbol "temperature" of type "Integer" again
        // This should panic with the specified message "Symbol already defined"
        engine.define_symbol(symbol_name, symbol_type);
    }

    #[test]
    fn test_goal_specification() {
        let mut engine = SymbolicReasoningEngine::new();

        let temp_symbol = engine.define_symbol("Temperature", "String");
        let weather_symbol = engine.define_symbol("Weather", "String");
        let hiking_symbol = engine.define_symbol("Hiking", "String");

        // Assert facts
        engine.assert_fact(weather_symbol.clone(), FactValue::Text("Sunny".into()));
        engine.assert_fact(temp_symbol.clone(), FactValue::Text("Moderate".into()));

        // Define rule for hiking suitability
        engine.define_rule(
            LogicalOperator::And(vec![
                LogicalOperator::AtomicFact(Fact::new(weather_symbol.clone(), FactValue::Text("Sunny".into()))),
                LogicalOperator::AtomicFact(Fact::new(temp_symbol.clone(), FactValue::Text("Moderate".into()))),
            ]),
            Fact::new(hiking_symbol.clone(), FactValue::Boolean(true))
        );

        // Specify the goal
        let goal = Fact::new(hiking_symbol.clone(), FactValue::Boolean(true));

        // Test if the engine can determine the goal
        let result = engine.specify_goal(&goal);

        // Assert that the engine successfully finds the solution to the specified goal
        assert_eq!(result, true, "The engine should successfully determine that hiking is suitable.");
    }

    #[test]
    fn test_recursive_rule_search() {
        let mut engine = SymbolicReasoningEngine::new();
        engine.enable_debug();

        // Define symbols for the test
        let weather = engine.define_symbol("Weather", "String");
        let temperature = engine.define_symbol("Temperature", "Integer");

        // Assert known facts into the engine's knowledge base
        engine.assert_fact(temperature.clone(), FactValue::Integer(25));
        engine.assert_fact(weather.clone(), FactValue::Text(String::from("Sunny")));

        // Define rules that require recursion to satisfy the goal
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
    }

    #[test]
    fn test_cycle_detection() {
        let mut engine = SymbolicReasoningEngine::new();

        let a_symbol = engine.define_symbol("A", "Boolean");
        let b_symbol = engine.define_symbol("B", "Boolean");
        let c_symbol = engine.define_symbol("C", "Boolean");

        engine.define_rule(
            LogicalOperator::AtomicFact(Fact::new(b_symbol.clone(), FactValue::Boolean(true))),
            Fact::new(a_symbol.clone(), FactValue::Boolean(true))
        );
        engine.define_rule(
            LogicalOperator::AtomicFact(Fact::new(c_symbol.clone(), FactValue::Boolean(true))),
            Fact::new(b_symbol.clone(), FactValue::Boolean(true))
        );
        engine.define_rule(
            LogicalOperator::AtomicFact(Fact::new(a_symbol.clone(), FactValue::Boolean(true))),
            Fact::new(c_symbol.clone(), FactValue::Boolean(true))
        );

        // Define a goal that would trigger the cycle
        let goal = Fact::new(a_symbol.clone(), FactValue::Boolean(true));

        // Attempting to satisfy this goal should not cause an infinite loop and should return false,
        // indicating that the cycle was detected and the engine continued operation.
        let result = engine.specify_goal(&goal);

        assert_eq!(result, false, "The engine should detect the cycle and not satisfy the goal.");
    }

    #[test]
    #[should_panic(expected = "Unsupported FactValue type from ComparableValue::Direct for conversion to f64")]
    fn test_direct_fact_value_panic() {
        let mut engine = SymbolicReasoningEngine::new();

        // Define symbols
        let a_symbol = engine.define_symbol("A", "Integer");
        let b_symbol = engine.define_symbol("B", "String");

        // Assert the fact: A = 0
        engine.assert_fact(a_symbol.clone(), FactValue::Integer(0));

        // Define the [invalid] rule: If Integer is greater than Text
        engine.define_rule(
            LogicalOperator::GreaterThan(
                Box::new(ComparableValue::Symbol(a_symbol.clone())),
                Box::new(ComparableValue::Direct(FactValue::Text("Invalid".to_string())))
            ),
            Fact {
                symbol: b_symbol.clone(),
                value: FactValue::Text("C".to_string()),
            },
        );

        // Perform forward chaining to infer new facts based on the rules.
        // This should invoke a panic based on our invalid rules.
        engine.forward_chaining();
    }

    #[test]
    #[should_panic(expected = "Symbol not found in knowledge base")]
    fn test_symbol_not_found_panic() {
        let mut engine = SymbolicReasoningEngine::new();

        // Define symbols
        let a_symbol = engine.define_symbol("A", "Integer");
        let b_symbol = engine.define_symbol("B", "String");

        // Assert the fact: A = 0
        engine.assert_fact(a_symbol.clone(), FactValue::Integer(0));

        // Define the [invalid] rule: If non-existent symbol is greater than 0
        engine.define_rule(
            LogicalOperator::GreaterThan(
                Box::new(ComparableValue::Symbol(b_symbol.clone())),
                Box::new(ComparableValue::Direct(FactValue::Integer(0)))
            ),
            Fact {
                symbol: b_symbol.clone(),
                value: FactValue::Text("C".to_string()),
            },
        );

        // Perform forward chaining to infer new facts based on the rules.
        // This should invoke a panic based on our invalid rules.
        engine.forward_chaining();
    }

    #[test]
    #[should_panic(expected = "Symbol not found in knowledge base")]
    fn test_symbol_name_not_found_panic() {
        let mut engine = SymbolicReasoningEngine::new();

        // Define symbols
        let a_symbol = engine.define_symbol("A", "Integer");
        let b_symbol = engine.define_symbol("B", "String");

        // Assert the fact: A = 0
        engine.assert_fact(a_symbol.clone(), FactValue::Integer(0));

        // Define the [invalid] rule: If non-existent symbol name is greater than 0
        engine.define_rule(
            LogicalOperator::GreaterThan(
                Box::new(ComparableValue::SymbolName("InvalidSymbol".to_string())),
                Box::new(ComparableValue::Direct(FactValue::Integer(0)))
            ),
            Fact {
                symbol: b_symbol.clone(),
                value: FactValue::Text("C".to_string()),
            },
        );

        // Perform forward chaining to infer new facts based on the rules.
        // This should invoke a panic based on our invalid rules.
        engine.forward_chaining();
    }
}
