#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionAttributeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _attributeProvider: *mut quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<*mut crate::System::Attribute>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<*mut crate::System::Attribute>,
        > = __cordl_object.invoke("GetAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<*mut crate::System::Attribute>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<*mut crate::System::Attribute>,
        > = __cordl_object.invoke("GetAttributes", (inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributeProvider))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributeProvider))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
impl AsRef<crate::Newtonsoft::Json::Serialization::IAttributeProvider>
for crate::Newtonsoft::Json::Serialization::ReflectionAttributeProvider {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Serialization::IAttributeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionAttributeProvider")]
impl AsMut<crate::Newtonsoft::Json::Serialization::IAttributeProvider>
for crate::Newtonsoft::Json::Serialization::ReflectionAttributeProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Newtonsoft::Json::Serialization::IAttributeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
