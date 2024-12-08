#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
#[repr(C)]
#[derive(Debug)]
pub struct ManifestBasedResourceGroveler {
    __cordl_parent: crate::System::Object,
    pub _mediator: *mut crate::System::Resources::ResourceManager_ResourceManagerMediator,
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::ManifestBasedResourceGroveler
    => "System.Resources"."ManifestBasedResourceGroveler"
);
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl std::ops::Deref for crate::System::Resources::ManifestBasedResourceGroveler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl std::ops::DerefMut for crate::System::Resources::ManifestBasedResourceGroveler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl crate::System::Resources::ManifestBasedResourceGroveler {
    pub fn New(
        mediator: *mut crate::System::Resources::ResourceManager_ResourceManagerMediator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mediator))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        mediator: *mut crate::System::Resources::ResourceManager_ResourceManagerMediator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mediator))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::ManifestBasedResourceGroveler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
