//! This module provides a safe Rust interface to the libSBML SpeciesReference class.
//!
//! The SpeciesReference class represents a reference to a species in an SBML model.
//! It can represent references to species, such as reactants, products, or other biochemical
//! processes. Each species reference can have properties like stoichiometry, and compartment.
//!
//! This wrapper provides safe access to the underlying C++ libSBML SpeciesReference class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use cxx::let_cxx_string;
use std::{cell::RefCell, pin::Pin, rc::Rc};

use crate::{
    inner, pin_ptr,
    reaction::Reaction,
    sbmlcxx::{self},
    sbo_term,
    traits::fromptr::FromPtr,
    upcast, upcast_annotation, upcast_pin,
};

/// A safe wrapper around the libSBML SpeciesReference class.
///
/// This struct maintains a reference to the underlying C++ SpeciesReference object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct SpeciesReference<'a> {
    inner: RefCell<Pin<&'a mut sbmlcxx::SpeciesReference>>,
}

// Set the inner trait for the SpeciesReference struct
inner!(sbmlcxx::SpeciesReference, SpeciesReference<'a>);

// Set the annotation trait for the SpeciesReference struct
upcast_annotation!(
    SpeciesReference<'a>,
    sbmlcxx::SpeciesReference,
    sbmlcxx::SBase
);

impl<'a> SpeciesReference<'a> {
    /// Creates a new SimpleSpeciesReference instance within the given Reaction.
    ///
    /// # Arguments
    /// * `reaction` - The parent Reaction that will contain this species reference
    /// * `sid` - The identifier for this species reference
    ///
    /// # Returns
    /// A new SpeciesReference instance
    pub(crate) fn new(reaction: &Reaction<'a>, sid: &str, ref_type: SpeciesReferenceType) -> Self {
        let species_reference_ptr = match ref_type {
            SpeciesReferenceType::Reactant => {
                reaction.inner().borrow_mut().as_mut().createReactant()
            }
            SpeciesReferenceType::Product => reaction.inner().borrow_mut().as_mut().createProduct(),
        };
        let mut species_reference = pin_ptr!(species_reference_ptr, sbmlcxx::SpeciesReference);

        species_reference.as_mut().initDefaults();

        // We need to fall back to custom wrappers for the species reference
        // because autocxx does not support setting the species reference's species
        // most likely because it is a virtual base class.
        let_cxx_string!(sid = sid);
        let simple_spec_ref = upcast_pin!(
            species_reference,
            sbmlcxx::SpeciesReference,
            sbmlcxx::SimpleSpeciesReference
        );

        simple_spec_ref.setSpecies(&sid);

        Self {
            inner: RefCell::new(species_reference),
        }
    }

    /// Returns a reference to the inner RefCell containing the SpeciesReference pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::SpeciesReference>> {
        &self.inner
    }

    /// Returns the species of the species reference.
    ///
    /// # Returns
    /// A string containing the species of the species reference.
    pub fn species(&self) -> String {
        let simple_spec_ref = upcast!(
            self,
            sbmlcxx::SpeciesReference,
            sbmlcxx::SimpleSpeciesReference
        );
        simple_spec_ref.getSpecies().to_str().unwrap().to_string()
    }

    /// Sets the species of the species reference.
    ///
    /// # Arguments
    /// * `species` - The species to set
    pub fn set_species(&self, species: &str) {
        let simple_spec_ref = upcast!(
            self,
            sbmlcxx::SpeciesReference,
            sbmlcxx::SimpleSpeciesReference
        );

        let_cxx_string!(species = species);
        simple_spec_ref.setSpecies(&species);
    }

    /// Returns the stoichiometry of the species reference.
    ///
    /// # Returns
    /// A string containing the stoichiometry of the species reference.
    pub fn stoichiometry(&self) -> f64 {
        self.inner.borrow().getStoichiometry()
    }

    /// Sets the stoichiometry of the species reference.
    ///
    /// # Arguments
    /// * `stoichiometry` - The stoichiometry to set
    pub fn set_stoichiometry(&self, stoichiometry: f64) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setStoichiometry(stoichiometry);
    }

    /// Returns whether the species reference is constant.
    ///
    /// # Returns
    /// A boolean indicating whether the species reference is constant.
    pub fn constant(&self) -> bool {
        self.inner.borrow().getConstant()
    }

    /// Sets whether the species reference is constant.
    ///
    /// # Arguments
    /// * `constant` - The constant to set
    pub fn set_constant(&self, constant: bool) {
        self.inner.borrow_mut().as_mut().setConstant(constant);
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::SpeciesReference, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::SpeciesReference> for SpeciesReference<'_> {
    /// Creates a new SpeciesReference instance from a unique pointer to a libSBML SpeciesReference.
    ///
    /// This method is primarily used internally by the Model class to create
    /// SpeciesReference instances from libSBML SpeciesReference pointers.
    ///
    /// # Arguments
    fn from_ptr(ptr: *mut sbmlcxx::SpeciesReference) -> Self {
        let species_reference = pin_ptr!(ptr, sbmlcxx::SpeciesReference);
        Self {
            inner: RefCell::new(species_reference),
        }
    }
}
/// Represents the type of a species reference in a reaction.
///
/// This enum is used internally to specify whether a species reference
/// represents a reactant or product when creating new species references
/// in a reaction.
pub enum SpeciesReferenceType {
    /// Indicates that the species reference is a reactant in the reaction
    Reactant,
    /// Indicates that the species reference is a product in the reaction  
    Product,
}

