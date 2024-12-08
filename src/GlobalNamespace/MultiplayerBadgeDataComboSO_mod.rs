#[cfg(feature = "MultiplayerBadgeDataComboSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeDataComboSO {
    __cordl_parent: crate::GlobalNamespace::MultiplayerBadgeDataMinMaxIntSO,
}
#[cfg(feature = "MultiplayerBadgeDataComboSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerBadgeDataComboSO =>
    ""."MultiplayerBadgeDataComboSO"
);
#[cfg(feature = "MultiplayerBadgeDataComboSO")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBadgeDataComboSO {
    type Target = crate::GlobalNamespace::MultiplayerBadgeDataMinMaxIntSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataComboSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBadgeDataComboSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataComboSO")]
impl crate::GlobalNamespace::MultiplayerBadgeDataComboSO {
    pub fn GetValue(
        &mut self,
        result: *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetValue", (result))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "MultiplayerBadgeDataComboSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBadgeDataComboSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
