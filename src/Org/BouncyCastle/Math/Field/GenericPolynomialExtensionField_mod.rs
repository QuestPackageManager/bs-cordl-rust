#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericPolynomialExtensionField {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub subfield: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::Field::IFiniteField,
    >,
    pub minimalPolynomial: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::Field::IPolynomial,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        subfield: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        >,
        polynomial: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IPolynomial,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subfield, polynomial))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        subfield: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        >,
        polynomial: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IPolynomial,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subfield, polynomial))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Characteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Characteristic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Degree(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Degree", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Dimension(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Dimension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinimalPolynomial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Field::IPolynomial>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IPolynomial,
        > = __cordl_object.invoke("get_MinimalPolynomial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Subfield(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Field::IFiniteField>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        > = __cordl_object.invoke("get_Subfield", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl AsRef<crate::Org::BouncyCastle::Math::Field::IExtensionField>
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::Field::IExtensionField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl AsMut<crate::Org::BouncyCastle::Math::Field::IExtensionField>
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Math::Field::IExtensionField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl AsRef<crate::Org::BouncyCastle::Math::Field::IFiniteField>
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::Field::IFiniteField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl AsMut<crate::Org::BouncyCastle::Math::Field::IFiniteField>
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Math::Field::IFiniteField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl AsRef<crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField>
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+GenericPolynomialExtensionField")]
impl AsMut<crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField>
for crate::Org::BouncyCastle::Math::Field::GenericPolynomialExtensionField {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
        unsafe { std::mem::transmute(self) }
    }
}
