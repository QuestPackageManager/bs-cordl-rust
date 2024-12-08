#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
#[repr(C)]
#[derive(Debug)]
pub struct IPolynomialExtensionField {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::Field::IPolynomialExtensionField =>
    "Org.BouncyCastle.Math.Field"."IPolynomialExtensionField"
);
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_MinimalPolynomial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::Field::IPolynomial,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::Field::IPolynomial = __cordl_object
            .invoke("get_MinimalPolynomial", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
