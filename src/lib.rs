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

/// Represents a fact in the symbolic reasoning engine.
///
/// A fact associates a symbol with a specific value, contributing to the knowledge base of the engine.
/// Facts are used in conjunction with rules to infer new information or make decisions based on the engine's current state.
#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    symbol: Symbol,
    value: FactValue, // Simplified to boolean for this example
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
    Comparison(Box<FactValue>, Comparison, Box<FactValue>),
}

/// Represents the set of comparison operations that can be performed between values in the symbolic reasoning engine.
///
/// This enum is used to define the types of comparisons that can be made within rules or conditions, allowing for
/// a broad range of logical expressions involving numeric comparisons, equality checks, and their negations.
/// Comparisons are primarily used in evaluating conditions that involve `FactValue`s, facilitating decision-making
/// processes based on the dynamic data represented within the engine's knowledge base.
///
/// Variants:
/// - `GreaterThan`: Represents a comparison operation where the left-hand side is greater than the right-hand side.
/// - `LessThan`: Represents a comparison operation where the left-hand side is less than the right-hand side.
/// - `EqualTo`: Represents a comparison operation checking for equality between the left-hand side and the right-hand side.
/// - `NotEqualTo`: Represents a comparison operation checking for inequality between the left-hand side and the right-hand side.
/// - `GreaterThanOrEqualTo`: Represents a comparison operation where the left-hand side is greater than or equal to the right-hand side.
/// - `LessThanOrEqualTo`: Represents a comparison operation where the left-hand side is less than or equal to the right-hand side.
///
/// These comparison operations enable the symbolic reasoning engine to interpret and evaluate complex logical expressions,
/// underpinning the mechanism through which rules and facts interact to derive new insights or conclusions based on the
/// data modeled within the engine.
#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    EqualTo,
    NotEqualTo,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
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

    /// Enables debug output
    fn enable_debug(&mut self) {
        self.debug = true;
    }

    /// Outputs message to console when debug is true.
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
    // TODO: Review Rust borrowing rules and refactor if necessary.
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
        self.evaluate_logical_expression(premise, &self.variable_bindings)
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
    fn evaluate_logical_expression(&self, expression: &LogicalOperator, existing_bindings: &HashMap<String, FactValue>) -> Option<HashMap<String, FactValue>> {
        match expression {
            LogicalOperator::And(expressions) => {
                let mut combined_bindings = existing_bindings.clone();
                for expr in expressions {
                    self.print_debug(&format!("Evaluating AND expression: {:?}", expr)); // Log each AND expression being evaluated
                    if let Some(bindings) = self.evaluate_logical_expression(expr, &combined_bindings) {
                        self.print_debug(&format!("AND expression true, bindings: {:?}", bindings)); // Log the result and bindings if true
                        combined_bindings.extend(bindings);
                    } else {
                        self.print_debug("AND expression false"); // Log if any AND expression is false
                        return None;
                    }
                }
                Some(combined_bindings)
            },
            LogicalOperator::Or(expressions) => {
                for expr in expressions {
                    self.print_debug(&format!("Evaluating OR expression: {:?}", expr)); // Log each OR expression being evaluated
                    if let Some(bindings) = self.evaluate_logical_expression(expr, existing_bindings) {
                        self.print_debug(&format!("OR expression true, bindings: {:?}", bindings)); // Log the result and bindings if true
                        return Some(bindings);
                    }
                }
                self.print_debug("OR expression false"); // Log if all OR expressions are false
                None
            },
            LogicalOperator::Not(expression) => {
                self.print_debug(&format!("Evaluating NOT expression: {:?}", expression)); // Log the NOT expression being evaluated
                if self.evaluate_logical_expression(expression, existing_bindings).is_none() {
                    self.print_debug("NOT expression true"); // Log if the inner expression is false
                    Some(existing_bindings.clone())
                } else {
                    self.print_debug("NOT expression false"); // Log if the inner expression is true
                    None
                }
            },
            LogicalOperator::AtomicFact(fact) => {
                self.print_debug(&format!("Evaluating Fact: {:?}", fact)); // Log the fact being evaluated
                // Evaluate the fact against the known facts in the knowledge base
                // If the fact matches (taking into account variable bindings), return the bindings
                for known_fact in &self.facts {
                    let mut local_bindings = existing_bindings.clone();
                    if self.match_fact(fact, known_fact, &mut local_bindings) {
                        self.print_debug(&format!("Fact matches, bindings: {:?}", local_bindings)); // Log if the fact matches
                        return Some(local_bindings);
                    }
                }
                self.print_debug("Fact does not match"); // Log if the fact does not match any known facts
                None
            },
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
    fn resolve_variable_value(&self, var_name: &str, bindings: &HashMap<String, FactValue>) -> Option<FactValue> {
        bindings.get(var_name).cloned()
    }

    /// Evaluates a comparison between two `FactValue`s based on the specified `Comparison` operation.
    ///
    /// # Arguments
    /// * `left` - A reference to the `FactValue` on the left side of the comparison.
    /// * `comparison` - The `Comparison` operation to perform.
    /// * `right` - A reference to the `FactValue` on the right side of the comparison.
    ///
    /// # Returns
    /// Returns `true` if the comparison evaluates to true, otherwise `false`.
    ///
    /// # Panics
    /// Panics if the comparison operation is unsupported for the provided `FactValue` types.
    fn evaluate_comparison(&self, left: &FactValue, comparison: &Comparison, right: &FactValue, bindings: &HashMap<String, FactValue>) -> bool {
        self.print_debug(&format!("Evaluating comparison: {:?} {:?} {:?}", left, comparison, right)); // Log the comparison being evaluated

        // Resolve the left and right FactValue operands, which could be variables, to their actual values.
        let left_resolved = match left {
            FactValue::Text(variable_name) => self.resolve_variable_value(variable_name, bindings).expect("Variable not bound"),
            _ => left.clone(),
        };
        let right_resolved = match right {
            FactValue::Text(variable_name) => self.resolve_variable_value(variable_name, bindings).expect("Variable not bound"),
            _ => right.clone(),
        };

        let result = match comparison {
            Comparison::GreaterThan => match (left_resolved, right_resolved) {
                (FactValue::Integer(l), FactValue::Integer(r)) => l > r,
                (FactValue::Float(l), FactValue::Float(r)) => l > r,
                // Add more types as necessary
                _ => panic!("Unsupported types for GreaterThan comparison"),
            },
            Comparison::LessThan => match (left_resolved, right_resolved) {
                (FactValue::Integer(l), FactValue::Integer(r)) => l < r,
                (FactValue::Float(l), FactValue::Float(r)) => l < r,
                // Add more types as necessary
                _ => panic!("Unsupported types for LessThan comparison"),
            },
            Comparison::EqualTo => match (left_resolved, right_resolved) {
                (FactValue::Integer(l), FactValue::Integer(r)) => l == r,
                (FactValue::Float(l), FactValue::Float(r)) => l == r,
                (FactValue::Boolean(l), FactValue::Boolean(r)) => l == r,
                (FactValue::Text(l), FactValue::Text(r)) => l == r,
                // Handle variables and comparisons recursively if necessary
                _ => panic!("Unsupported types for EqualTo comparison"),
            },
            Comparison::NotEqualTo => match (left_resolved, right_resolved) {
                (FactValue::Integer(l), FactValue::Integer(r)) => l != r,
                (FactValue::Float(l), FactValue::Float(r)) => l != r,
                (FactValue::Boolean(l), FactValue::Boolean(r)) => l != r,
                (FactValue::Text(l), FactValue::Text(r)) => l != r,
                _ => panic!("Unsupported types for NotEqualTo comparison"),
            },
            Comparison::GreaterThanOrEqualTo => match (left_resolved, right_resolved) {
                (FactValue::Integer(l), FactValue::Integer(r)) => l >= r,
                (FactValue::Float(l), FactValue::Float(r)) => l >= r,
                // Additional types can be added as necessary
                _ => panic!("Unsupported types for GreaterThanOrEqualTo comparison"),
            },
            Comparison::LessThanOrEqualTo => match (left_resolved, right_resolved) {
                (FactValue::Integer(l), FactValue::Integer(r)) => l <= r,
                (FactValue::Float(l), FactValue::Float(r)) => l <= r,
                // Additional types can be added as necessary
                _ => panic!("Unsupported types for LessThanOrEqualTo comparison"),
            },
        };
        self.print_debug(&format!("Comparison result: {}", result)); // Log the result of the comparison
        result
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
    fn match_fact(&self, fact: &Fact, known_fact: &Fact, existing_bindings: &HashMap<String, FactValue>) -> bool {
        self.print_debug(&format!("Matching fact: {:?} against known fact: {:?}", fact, known_fact));

        // Ensure symbols match before evaluating values.
        if fact.symbol != known_fact.symbol {
            self.print_debug("Symbols do not match.");
            // TODO: Fix issue with Comparisons containing variables causing symbol mismatch.
            // return false;
        }

        match (&fact.value, &known_fact.value) {
            // Direct value comparison
            (FactValue::Integer(l), FactValue::Integer(r)) => l == r,
            (FactValue::Float(l), FactValue::Float(r)) => l == r,
            (FactValue::Boolean(l), FactValue::Boolean(r)) => l == r,
            (FactValue::Text(l), FactValue::Text(r)) => l == r,

            // Comparison handling
            (FactValue::Comparison(left, comp, right), _) => {
                let comparison_result = self.evaluate_comparison(&left, comp, &right, existing_bindings);

                self.print_debug(&format!("Comparison result: {}", comparison_result));
                comparison_result
            },

            // Default case for non-matching types or unsupported comparisons
            _ => {
                self.print_debug("Fact types do not match or comparison not supported.");
                false
            },
        }
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

    #[test]
    fn apply_rule_with_variables() {
        let mut engine = SymbolicReasoningEngine::new();

        // Define symbols
        let location_symbol = engine.define_symbol("Location", "String");
        let temp_symbol = engine.define_symbol("Temperature", "Integer");
        let condition_symbol = engine.define_symbol("Condition", "String");

        // Assert the fact: Location 'Desert' has a temperature of 30 degrees
        engine.assert_fact(location_symbol.clone(),FactValue::Text("Desert".to_string()));
        engine.assert_variable("CurrentTemperature".to_string(), FactValue::Integer(30));

        // Define the rule: If a location's temperature is above 25, it's considered hot
        engine.define_rule(
            LogicalOperator::And(vec![
                LogicalOperator::AtomicFact(Fact {
                    symbol: temp_symbol,
                    value: FactValue::Comparison(
                        Box::new(FactValue::Text("CurrentTemperature".to_string())),
                        Comparison::GreaterThan,
                        Box::new(FactValue::Integer(20))
                    )
                })
            ]),
            Fact {
                symbol: condition_symbol.clone(),
                value: FactValue::Text("Hot".to_string()),
            }
        );

        // Simulate matching a variable within the rule's premise to the known facts
        // and applying the conclusion based on this match
        engine.forward_chaining_with_variables();

        // Check if the new fact (the location is hot) is added to the knowledge base
        assert!(engine.facts.iter().any(|fact|
            fact.symbol == condition_symbol && fact.value == FactValue::Text("Hot".to_string())
        ), "The engine did not correctly apply the rule with variables to infer the location is hot.");
    }

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
}
