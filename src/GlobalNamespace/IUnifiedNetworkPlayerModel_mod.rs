#[cfg(feature = "IUnifiedNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IUnifiedNetworkPlayerModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IUnifiedNetworkPlayerModel =>
    ""."IUnifiedNetworkPlayerModel"
);
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
impl std::ops::Deref for crate::GlobalNamespace::IUnifiedNetworkPlayerModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::IUnifiedNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
impl crate::GlobalNamespace::IUnifiedNetworkPlayerModel {
    pub fn ResetMasterServerReachability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetMasterServerReachability", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveNetworkPlayerModelType(
        &mut self,
        activeNetworkPlayerModelType: crate::GlobalNamespace::UnifiedNetworkPlayerModel_ActiveNetworkPlayerModelType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveNetworkPlayerModelType", (activeNetworkPlayerModelType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetServerFilter(
        &mut self,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetServerFilter", (selectionMask, configuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_partyRefreshingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_partyRefreshingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_code(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_code", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_publicServers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        > = __cordl_object.invoke("get_publicServers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_secret", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_partyRefreshingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_partyRefreshingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IUnifiedNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayerModel>>
for crate::GlobalNamespace::IUnifiedNetworkPlayerModel {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayerModel> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IUnifiedNetworkPlayerModel")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayerModel>>
for crate::GlobalNamespace::IUnifiedNetworkPlayerModel {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayerModel> {
        unsafe { std::mem::transmute(self) }
    }
}
