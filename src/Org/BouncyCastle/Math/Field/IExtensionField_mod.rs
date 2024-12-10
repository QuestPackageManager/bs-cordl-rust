#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
#[repr(C)]
#[derive(Debug)]
pub struct IExtensionField {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Field::IExtensionField
    => "Org.BouncyCastle.Math.Field"."IExtensionField"
);
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Field::IExtensionField {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Field::IExtensionField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
impl crate::Org::BouncyCastle::Math::Field::IExtensionField {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Degree(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Degree", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Field::IExtensionField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
impl AsRef<crate::Org::BouncyCastle::Math::Field::IFiniteField>
for crate::Org::BouncyCastle::Math::Field::IExtensionField {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::Field::IFiniteField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IExtensionField")]
impl AsMut<crate::Org::BouncyCastle::Math::Field::IFiniteField>
for crate::Org::BouncyCastle::Math::Field::IExtensionField {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Math::Field::IFiniteField {
        unsafe { std::mem::transmute(self) }
    }
}
