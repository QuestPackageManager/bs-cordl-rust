#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Models::LeaderboardEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "LeaderboardEntry";
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
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
impl std::ops::Deref for crate::Oculus::Platform::Models::LeaderboardEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+LeaderboardEntry")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::LeaderboardEntry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (o))?
        };
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
