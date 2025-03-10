//! This module provides a safe Rust interface to the libSBML Parameter class.
//!
//! The Parameter class represents a named value in an SBML model. Parameters can be used
//! to define constants or variables that are referenced by other model elements like
//! reactions and rules. Each parameter has properties like value, units, and whether
//! it is constant.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Parameter class while
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
    upcast_annotation,
};

/// A safe wrapper around the libSBML Parameter class.
///
/// This struct maintains a reference to the underlying C++ Parameter object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Parameter<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::Parameter>>,
}

// Set the inner trait for the Parameter struct
inner!(sbmlcxx::Parameter, Parameter<'a>);

// Set the annotation trait for the Parameter struct
upcast_annotation!(Parameter<'a>, sbmlcxx::Parameter, sbmlcxx::SBase);

impl<'a> Parameter<'a> {
    /// Creates a new Parameter instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this parameter
    /// * `id` - The identifier for this parameter
    ///
    /// # Returns
    /// A new Parameter instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let parameter_ptr = model.inner().borrow_mut().as_mut().createParameter();
        let mut parameter = pin_ptr!(parameter_ptr, sbmlcxx::Parameter);

        // Set the default values for the parameter
        parameter.as_mut().initDefaults();

        // Set the id of the parameter
        let_cxx_string!(id = id);
        parameter.as_mut().setId(&id);

        Self {
            inner: RefCell::new(parameter),
        }
    }

    /// Gets the parameter's identifier.
    ///
    /// # Returns
    /// The parameter's ID as a String
    pub fn id(&self) -> String {
        self.inner.borrow().as_ref().getId().to_string()
    }

    /// Sets the parameter's identifier.
    ///
    /// # Arguments
    /// * `id` - The new identifier to set
    pub fn set_id(&self, id: &str) {
        let_cxx_string!(id = id);
        self.inner.borrow_mut().as_mut().setId(&id);
    }

    /// Gets the parameter's name.
    ///
    /// # Returns
    /// The parameter's name as a String
    pub fn name(&self) -> String {
        self.inner.borrow().as_ref().getName().to_string()
    }

    /// Sets the parameter's name.
    ///
    /// # Arguments
    /// * `name` - The new name to set
    pub fn set_name(&self, name: &str) {
        let_cxx_string!(name = name);
        self.inner.borrow_mut().as_mut().setName(&name);
    }

    /// Gets the parameter's value.
    ///
    /// # Returns
    /// Some(value) if the parameter has a value set, None otherwise
    pub fn value(&self) -> Option<f64> {
        if self.inner.borrow().as_ref().isSetValue() {
            Some(self.inner.borrow().as_ref().getValue())
        } else {
            None
        }
    }

    /// Sets the parameter's value.
    ///
    /// # Arguments
    /// * `value` - The new value to set
    pub fn set_value(&self, value: f64) {
        self.inner.borrow_mut().as_mut().setValue(value);
    }

    /// Gets the parameter's units.
    ///
    /// # Returns
    /// The parameter's units as a String
    pub fn units(&self) -> String {
        self.inner.borrow().as_ref().getUnits().to_string()
    }

    /// Sets the parameter's units.
    ///
    /// # Arguments
    /// * `units` - The new units to set
    pub fn set_units(&self, units: &str) {
        let_cxx_string!(units = units);
        self.inner.borrow_mut().as_mut().setUnits(&units);
    }

    /// Gets whether the parameter is constant.
    ///
    /// # Returns
    /// true if the parameter is constant, false otherwise
    pub fn constant(&self) -> bool {
        self.inner.borrow().as_ref().isSetConstant()
    }

    /// Sets whether the parameter is constant.
    ///
    /// # Arguments
    /// * `constant` - Whether the parameter should be constant
    pub fn set_constant(&self, constant: bool) {
        self.inner.borrow_mut().as_mut().setConstant(constant);
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Parameter, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::Parameter> for Parameter<'_> {
    /// Creates a new Parameter instance from a unique pointer to a libSBML Parameter.
    ///
    /// This method is primarily used internally by the Model class to create
    /// Parameter instances from libSBML Parameter pointers.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML Parameter
    ///
    /// # Returns
    /// A new Parameter instance
    fn from_ptr(ptr: *mut sbmlcxx::Parameter) -> Self {
        let parameter = pin_ptr!(ptr, sbmlcxx::Parameter);
        Self {
            inner: RefCell::new(parameter),
        }
    }
}
/// A builder for constructing Parameter instances with a fluent API.
///
/// This struct provides a builder pattern interface for creating and configuring
/// Parameter objects. It allows chaining method calls to set various properties
/// before finally constructing the Parameter.
pub struct ParameterBuilder<'a> {
    parameter: Rc<Parameter<'a>>,
}

impl<'a> ParameterBuilder<'a> {
    /// Creates a new ParameterBuilder instance.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this parameter
    /// * `id` - The identifier for this parameter
    ///
    /// # Returns
    /// A new ParameterBuilder instance
    pub fn new(model: &Model<'a>, id: &str) -> Self {
        let parameter = model.create_parameter(id);
        Self { parameter }
    }

    /// Sets the value for this parameter.
    ///
    /// # Arguments
    /// * `value` - The value to set
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn value(self, value: f64) -> Self {
        self.parameter.set_value(value);
        self
    }

    /// Sets the units for this parameter.
    ///
    /// # Arguments
    /// * `units` - The units to set
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn units(self, units: &str) -> Self {
        self.parameter.set_units(units);
        self
    }

    /// Sets whether this parameter is constant.
    ///
    /// # Arguments
    /// * `constant` - Whether this parameter should be constant
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn constant(self, constant: bool) -> Self {
        self.parameter.set_constant(constant);
        self
    }

    /// Sets the annotation for this parameter from a string.
    ///
    /// # Arguments
    /// * `annotation` - The annotation string to set
    ///
    /// # Returns
    /// Result containing the builder instance or error
    pub fn annotation(self, annotation: &str) -> Result<Self, SeError> {
        self.parameter
            .set_annotation(annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Sets the annotation for this parameter by serializing the provided data.
    ///
    /// # Arguments
    /// * `annotation` - The annotation data to serialize and set
    ///
    /// # Returns
    /// Result containing the builder instance or serialization error
    pub fn annotation_serde<T: serde::Serialize>(self, annotation: &T) -> Result<Self, SeError> {
        let annotation = to_string(annotation)?;
        self.parameter
            .set_annotation(&annotation)
            .map_err(|e| SeError::Custom(e.to_string()))?;
        Ok(self)
    }

    /// Builds and returns the configured Parameter.
    ///
    /// # Returns
    /// The constructed Parameter instance wrapped in an Rc
    pub fn build(self) -> Rc<Parameter<'a>> {
        self.parameter
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{model::Model, sbmldoc::SBMLDocument};

    #[test]
    fn test_parameter_creation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = Parameter::new(&model, "test");

        parameter.set_value(1.0);
        parameter.set_id("new_id");
        parameter.set_name("test_name");
        parameter.set_constant(true);
        parameter.set_units("mole");

        assert_eq!(parameter.id(), "new_id");
        assert_eq!(parameter.name(), "test_name");
        assert_eq!(parameter.constant(), true);
        assert_eq!(parameter.units(), "mole");
        assert_eq!(parameter.value(), Some(1.0));
    }

    #[test]
    fn test_parameter_builder() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = ParameterBuilder::new(&model, "test")
            .value(1.0)
            .units("mole")
            .constant(true)
            .build();

        assert_eq!(parameter.id(), "test");
        assert_eq!(parameter.value(), Some(1.0));
        assert_eq!(parameter.units(), "mole");
        assert_eq!(parameter.constant(), true);
    }

    #[test]
    fn test_parameter_annotation() {
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = ParameterBuilder::new(&model, "test")
            .annotation("<test>test</test>")
            .expect("Failed to set annotation")
            .build();
        assert_eq!(
            parameter
                .get_annotation()
                .replace("\n", "")
                .replace(" ", ""),
            "<annotation><test>test</test></annotation>"
        );
    }

    #[test]
    fn test_parameter_builder_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            test: String,
        }

        let annotation = Test {
            test: String::from("test"),
        };
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = ParameterBuilder::new(&model, "test")
            .annotation_serde(&annotation)
            .expect("Failed to set annotation")
            .build();

        let extracted: Test = parameter
            .get_annotation_serde()
            .expect("Failed to get annotation");
        assert_eq!(extracted.test, "test");
    }

    #[test]
    fn test_parameter_annotation_serde() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            test: String,
        }

        let annotation = Test {
            test: String::from("test"),
        };
        let doc = SBMLDocument::default();
        let model = Model::new(&doc, "test");
        let parameter = ParameterBuilder::new(&model, "test")
            .annotation_serde(&annotation)
            .unwrap()
            .build();

        parameter
            .set_annotation_serde(&annotation)
            .expect("Failed to set annotation");

        let extracted: Test = parameter
            .get_annotation_serde()
            .expect("Failed to get annotation");
        assert_eq!(extracted.test, "test");
    }
}
