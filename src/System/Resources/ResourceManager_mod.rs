#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_CultureNameResourceSetPair {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Resources::ResourceManager_CultureNameResourceSetPair => "System.Resources"
    ."ResourceManager/CultureNameResourceSetPair"
);
#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
impl std::ops::Deref
for crate::System::Resources::ResourceManager_CultureNameResourceSetPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
impl std::ops::DerefMut
for crate::System::Resources::ResourceManager_CultureNameResourceSetPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
impl crate::System::Resources::ResourceManager_CultureNameResourceSetPair {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::ResourceManager_CultureNameResourceSetPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+ResourceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager {
    __cordl_parent: crate::System::Object,
    pub ResourceSets: *mut crate::System::Collections::Hashtable,
    pub _resourceSets: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Resources::ResourceSet,
    >,
    pub MainAssembly: *mut crate::System::Reflection::Assembly,
    pub _neutralResourcesCulture: *mut crate::System::Globalization::CultureInfo,
    pub _lastUsedResourceCache: *mut crate::System::Resources::ResourceManager_CultureNameResourceSetPair,
    pub UseManifest: bool,
    pub UseSatelliteAssem: bool,
    pub _fallbackLoc: crate::System::Resources::UltimateResourceFallbackLocation,
    pub _callingAssembly: *mut crate::System::Reflection::Assembly,
    pub m_callingAssembly: *mut crate::System::Reflection::RuntimeAssembly,
    pub resourceGroveler: *mut crate::System::Resources::IResourceGroveler,
}
#[cfg(feature = "System+Resources+ResourceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::ResourceManager =>
    "System.Resources"."ResourceManager"
);
#[cfg(feature = "System+Resources+ResourceManager")]
impl std::ops::Deref for crate::System::Resources::ResourceManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceManager")]
impl std::ops::DerefMut for crate::System::Resources::ResourceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceManager")]
impl crate::System::Resources::ResourceManager {
    #[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
    pub type ResourceManagerMediator = crate::System::Resources::ResourceManager_ResourceManagerMediator;
    #[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
    pub type CultureNameResourceSetPair = crate::System::Resources::ResourceManager_CultureNameResourceSetPair;
    pub fn OnDeserialized(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", (ctx))?;
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
    pub fn OnSerializing(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerializing", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDeserializing(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializing", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Resources+ResourceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Resources::ResourceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_ResourceManagerMediator {
    __cordl_parent: crate::System::Object,
    pub _rm: *mut crate::System::Resources::ResourceManager,
}
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Resources::ResourceManager_ResourceManagerMediator => "System.Resources"
    ."ResourceManager/ResourceManagerMediator"
);
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
impl std::ops::Deref
for crate::System::Resources::ResourceManager_ResourceManagerMediator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
impl std::ops::DerefMut
for crate::System::Resources::ResourceManager_ResourceManagerMediator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
impl crate::System::Resources::ResourceManager_ResourceManagerMediator {
    pub fn _ctor(
        &mut self,
        rm: *mut crate::System::Resources::ResourceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rm))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        rm: *mut crate::System::Resources::ResourceManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rm))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::ResourceManager_ResourceManagerMediator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
