#[cfg(feature = "Oculus+Platform+Models+LeaderboardList")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardList {
    __cordl_parent: crate::Oculus::Platform::Models::DeserializableList_1<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Leaderboard>,
    >,
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Models::LeaderboardList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "LeaderboardList";
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
#[cfg(feature = "Oculus+Platform+Models+LeaderboardList")]
impl std::ops::Deref for crate::Oculus::Platform::Models::LeaderboardList {
    type Target = crate::Oculus::Platform::Models::DeserializableList_1<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Leaderboard>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardList")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::LeaderboardList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardList")]
impl crate::Oculus::Platform::Models::LeaderboardList {
    pub fn New(
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::LeaderboardList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
