#[cfg(feature = "Oculus+Platform+Models+Challenge")]
#[repr(C)]
#[derive(Debug)]
pub struct Challenge {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub CreationType: crate::Oculus::Platform::ChallengeCreationType,
    pub Description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub EndDate: crate::System::DateTime,
    pub _cordl_ID: u64,
    pub InvitedUsersOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub InvitedUsers: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub Leaderboard: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::Leaderboard,
    >,
    pub ParticipantsOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub Participants: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::UserList,
    >,
    pub StartDate: crate::System::DateTime,
    pub Title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Visibility: crate::Oculus::Platform::ChallengeVisibility,
}
#[cfg(feature = "Oculus+Platform+Models+Challenge")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Models::Challenge {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "Challenge";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (o))
        };
        Ok(__cordl_ret.into())
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
