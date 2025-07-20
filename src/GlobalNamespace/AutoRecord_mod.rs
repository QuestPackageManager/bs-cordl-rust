#[cfg(feature = "AutoRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoRecord {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AutoRecord")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::AutoRecord {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AutoRecord";
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
#[cfg(feature = "AutoRecord")]
impl std::ops::Deref for crate::GlobalNamespace::AutoRecord {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AutoRecord")]
impl std::ops::DerefMut for crate::GlobalNamespace::AutoRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AutoRecord")]
impl crate::GlobalNamespace::AutoRecord {
    #[cfg(feature = "AutoRecord+Beatmap")]
    pub type Beatmap = crate::GlobalNamespace::AutoRecord_Beatmap;
    pub fn AddNoteHandFrames(
        note: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteLineCount: i32,
        cutStart: f32,
        cutEnd: f32,
        trackOrientation: crate::UnityEngine::Quaternion,
        handFrames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::BeatSaber::RecPlay::PoseFrame,
                    >,
                >,
            >,
        >,
        lastCutDirections: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::NoteCutDirection>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
                            i32,
                            f32,
                            f32,
                            crate::UnityEngine::Quaternion,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<
                                            crate::BeatSaber::RecPlay::PoseFrame,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::GlobalNamespace::NoteCutDirection,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("AddNoteHandFrames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddNoteHandFrames", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        note,
                        noteLineCount,
                        cutStart,
                        cutEnd,
                        trackOrientation,
                        handFrames,
                        lastCutDirections,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSliderHandFrames(
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        noteLineCount: i32,
        cutStart: f32,
        cutEnd: f32,
        trackOrientation: crate::UnityEngine::Quaternion,
        handFrames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::BeatSaber::RecPlay::PoseFrame,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::SliderData,
                            >,
                            i32,
                            f32,
                            f32,
                            crate::UnityEngine::Quaternion,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<
                                            crate::BeatSaber::RecPlay::PoseFrame,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("AddSliderHandFrames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddSliderHandFrames", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        slider,
                        noteLineCount,
                        cutStart,
                        cutEnd,
                        trackOrientation,
                        handFrames,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayerPoseFrames(
        beatmap: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::AutoRecord_Beatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::RecPlay::PlayerPoseFrames> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::AutoRecord_Beatmap,
                        >),
                        crate::BeatSaber::RecPlay::PlayerPoseFrames,
                        1usize,
                    >("CreatePlayerPoseFrames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreatePlayerPoseFrames", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::RecPlay::PlayerPoseFrames = unsafe {
            method.invoke_unchecked((), (beatmap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LocateCutPosition(
        lineCount: i32,
        line: i32,
        layer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, i32, crate::GlobalNamespace::NoteLineLayer),
                        crate::UnityEngine::Vector3,
                        3usize,
                    >("LocateCutPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LocateCutPosition", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (lineCount, line, layer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _CreatePlayerPoseFrames_g__FixFrames_1_0(
        frames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatSaber::RecPlay::PoseFrame>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::BeatSaber::RecPlay::PoseFrame,
                            >,
                        >),
                        i32,
                        1usize,
                    >("<CreatePlayerPoseFrames>g__FixFrames|1_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "<CreatePlayerPoseFrames>g__FixFrames|1_0",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (frames))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AutoRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AutoRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AutoRecord_Beatmap {
    pub beatsPerMinute: f32,
    pub noteLineCount: i32,
    pub items: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    >,
}
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::AutoRecord_Beatmap {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AutoRecord/Beatmap";
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
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::AutoRecord_Beatmap {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::AutoRecord_Beatmap {
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
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::AutoRecord_Beatmap {
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
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::AutoRecord_Beatmap {
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
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::AutoRecord_Beatmap {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
impl crate::GlobalNamespace::AutoRecord_Beatmap {}
