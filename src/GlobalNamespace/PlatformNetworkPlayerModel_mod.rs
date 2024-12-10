#[cfg(feature = "PlatformNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformNetworkPlayerModel {
    __cordl_parent: crate::GlobalNamespace::BaseNetworkPlayerModel,
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformNetworkPlayerModel =>
    ""."PlatformNetworkPlayerModel"
);
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    type Target = crate::GlobalNamespace::BaseNetworkPlayerModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl crate::GlobalNamespace::PlatformNetworkPlayerModel {
    #[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
    pub type CreatePartyConfig = crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig;
    pub fn DestroyPartyConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyPartyConnection", ())?;
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
    pub fn get_friends(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::INetworkPlayer,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::INetworkPlayer,
            >,
        > = __cordl_object.invoke("get_friends", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformNetworkPlayerModel_CreatePartyConfig {
    __cordl_parent: crate::GlobalNamespace::BaseNetworkPlayerModel_PartyConfig,
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig => ""
    ."PlatformNetworkPlayerModel/CreatePartyConfig"
);
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl std::ops::Deref
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    type Target = crate::GlobalNamespace::BaseNetworkPlayerModel_PartyConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
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
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl AsRef<
    crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        *mut crate::GlobalNamespace::PlatformNetworkPlayerModel,
    >,
> for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        *mut crate::GlobalNamespace::PlatformNetworkPlayerModel,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl AsMut<
    crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        *mut crate::GlobalNamespace::PlatformNetworkPlayerModel,
    >,
> for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        *mut crate::GlobalNamespace::PlatformNetworkPlayerModel,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
