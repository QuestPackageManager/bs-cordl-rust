#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultReferenceResolver {
    __cordl_parent: crate::System::Object,
    pub _referenceCount: i32,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DefaultReferenceResolver =>
    "Newtonsoft.Json.Serialization"."DefaultReferenceResolver"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    pub fn AddReference(
        &mut self,
        context: *mut crate::System::Object,
        reference: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReference", (context, reference, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetMappings(
        &mut self,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
            *mut crate::System::String,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
            *mut crate::System::String,
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetMappings", (context))?;
        Ok(__cordl_ret)
    }
    pub fn GetReference(
        &mut self,
        context: *mut crate::System::Object,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetReference", (context, value))?;
        Ok(__cordl_ret)
    }
    pub fn IsReferenced(
        &mut self,
        context: *mut crate::System::Object,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsReferenced", (context, value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ResolveReference(
        &mut self,
        context: *mut crate::System::Object,
        reference: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ResolveReference", (context, reference))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
