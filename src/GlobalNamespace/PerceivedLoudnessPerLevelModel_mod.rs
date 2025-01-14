#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PerceivedLoudnessPerLevelModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _loudnessLevelPerLevelId: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            f32,
        >,
    >,
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PerceivedLoudnessPerLevelModel";
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
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl std::ops::Deref for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    pub const kDefaultLoudness: f32 = -6f32;
    pub const kPerceivedLoudnessTarget: f32 = -11f32;
    pub const kSfxLoudnessTarget: f32 = -10f32;
    pub fn GetLoudnessByLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                f32,
                1usize,
            >("GetLoudnessByLevelId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLoudnessByLevelId", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (levelId)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLoudnessByLevelIdOrNull(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f32>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::Nullable_1<f32>,
                1usize,
            >("GetLoudnessByLevelIdOrNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLoudnessByLevelIdOrNull", 1usize
                )
            });
        let __cordl_ret: crate::System::Nullable_1<f32> = unsafe {
            method.invoke_unchecked(self, (levelId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLoudnessCorrectionByLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                f32,
                1usize,
            >("GetLoudnessCorrectionByLevelId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLoudnessCorrectionByLevelId", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (levelId)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxSfxVolumeByLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                f32,
                1usize,
            >("GetMaxSfxVolumeByLevelId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMaxSfxVolumeByLevelId", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (levelId)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        loudnessPerLeveDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerceivedLoudnessSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loudnessPerLeveDataList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        loudnessPerLeveDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerceivedLoudnessSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PerceivedLoudnessSO,
                        >,
                    >,
                >),
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
            method.invoke_unchecked(self, (loudnessPerLeveDataList))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
