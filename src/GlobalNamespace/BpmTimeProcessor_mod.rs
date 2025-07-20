#[cfg(feature = "BpmTimeProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmTimeProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bpmChangeDataList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData,
        >,
    >,
    pub currentBpmChangesDataIdx: i32,
}
#[cfg(feature = "BpmTimeProcessor")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BpmTimeProcessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BpmTimeProcessor";
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
#[cfg(feature = "BpmTimeProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::BpmTimeProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::BpmTimeProcessor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl crate::GlobalNamespace::BpmTimeProcessor {
    #[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
    pub type BpmChangeData = crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData;
    pub fn CalculateTime(
        prevBpmChangeData: crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData,
        beat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData, f32),
                        f32,
                        2usize,
                    >("CalculateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CalculateTime", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (prevBpmChangeData, beat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertBeatToTime(
        &mut self,
        beat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(f32), f32, 1usize>("ConvertBeatToTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertBeatToTime", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (beat))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_AudioSaveData2(
        audioSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audioSaveData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_f32_IReadOnlyList_1_0(
        startBpm: f32,
        events: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startBpm, events))?;
        Ok(__cordl_object.into())
    }
    pub fn New_f32_IReadOnlyList_1_1(
        startBpm: f32,
        bpmEventsSaveData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::BpmChangeEventData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startBpm, bpmEventsSaveData))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Reset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AudioSaveData2(
        &mut self,
        audioSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (audioSaveData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_IReadOnlyList_1_0(
        &mut self,
        startBpm: f32,
        events: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (startBpm, events))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_IReadOnlyList_1_1(
        &mut self,
        startBpm: f32,
        bpmEventsSaveData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::BpmChangeEventData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::BeatmapSaveDataVersion3::BpmChangeEventData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (startBpm, bpmEventsSaveData))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BpmTimeProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl AsRef<crate::GlobalNamespace::IBeatToTimeConverter>
for crate::GlobalNamespace::BpmTimeProcessor {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatToTimeConverter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl AsMut<crate::GlobalNamespace::IBeatToTimeConverter>
for crate::GlobalNamespace::BpmTimeProcessor {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatToTimeConverter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BpmTimeProcessor_BpmChangeData {
    pub bpmChangeStartTime: f32,
    pub bpmChangeStartBpmTime: f32,
    pub bpm: f32,
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BpmTimeProcessor/BpmChangeData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
impl crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    pub fn _ctor(
        &mut self,
        bpmChangeStartTime: f32,
        bpmChangeStartBpmTime: f32,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (bpmChangeStartTime, bpmChangeStartBpmTime, bpm),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
