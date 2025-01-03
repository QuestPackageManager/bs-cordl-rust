#[cfg(feature = "System+Resources+ResourceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ResourceSets: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub _resourceSets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::System::Resources::ResourceSet,
        >,
    >,
    pub MainAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    pub _neutralResourcesCulture: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CultureInfo,
    >,
    pub _lastUsedResourceCache: quest_hook::libil2cpp::Gc<
        crate::System::Resources::ResourceManager_CultureNameResourceSetPair,
    >,
    pub UseManifest: bool,
    pub UseSatelliteAssem: bool,
    pub _fallbackLoc: crate::System::Resources::UltimateResourceFallbackLocation,
    pub _callingAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    pub m_callingAssembly: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::RuntimeAssembly,
    >,
    pub resourceGroveler: quest_hook::libil2cpp::Gc<
        crate::System::Resources::IResourceGroveler,
    >,
}
#[cfg(feature = "System+Resources+ResourceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::ResourceManager =>
    "System.Resources"."ResourceManager"
);
#[cfg(feature = "System+Resources+ResourceManager")]
impl std::ops::Deref for crate::System::Resources::ResourceManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
    pub type CultureNameResourceSetPair = crate::System::Resources::ResourceManager_CultureNameResourceSetPair;
    #[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
    pub type ResourceManagerMediator = crate::System::Resources::ResourceManager_ResourceManagerMediator;
    pub fn CompareNames(
        asmTypeName1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        asmName2: quest_hook::libil2cpp::Gc<crate::System::Reflection::AssemblyName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareNames", (asmTypeName1, typeName2, asmName2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDeserialized(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", (ctx))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Resources+ResourceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Resources::ResourceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+ResourceManager+CultureNameResourceSetPair")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_CultureNameResourceSetPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
#[cfg(feature = "System+Resources+ResourceManager+ResourceManagerMediator")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_ResourceManagerMediator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _rm: quest_hook::libil2cpp::Gc<crate::System::Resources::ResourceManager>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        rm: quest_hook::libil2cpp::Gc<crate::System::Resources::ResourceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rm))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<crate::System::Resources::ResourceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rm))?;
        Ok(__cordl_ret.into())
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
