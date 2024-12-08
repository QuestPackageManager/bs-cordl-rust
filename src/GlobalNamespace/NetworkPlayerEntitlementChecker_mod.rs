#[cfg(feature = "NetworkPlayerEntitlementChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPlayerEntitlementChecker {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rpcManager: *mut crate::GlobalNamespace::IMenuRpcManager,
    pub _additionalContentModel: *mut crate::GlobalNamespace::IAdditionalContentModel,
    pub _entitlementModel: *mut crate::GlobalNamespace::IEntitlementModel,
}
#[cfg(feature = "NetworkPlayerEntitlementChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetworkPlayerEntitlementChecker
    => ""."NetworkPlayerEntitlementChecker"
);
#[cfg(feature = "NetworkPlayerEntitlementChecker")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkPlayerEntitlementChecker {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayerEntitlementChecker")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetworkPlayerEntitlementChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayerEntitlementChecker")]
impl crate::GlobalNamespace::NetworkPlayerEntitlementChecker {
    #[cfg(feature = "NetworkPlayerEntitlementChecker+_GetEntitlementStatus_d__7")]
    pub type _GetEntitlementStatus_d__7 = crate::GlobalNamespace::NetworkPlayerEntitlementChecker__GetEntitlementStatus_d__7;
    #[cfg(feature = "NetworkPlayerEntitlementChecker+_HandleGetIsEntitledToLevel_d__6")]
    pub type _HandleGetIsEntitledToLevel_d__6 = crate::GlobalNamespace::NetworkPlayerEntitlementChecker__HandleGetIsEntitledToLevel_d__6;
    pub fn GetEntitlementStatus(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::EntitlementsStatus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::EntitlementsStatus,
        > = __cordl_object.invoke("GetEntitlementStatus", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleDataInvalidated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDataInvalidated", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGetIsEntitledToLevel(
        &mut self,
        userId: *mut crate::System::String,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGetIsEntitledToLevel", (userId, levelId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "NetworkPlayerEntitlementChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetworkPlayerEntitlementChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
