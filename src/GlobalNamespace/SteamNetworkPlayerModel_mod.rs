#[cfg(feature = "SteamNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamNetworkPlayerModel {
    __cordl_parent: crate::GlobalNamespace::PlatformNetworkPlayerModel,
}
#[cfg(feature = "SteamNetworkPlayerModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SteamNetworkPlayerModel => ""
    ."SteamNetworkPlayerModel"
);
#[cfg(feature = "SteamNetworkPlayerModel")]
impl std::ops::Deref for crate::GlobalNamespace::SteamNetworkPlayerModel {
    type Target = crate::GlobalNamespace::PlatformNetworkPlayerModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamNetworkPlayerModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SteamNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamNetworkPlayerModel")]
impl crate::GlobalNamespace::SteamNetworkPlayerModel {
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
#[cfg(feature = "SteamNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamNetworkPlayerModel")]
impl AsRef<crate::GlobalNamespace::INetworkPlayerModel>
for crate::GlobalNamespace::SteamNetworkPlayerModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::INetworkPlayerModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SteamNetworkPlayerModel")]
impl AsMut<crate::GlobalNamespace::INetworkPlayerModel>
for crate::GlobalNamespace::SteamNetworkPlayerModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INetworkPlayerModel {
        unsafe { std::mem::transmute(self) }
    }
}
