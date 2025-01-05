#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+ProviderException")]
#[repr(C)]
#[derive(Debug)]
pub struct ProviderException {
    __cordl_parent: crate::UnityEngine::ResourceManagement::Exceptions::OperationException,
    pub _Location_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+ProviderException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Exceptions::ProviderException =>
    "UnityEngine.ResourceManagement.Exceptions"."ProviderException"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+ProviderException")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Exceptions::ProviderException {
    type Target = crate::UnityEngine::ResourceManagement::Exceptions::OperationException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+ProviderException")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Exceptions::ProviderException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+ProviderException")]
impl crate::UnityEngine::ResourceManagement::Exceptions::ProviderException {
    pub fn New(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, location, innerException))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, location, innerException))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = __cordl_object.invoke("get_Location", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+ProviderException")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Exceptions::ProviderException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