/// A builder for constructing SpeciesReference instances with a fluent API.
///
/// This struct provides a builder pattern interface for creating and configuring
/// SpeciesReference objects. It allows chaining method calls to set various properties
/// before finally constructing the SpeciesReference.
pub struct SpeciesReferenceBuilder<'a> {
    species_reference: Rc<SpeciesReference<'a>>,
}

impl<'a> SpeciesReferenceBuilder<'a> {
    /// Creates a new SpeciesReferenceBuilder instance.
    ///
    /// # Arguments
    /// * `reaction` - The parent Reaction that will contain this species reference
    /// * `sid` - The species identifier for this reference
    /// * `ref_type` - The type of reference (reactant or product)
    ///
    /// # Returns
    /// A new SpeciesReferenceBuilder instance
    pub fn new(reaction: &Reaction<'a>, sid: &str, ref_type: SpeciesReferenceType) -> Self {
        let species_reference = match ref_type {
            SpeciesReferenceType::Reactant => reaction.create_reactant(sid, 1.0),
            SpeciesReferenceType::Product => reaction.create_product(sid, 1.0),
        };

        Self { species_reference }
    }

    /// Sets the stoichiometry for this species reference.
    ///
    /// # Arguments
    /// * `stoichiometry` - The stoichiometric coefficient to set
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn stoichiometry(self, stoichiometry: f64) -> Self {
        self.species_reference.set_stoichiometry(stoichiometry);
        self
    }

    /// Sets whether this species reference is constant.
    ///
    /// # Arguments
    /// * `constant` - Whether this species reference should be constant
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn constant(self, constant: bool) -> Self {
        self.species_reference.set_constant(constant);
        self
    }

    /// Sets the annotation for this species reference.
    ///
    /// # Arguments
    /// * `annotation` - The annotation string to set
    ///
    /// # Returns
    /// Result containing the builder instance or error
    pub fn annotation(self, annotation: &str) -> Result<Self, SeError> {
        self.species_reference
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
    /// Result containing the builder instance or error
    pub fn annotation_serde<T: Serialize>(self, annotation: &T) -> Result<Self, SeError> {
        self.species_reference.set_annotation_serde(annotation)?;
        Ok(self)
    }

    /// Builds and returns the configured SpeciesReference instance.
    ///
    /// # Returns
    /// The constructed SpeciesReference instance
    pub fn build(self) -> Rc<SpeciesReference<'a>> {
        self.species_reference
    }
}

