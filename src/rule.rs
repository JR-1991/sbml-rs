//! This module provides a safe Rust interface to the libSBML RateRule class.
//!
//! The RateRule class represents a rate rule in an SBML model.
//! It can represent a rate rule for a species or a parameter.
//!
//! This wrapper provides safe access to the underlying C++ libSBML RateRule class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx::let_cxx_string;

use crate::{
    inner,
    model::Model,
    pin_ptr,
    sbmlcxx::{self},
    sbo_term,
    traits::fromptr::FromPtr,
    upcast_annotation, upcast_pin,
};

/// Enum representing the type of a rule
pub enum RuleType {
    /// A rate rule
    RateRule,
    /// An assignment rule
    AssignmentRule,
}

/// A safe wrapper around the libSBML Species class.
///
/// This struct maintains a reference to the underlying C++ Species object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Rule<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::Rule>>,
}

// Set the inner trait for the Rule struct
inner!(sbmlcxx::Rule, Rule<'a>);

// Set the annotation trait for the Rule struct
upcast_annotation!(Rule<'a>, sbmlcxx::Rule, sbmlcxx::SBase);

impl<'a> Rule<'a> {
    /// Creates a new RateRule instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this rate rule
    /// * `id` - The identifier for this rate rule
    ///
    /// # Returns
    /// A new Rule instance
    pub fn new_rate_rule(model: &Model<'a>, variable: &str, formula: &str) -> Self {
        let rate_rule_ptr = model.inner().borrow_mut().as_mut().createRateRule();
        let mut rate_rule = pin_ptr!(rate_rule_ptr, sbmlcxx::RateRule);
        let mut rule = upcast_pin!(rate_rule, sbmlcxx::RateRule, sbmlcxx::Rule);

        // Set the id of the rate rule
        let_cxx_string!(variable = variable);
        rule.as_mut().setVariable(&variable);

        // Set the formula of the rate rule
        let_cxx_string!(formula = formula);
        rule.as_mut().setFormula(&formula);

        Self {
            inner: RefCell::new(rule),
        }
    }

    /// Creates a new AssignmentRule instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this assignment rule
    /// * `variable` - The identifier for this assignment rule
    /// * `formula` - The formula defining the value to assign to the variable
    ///
    /// # Returns
    /// A new Rule instance
    pub fn new_assignment_rule(model: &Model<'a>, variable: &str, formula: &str) -> Self {
        let assignment_rule_ptr = model.inner().borrow_mut().as_mut().createAssignmentRule();
        let mut assignment_rule = pin_ptr!(assignment_rule_ptr, sbmlcxx::AssignmentRule);
        let mut rule = upcast_pin!(assignment_rule, sbmlcxx::AssignmentRule, sbmlcxx::Rule);

        let_cxx_string!(variable = variable);
        rule.as_mut().setVariable(&variable);

        let_cxx_string!(formula = formula);
        rule.as_mut().setFormula(&formula);

        Self {
            inner: RefCell::new(rule),
        }
    }

    /// Returns a reference to the inner RefCell containing the RateRule pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    #[allow(dead_code)]
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::Rule>> {
        &self.inner
    }

    /// Returns the variable of the rate rule.
    ///
    /// # Returns
    /// The variable of the rate rule as a String
    pub fn variable(&self) -> String {
        self.inner
            .borrow()
            .getVariable()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Sets the variable of the rate rule.
    ///
    /// # Arguments
    /// * `variable` - The variable to set
    pub fn set_variable(&self, variable: &str) {
        let_cxx_string!(variable = variable);
        self.inner.borrow_mut().as_mut().setVariable(&variable);
    }

    /// Returns the formula of the rate rule.
    ///
    /// # Returns
    /// The formula of the rate rule as a String
    pub fn formula(&self) -> String {
        self.inner
            .borrow()
            .getFormula()
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Sets the formula of the rate rule.
    ///
    /// # Arguments
    /// * `formula` - The formula to set
    pub fn set_formula(&self, formula: &str) {
        let_cxx_string!(formula = formula);
        self.inner.borrow_mut().as_mut().setFormula(&formula);
    }

    /// Returns the type of the rule.
    ///
    /// # Returns
    /// The type of the rule as a RuleType
    pub fn rule_type(&self) -> Result<RuleType, Box<dyn Error>> {
        let rule = self.inner.borrow();
        if rule.isRate() {
            Ok(RuleType::RateRule)
        } else if rule.isAssignment() {
            Ok(RuleType::AssignmentRule)
        } else {
            Err("Unknown rule type".into())
        }
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Rule, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::Rule> for Rule<'_> {
    /// Creates a new RateRule instance from a unique pointer to a libSBML RateRule.
    ///
    /// This method is primarily used internally by the Model class to create
    /// RateRule instances from libSBML RateRule pointers.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML RateRule
    fn from_ptr(ptr: *mut sbmlcxx::Rule) -> Self {
        let rule = pin_ptr!(ptr, sbmlcxx::Rule);
        Self {
            inner: RefCell::new(rule),
        }
    }
}

/// A builder for creating RateRule instances.
///
/// This struct provides a fluent interface for configuring and building RateRule instances.
/// It allows for setting the variable, formula, and annotation of a rate rule.
///
pub struct RateRuleBuilder<'a> {
    rate_rule: Rc<Rule<'a>>,
}

impl<'a> RateRuleBuilder<'a> {
    /// Creates a new RateRuleBuilder instance.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this rate rule
    /// * `variable` - The variable that this rate rule affects
    /// * `formula` - The mathematical formula defining the rate of change
    ///
    /// # Returns
    /// A new RateRuleBuilder instance
    pub fn new(model: &Model<'a>, variable: &str, formula: &str) -> Self {
        let rate_rule = model.create_rate_rule(variable, formula);
        Self { rate_rule }
    }

    /// Sets the annotation string for this rate rule.
    ///
    /// # Arguments
    /// * `annotation` - The annotation string to set
    ///
    /// # Returns
    /// Result containing the builder for chaining or an error
    pub fn annotation(self, annotation: &str) -> Result<Self, SeError> {
        self.rate_rule
            .set_annotation(annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Sets the annotation using a serializable type.
    ///
    /// # Arguments
    /// * `annotation` - The annotation data to serialize and set
    ///
    /// # Returns
    /// Result containing the builder for chaining or a serialization error
    pub fn annotation_serde<T: Serialize>(self, annotation: &T) -> Result<Self, SeError> {
        self.rate_rule.set_annotation_serde(annotation)?;
        Ok(self)
    }

    /// Builds and returns the configured RateRule.
    ///
    /// # Returns
    /// The constructed RateRule instance wrapped in an Rc
    pub fn build(self) -> Rc<Rule<'a>> {
        self.rate_rule
    }
}

/// A builder for creating AssignmentRule instances.
///
/// This struct provides a fluent interface for configuring and building AssignmentRule instances.
/// It allows for setting the variable, formula, and annotation of an assignment rule.
///
pub struct AssignmentRuleBuilder<'a> {
    assignment_rule: Rc<Rule<'a>>,
}

/// Builder pattern implementation for AssignmentRule.
///
/// This struct provides a fluent interface for constructing AssignmentRule instances
/// with optional configuration. It follows the builder pattern to allow step-by-step
/// construction of an AssignmentRule with method chaining.
impl<'a> AssignmentRuleBuilder<'a> {
    /// Creates a new AssignmentRuleBuilder instance.
    ///
    /// This is the entry point for constructing an AssignmentRule using the builder pattern.
    /// It initializes a new AssignmentRule with the required base parameters and wraps it
    /// in the builder struct.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this assignment rule
    /// * `variable` - The variable that this assignment rule affects. This is the identifier
    ///               of the element (species, parameter etc.) whose value is being set
    /// * `formula` - The mathematical formula defining the value to assign to the variable
    ///
    /// # Returns
    /// A new AssignmentRuleBuilder instance configured with the provided parameters
    pub fn new(model: &Model<'a>, variable: &str, formula: &str) -> Self {
        let assignment_rule = model.create_assignment_rule(variable, formula);
        Self { assignment_rule }
    }

    /// Sets a string annotation for the assignment rule.
    ///
    /// This method allows adding XML-formatted annotation data to provide additional
    /// information about the assignment rule. The annotation should be a valid XML string.
    ///
    /// # Arguments
    /// * `annotation` - The XML annotation string to set
    ///
    /// # Returns
    /// Result containing the builder for method chaining if successful, or a serialization
    /// error if the annotation could not be set
    pub fn annotation(self, annotation: &str) -> Result<Self, SeError> {
        self.assignment_rule
            .set_annotation(annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Sets a structured annotation for the assignment rule using a serializable type.
    ///
    /// This method provides a type-safe way to set annotations by accepting any type that
    /// implements the Serialize trait. The provided data structure will be automatically
    /// serialized to XML format.
    ///
    /// # Type Parameters
    /// * `T` - The type of the annotation data, which must implement Serialize
    ///
    /// # Arguments
    /// * `annotation` - The annotation data to serialize and set
    ///
    /// # Returns
    /// Result containing the builder for method chaining if successful, or a serialization
    /// error if the annotation could not be serialized or set
    pub fn annotation_serde<T: Serialize>(self, annotation: &T) -> Result<Self, SeError> {
        self.assignment_rule.set_annotation_serde(annotation)?;
        Ok(self)
    }

    /// Finalizes the builder and returns the constructed AssignmentRule.
    ///
    /// This method consumes the builder and returns the fully configured AssignmentRule
    /// wrapped in an Rc (reference counted smart pointer) for shared ownership.
    ///
    /// # Returns
    /// The constructed AssignmentRule instance wrapped in an Rc
    pub fn build(self) -> Rc<Rule<'a>> {
        self.assignment_rule
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::Model, SBMLDocument};

    #[test]
    fn test_rate_rule_new() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let rate_rule = Rule::new_rate_rule(&model, "s1", "s1 + s2");
        assert_eq!(rate_rule.variable(), "s1");
        assert_eq!(rate_rule.formula(), "s1 + s2");
    }

    #[test]
    fn test_rate_rule_builder() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let rate_rule = RateRuleBuilder::new(&model, "s1", "s1 + s2")
            .annotation("<test>test</test>")
            .expect("Failed to set annotation")
            .build();
        assert_eq!(rate_rule.variable(), "s1");
        assert_eq!(rate_rule.formula(), "s1 + s2");
        assert_eq!(
            rate_rule
                .get_annotation()
                .replace("\n", "")
                .replace(' ', ""),
            "<annotation><test>test</test></annotation>"
        );
    }

    #[test]
    fn test_rate_rule_builder_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let rate_rule = RateRuleBuilder::new(&model, "s1", "s1 + s2")
            .annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .expect("Failed to set annotation")
            .build();

        assert_eq!(rate_rule.variable(), "s1");
        assert_eq!(rate_rule.formula(), "s1 + s2");
        let annotation = rate_rule.get_annotation_serde::<TestAnnotation>().unwrap();
        assert_eq!(annotation.test, "test");
    }

    #[test]
    fn test_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let rate_rule = RateRuleBuilder::new(&model, "s1", "s1 + s2")
            .annotation_serde(&TestAnnotation {
                test: "test".to_string(),
            })
            .expect("Failed to set annotation")
            .build();

        let annotation = rate_rule
            .get_annotation_serde::<TestAnnotation>()
            .expect("Failed to deserialize annotation");

        assert_eq!(annotation.test, "test");
    }

    #[test]
    fn test_annotation() {
        let doc = SBMLDocument::new(3, 2);
        let model = Model::new(&doc, "test");
        let rate_rule = RateRuleBuilder::new(&model, "s1", "s1 + s2")
            .annotation("<test>test</test>")
            .expect("Failed to set annotation")
            .build();
        assert_eq!(
            rate_rule
                .get_annotation()
                .replace("\n", "")
                .replace(' ', ""),
            "<annotation><test>test</test></annotation>"
        );
    }
}
