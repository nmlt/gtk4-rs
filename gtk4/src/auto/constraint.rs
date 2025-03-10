// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ConstraintAttribute;
use crate::ConstraintRelation;
use crate::ConstraintTarget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct Constraint(Object<ffi::GtkConstraint, ffi::GtkConstraintClass>);

    match fn {
        type_ => || ffi::gtk_constraint_get_type(),
    }
}

impl Constraint {
    #[doc(alias = "gtk_constraint_new")]
    pub fn new<P: IsA<ConstraintTarget>, Q: IsA<ConstraintTarget>>(
        target: Option<&P>,
        target_attribute: ConstraintAttribute,
        relation: ConstraintRelation,
        source: Option<&Q>,
        source_attribute: ConstraintAttribute,
        multiplier: f64,
        constant: f64,
        strength: i32,
    ) -> Constraint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constraint_new(
                target.map(|p| p.as_ref()).to_glib_none().0,
                target_attribute.into_glib(),
                relation.into_glib(),
                source.map(|p| p.as_ref()).to_glib_none().0,
                source_attribute.into_glib(),
                multiplier,
                constant,
                strength,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_new_constant")]
    pub fn new_constant<P: IsA<ConstraintTarget>>(
        target: Option<&P>,
        target_attribute: ConstraintAttribute,
        relation: ConstraintRelation,
        constant: f64,
        strength: i32,
    ) -> Constraint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constraint_new_constant(
                target.map(|p| p.as_ref()).to_glib_none().0,
                target_attribute.into_glib(),
                relation.into_glib(),
                constant,
                strength,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Constraint`]
    /// This method returns an instance of [`ConstraintBuilder`] which can be used to create a [`Constraint`].
    pub fn builder() -> ConstraintBuilder {
        ConstraintBuilder::default()
    }

    #[doc(alias = "gtk_constraint_get_constant")]
    #[doc(alias = "get_constant")]
    pub fn constant(&self) -> f64 {
        unsafe { ffi::gtk_constraint_get_constant(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_multiplier")]
    #[doc(alias = "get_multiplier")]
    pub fn multiplier(&self) -> f64 {
        unsafe { ffi::gtk_constraint_get_multiplier(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_relation")]
    #[doc(alias = "get_relation")]
    pub fn relation(&self) -> ConstraintRelation {
        unsafe { from_glib(ffi::gtk_constraint_get_relation(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_source")]
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Option<ConstraintTarget> {
        unsafe { from_glib_none(ffi::gtk_constraint_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_source_attribute")]
    #[doc(alias = "get_source_attribute")]
    pub fn source_attribute(&self) -> ConstraintAttribute {
        unsafe {
            from_glib(ffi::gtk_constraint_get_source_attribute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_get_strength")]
    #[doc(alias = "get_strength")]
    pub fn strength(&self) -> i32 {
        unsafe { ffi::gtk_constraint_get_strength(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_target")]
    #[doc(alias = "get_target")]
    pub fn target(&self) -> Option<ConstraintTarget> {
        unsafe { from_glib_none(ffi::gtk_constraint_get_target(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_target_attribute")]
    #[doc(alias = "get_target_attribute")]
    pub fn target_attribute(&self) -> ConstraintAttribute {
        unsafe {
            from_glib(ffi::gtk_constraint_get_target_attribute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_is_attached")]
    pub fn is_attached(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_attached(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_is_constant")]
    pub fn is_constant(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_constant(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_is_required")]
    pub fn is_required(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_required(self.to_glib_none().0)) }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Constraint`].
pub struct ConstraintBuilder {
    constant: Option<f64>,
    multiplier: Option<f64>,
    relation: Option<ConstraintRelation>,
    source: Option<ConstraintTarget>,
    source_attribute: Option<ConstraintAttribute>,
    strength: Option<i32>,
    target: Option<ConstraintTarget>,
    target_attribute: Option<ConstraintAttribute>,
}

impl ConstraintBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ConstraintBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Constraint`].
    pub fn build(self) -> Constraint {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref constant) = self.constant {
            properties.push(("constant", constant));
        }
        if let Some(ref multiplier) = self.multiplier {
            properties.push(("multiplier", multiplier));
        }
        if let Some(ref relation) = self.relation {
            properties.push(("relation", relation));
        }
        if let Some(ref source) = self.source {
            properties.push(("source", source));
        }
        if let Some(ref source_attribute) = self.source_attribute {
            properties.push(("source-attribute", source_attribute));
        }
        if let Some(ref strength) = self.strength {
            properties.push(("strength", strength));
        }
        if let Some(ref target) = self.target {
            properties.push(("target", target));
        }
        if let Some(ref target_attribute) = self.target_attribute {
            properties.push(("target-attribute", target_attribute));
        }
        glib::Object::new::<Constraint>(&properties)
            .expect("Failed to create an instance of Constraint")
    }

    pub fn constant(mut self, constant: f64) -> Self {
        self.constant = Some(constant);
        self
    }

    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = Some(multiplier);
        self
    }

    pub fn relation(mut self, relation: ConstraintRelation) -> Self {
        self.relation = Some(relation);
        self
    }

    pub fn source<P: IsA<ConstraintTarget>>(mut self, source: &P) -> Self {
        self.source = Some(source.clone().upcast());
        self
    }

    pub fn source_attribute(mut self, source_attribute: ConstraintAttribute) -> Self {
        self.source_attribute = Some(source_attribute);
        self
    }

    pub fn strength(mut self, strength: i32) -> Self {
        self.strength = Some(strength);
        self
    }

    pub fn target<P: IsA<ConstraintTarget>>(mut self, target: &P) -> Self {
        self.target = Some(target.clone().upcast());
        self
    }

    pub fn target_attribute(mut self, target_attribute: ConstraintAttribute) -> Self {
        self.target_attribute = Some(target_attribute);
        self
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Constraint")
    }
}
