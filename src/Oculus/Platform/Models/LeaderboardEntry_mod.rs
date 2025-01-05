#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardEntry {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub DisplayScore: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ExtraData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _cordl_ID: u64,
    pub Rank: i32,
    pub Score: i64,
    pub SupplementaryMetricOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::SupplementaryMetric,
    >,
    pub SupplementaryMetric: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::SupplementaryMetric,
    >,
    pub Timestamp: crate::System::DateTime,
    pub User: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::LeaderboardEntry =>
    "Oculus.Platform.Models"."LeaderboardEntry"
);
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
impl std::ops::Deref for crate::Oculus::Platform::Models::LeaderboardEntry {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::LeaderboardEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
impl crate::Oculus::Platform::Models::LeaderboardEntry {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::LeaderboardEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
