#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntryList")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardEntryList {
    __cordl_parent: crate::Oculus::Platform::Models::DeserializableList_1<
        *mut crate::Oculus::Platform::Models::LeaderboardEntry,
    >,
    pub TotalCount: u64,
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntryList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::LeaderboardEntryList
    => "Oculus.Platform.Models"."LeaderboardEntryList"
);
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntryList")]
impl std::ops::Deref for crate::Oculus::Platform::Models::LeaderboardEntryList {
    type Target = crate::Oculus::Platform::Models::DeserializableList_1<
        *mut crate::Oculus::Platform::Models::LeaderboardEntry,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntryList")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::LeaderboardEntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntryList")]
impl crate::Oculus::Platform::Models::LeaderboardEntryList {
    pub fn _ctor(
        &mut self,
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntryList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::LeaderboardEntryList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
