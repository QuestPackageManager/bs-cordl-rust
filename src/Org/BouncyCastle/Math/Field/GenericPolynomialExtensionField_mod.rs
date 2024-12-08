#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericPolynomialExtensionField {
    __cordl_parent: crate::System::Object,
    pub subfield: *mut crate::Org::BouncyCastle::Math::Field::IFiniteField,
    pub minimalPolynomial: *mut crate::Org::BouncyCastle::Math::Field::IPolynomial,
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField =>
    "Org.BouncyCastle.Math.Field"."GenericPolynomialExtensionField"
);
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        subfield: *mut crate::Org::BouncyCastle::Math::Field::IFiniteField,
        polynomial: *mut crate::Org::BouncyCastle::Math::Field::IPolynomial,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subfield, polynomial))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        subfield: *mut crate::Org::BouncyCastle::Math::Field::IFiniteField,
        polynomial: *mut crate::Org::BouncyCastle::Math::Field::IPolynomial,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subfield, polynomial))?;
        Ok(__cordl_ret)
    }
    pub fn get_Characteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Characteristic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Degree(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Degree", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Dimension(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Dimension", ())?;
        Ok(__cordl_ret)
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
    pub fn get_Subfield(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::Field::IFiniteField,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::Field::IFiniteField = __cordl_object
            .invoke("get_Subfield", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
