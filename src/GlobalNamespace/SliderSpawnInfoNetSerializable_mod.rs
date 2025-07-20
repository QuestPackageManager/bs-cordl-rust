#[cfg(feature = "SliderSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderSpawnInfoNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub colorType: crate::GlobalNamespace::ColorType,
    pub sliderType: crate::GlobalNamespace::SliderData_Type,
    pub hasHeadNote: bool,
    pub headTime: f32,
    pub headBeat: f32,
    pub headLineIndex: i32,
    pub headLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub headBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub headControlPointLengthMultiplier: f32,
    pub headCutDirection: crate::GlobalNamespace::NoteCutDirection,
    pub headCutDirectionAngleOffset: f32,
    pub hasTailNote: bool,
    pub tailTime: f32,
    pub tailLineIndex: i32,
    pub tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub tailBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub tailControlPointLengthMultiplier: f32,
    pub tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
    pub tailCutDirectionAngleOffset: f32,
    pub midAnchorMode: crate::GlobalNamespace::SliderMidAnchorMode,
    pub sliceCount: i32,
    pub squishAmount: f32,
    pub headNoteOffset: crate::GlobalNamespace::Vector3Serializable,
    pub headGravityBase: f32,
    pub tailNoteOffset: crate::GlobalNamespace::Vector3Serializable,
    pub tailGravityBase: f32,
    pub rotation: f32,
}
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SliderSpawnInfoNetSerializable";
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
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::Utils::NetDataReader,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Deserialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Deserialize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        colorType: crate::GlobalNamespace::ColorType,
        sliderType: crate::GlobalNamespace::SliderData_Type,
        hasHeadNote: bool,
        headTime: f32,
        headBeat: f32,
        headLineIndex: i32,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        headCutDirectionAngleOffset: f32,
        hasTailNote: bool,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailCutDirectionAngleOffset: f32,
        midAnchorMode: crate::GlobalNamespace::SliderMidAnchorMode,
        sliceCount: i32,
        squishAmount: f32,
        headNoteOffset: crate::UnityEngine::Vector3,
        headGravityBase: f32,
        tailNoteOffset: crate::UnityEngine::Vector3,
        tailGravityBase: f32,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderSpawnInfoNetSerializable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::ColorType,
                            crate::GlobalNamespace::SliderData_Type,
                            bool,
                            f32,
                            f32,
                            i32,
                            crate::GlobalNamespace::NoteLineLayer,
                            crate::GlobalNamespace::NoteLineLayer,
                            f32,
                            crate::GlobalNamespace::NoteCutDirection,
                            f32,
                            bool,
                            f32,
                            i32,
                            crate::GlobalNamespace::NoteLineLayer,
                            crate::GlobalNamespace::NoteLineLayer,
                            f32,
                            crate::GlobalNamespace::NoteCutDirection,
                            f32,
                            crate::GlobalNamespace::SliderMidAnchorMode,
                            i32,
                            f32,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            f32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
                        >,
                        27usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            27usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        colorType,
                        sliderType,
                        hasHeadNote,
                        headTime,
                        headBeat,
                        headLineIndex,
                        headLineLayer,
                        headBeforeJumpLineLayer,
                        headControlPointLengthMultiplier,
                        headCutDirection,
                        headCutDirectionAngleOffset,
                        hasTailNote,
                        tailTime,
                        tailLineIndex,
                        tailLineLayer,
                        tailBeforeJumpLineLayer,
                        tailControlPointLengthMultiplier,
                        tailCutDirection,
                        tailCutDirectionAngleOffset,
                        midAnchorMode,
                        sliceCount,
                        squishAmount,
                        headNoteOffset,
                        headGravityBase,
                        tailNoteOffset,
                        tailGravityBase,
                        rotation,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Obtain() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderSpawnInfoNetSerializable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
                        >,
                        0usize,
                    >("Obtain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Obtain",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::Utils::NetDataWriter,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Serialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Serialize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
