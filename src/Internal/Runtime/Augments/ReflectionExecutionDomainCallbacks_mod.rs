#[cfg(feature = "Internal+Runtime+Augments+ReflectionExecutionDomainCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionExecutionDomainCallbacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Runtime+Augments+ReflectionExecutionDomainCallbacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks =>
    "Internal.Runtime.Augments"."ReflectionExecutionDomainCallbacks"
);
#[cfg(feature = "Internal+Runtime+Augments+ReflectionExecutionDomainCallbacks")]
impl std::ops::Deref
for crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+ReflectionExecutionDomainCallbacks")]
impl std::ops::DerefMut
for crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+ReflectionExecutionDomainCallbacks")]
impl crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks {
    pub fn CreateMissingMetadataException(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("CreateMissingMetadataException", (attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Runtime+Augments+ReflectionExecutionDomainCallbacks")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
