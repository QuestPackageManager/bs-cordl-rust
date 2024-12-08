#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionAttributeProvider {
    __cordl_parent: crate::System::Object,
    pub _attributeProvider: *mut crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::ReflectionAttributeProvider =>
    "Newtonsoft.Json.Serialization"."ReflectionAttributeProvider"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::ReflectionAttributeProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::ReflectionAttributeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
impl crate::Newtonsoft::Json::Serialization::ReflectionAttributeProvider {
    pub fn _ctor(
        &mut self,
        attributeProvider: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributeProvider))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<*mut crate::System::Attribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::Attribute,
        > = __cordl_object.invoke("GetAttributes", (inherit))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: *mut crate::System::Type,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<*mut crate::System::Attribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::Attribute,
        > = __cordl_object.invoke("GetAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        attributeProvider: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributeProvider))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::ReflectionAttributeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
