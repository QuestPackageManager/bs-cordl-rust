#[cfg(feature = "PlayerNameEqualityComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerNameEqualityComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PlayerNameEqualityComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerNameEqualityComparer => ""
    ."PlayerNameEqualityComparer"
);
#[cfg(feature = "PlayerNameEqualityComparer")]
impl std::ops::Deref for PlayerNameEqualityComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerNameEqualityComparer")]
impl std::ops::DerefMut for PlayerNameEqualityComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerNameEqualityComparer")]
impl PlayerNameEqualityComparer {
    pub fn GetHashCode(
        &mut self,
        obj: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
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
    pub fn Equals(
        &mut self,
        x: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        y: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
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
#[cfg(feature = "PlayerNameEqualityComparer")]
impl quest_hook::libil2cpp::ObjectType for PlayerNameEqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
