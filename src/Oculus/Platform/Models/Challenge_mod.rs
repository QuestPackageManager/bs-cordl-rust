#[cfg(feature = "Oculus+Platform+Models+Challenge")]
#[repr(C)]
#[derive(Debug)]
pub struct Challenge {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub CreationType: crate::Oculus::Platform::ChallengeCreationType,
    pub Description: *mut quest_hook::libil2cpp::Il2CppString,
    pub EndDate: crate::System::DateTime,
    pub _cordl_ID: u64,
    pub InvitedUsersOptional: *mut crate::Oculus::Platform::Models::UserList,
    pub InvitedUsers: *mut crate::Oculus::Platform::Models::UserList,
    pub Leaderboard: *mut crate::Oculus::Platform::Models::Leaderboard,
    pub ParticipantsOptional: *mut crate::Oculus::Platform::Models::UserList,
    pub Participants: *mut crate::Oculus::Platform::Models::UserList,
    pub StartDate: crate::System::DateTime,
    pub Title: *mut quest_hook::libil2cpp::Il2CppString,
    pub Visibility: crate::Oculus::Platform::ChallengeVisibility,
}
#[cfg(feature = "Oculus+Platform+Models+Challenge")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::Challenge =>
    "Oculus.Platform.Models"."Challenge"
);
#[cfg(feature = "Oculus+Platform+Models+Challenge")]
impl std::ops::Deref for crate::Oculus::Platform::Models::Challenge {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+Challenge")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::Challenge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+Challenge")]
impl crate::Oculus::Platform::Models::Challenge {
    pub fn New(o: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+Models+Challenge")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Models::Challenge {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