#[cfg(test)]
mod tests {
    use crate::SBMLDocument;

    use super::*;

    /// Tests creating a new reactant species reference directly
    ///
    /// This test verifies that:
    /// - A species reference can be created with a reaction and species ID
    /// - The species ID is correctly set and retrievable
    #[test]
    fn test_create_species_reference() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test_model");
        let reaction = model.create_reaction("test_reaction");
        let species_reference =
            SpeciesReference::new(&reaction, "test_species", SpeciesReferenceType::Reactant);

        species_reference.set_constant(true);
        species_reference.set_stoichiometry(1.0);

        // Check that the species reference is created correctly
        assert_eq!(species_reference.species(), "test_species");
        assert_eq!(species_reference.constant(), true);
        assert_eq!(species_reference.stoichiometry(), 1.0);
    }

    /// Tests creating a new product species reference directly
    ///
    /// This test verifies that a product species reference can be created
    /// without error using the Product reference type
    #[test]
    fn test_create_product_species_reference() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test_model");
        let reaction = model.create_reaction("test_reaction");
        let species_reference =
            SpeciesReference::new(&reaction, "test_species", SpeciesReferenceType::Product);

        species_reference.set_constant(true);
        species_reference.set_stoichiometry(1.0);

        // Check that the species reference is created correctly
        assert_eq!(species_reference.species(), "test_species");
        assert_eq!(species_reference.constant(), true);
        assert_eq!(species_reference.stoichiometry(), 1.0);
    }

    /// Tests the species reference builder pattern
    ///
    /// This test verifies that:
    /// - A species reference can be created using the builder pattern
    /// - Properties like stoichiometry and constant can be set via builder methods
    /// - The built species reference has the correct property values
    #[test]
    fn test_species_reference_builder() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test_model");
        let reaction = model.create_reaction("test_reaction");
        let species_reference =
            SpeciesReferenceBuilder::new(&reaction, "test_species", SpeciesReferenceType::Reactant)
                .stoichiometry(1.0)
                .constant(true)
                .build();

        assert_eq!(species_reference.species(), "test_species");
        assert_eq!(species_reference.stoichiometry(), 1.0);
        assert_eq!(species_reference.constant(), true);
    }

    /// Tests setting string annotations via the builder
    ///
    /// This test verifies that:
    /// - String annotations can be set using the builder pattern
    /// - The annotation is correctly stored and retrievable
    /// - The annotation is wrapped in proper XML tags
    #[test]
    fn test_species_reference_builder_str_annotation() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test_model");
        let reaction = model.create_reaction("test_reaction");
        let species_reference =
            SpeciesReferenceBuilder::new(&reaction, "test_species", SpeciesReferenceType::Reactant)
                .annotation("<test>test_annotation</test>")
                .unwrap()
                .build();

        assert_eq!(
            species_reference
                .get_annotation()
                .replace("\n", "")
                .replace(' ', ""),
            "<annotation><test>test_annotation</test></annotation>"
        );
    }

    /// Tests setting and retrieving serializable annotations via the builder
    ///
    /// This test verifies that:
    /// - Serializable structs can be used as annotations
    /// - The annotation is correctly serialized, stored, and later deserialized
    /// - The deserialized data matches the original input
    #[test]
    fn test_species_reference_builder_serde_annotation() {
        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test_model");
        let reaction = model.create_reaction("test_reaction");
        let annotation = TestAnnotation {
            test: "test_annotation".to_string(),
        };
        let species_reference =
            SpeciesReferenceBuilder::new(&reaction, "test_species", SpeciesReferenceType::Reactant)
                .annotation_serde(&annotation)
                .unwrap()
                .build();

        // Extract the annotation from the species reference
        let extracted_annotation: TestAnnotation =
            species_reference.get_annotation_serde().unwrap();
        assert_eq!(extracted_annotation.test, "test_annotation");
    }
}
