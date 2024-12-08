#[cfg(feature = "MultiplayerBadgesModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgesModelSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _positiveBadges: *mut crate::System::Collections::Generic::List_1<
        *mut MultiplayerBadgeDataSO,
    >,
    pub _negativeBadges: *mut crate::System::Collections::Generic::List_1<
        *mut MultiplayerBadgeDataSO,
    >,
}
#[cfg(feature = "MultiplayerBadgesModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerBadgesModelSO => ""
    ."MultiplayerBadgesModelSO"
);
#[cfg(feature = "MultiplayerBadgesModelSO")]
impl std::ops::Deref for MultiplayerBadgesModelSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgesModelSO")]
impl std::ops::DerefMut for MultiplayerBadgesModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgesModelSO")]
impl MultiplayerBadgesModelSO {
    pub fn get_positiveBadges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerBadgeDataSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerBadgeDataSO,
        > = __cordl_object.invoke("get_positiveBadges", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_negativeBadges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerBadgeDataSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerBadgeDataSO,
        > = __cordl_object.invoke("get_negativeBadges", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerBadgesModelSO")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerBadgesModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
