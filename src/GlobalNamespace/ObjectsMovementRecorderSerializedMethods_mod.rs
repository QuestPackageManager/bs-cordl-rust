#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorderSerializedMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObjectsMovementRecorderSerializedMethods";
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
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl std::ops::Deref
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    pub const kBackgroundPlaybackScreenshotType: &'static str = "Background";
    pub const kFirstPersonCameraView: &'static str = "FirstPerson";
    pub const kForegroundPlaybackScreenshotType: &'static str = "Foreground";
    pub const kOffRecordingMode: &'static str = "Off";
    pub const kPlaybackRecordingMode: &'static str = "Playback";
    pub const kRecordRecordingMode: &'static str = "Record";
    pub const kThirdPersonCameraView: &'static str = "ThirdPerson";
    pub fn CameraViewFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraView: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
                            >,
                        ),
                        bool,
                        2usize,
                    >("CameraViewFromSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CameraViewFromSerializedName", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, cameraView))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ModeFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ModeFromSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ModeFromSerializedName", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (name, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn PlaybackScreenshotTypeFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
                            >,
                        ),
                        bool,
                        2usize,
                    >("PlaybackScreenshotTypeFromSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PlaybackScreenshotTypeFromSerializedName",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName_ObjectsMovementRecorder_CameraView1(
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::ObjectsMovementRecorder_CameraView),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("SerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (cameraView))? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName_ObjectsMovementRecorder_Mode0(
        mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::ObjectsMovementRecorder_Mode),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("SerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName_PlaybackScreenshot_PlaybackRenderer_Type2(
        _cordl_type: crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("SerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
