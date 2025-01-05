#[cfg(feature = "SliderSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderSpawnInfoNetSerializable {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PoolableSerializable,
    >,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderSpawnInfoNetSerializable
    => ""."SliderSpawnInfoNetSerializable"
);
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PoolableSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderSpawnInfoNetSerializable")]
impl crate::GlobalNamespace::SliderSpawnInfoNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        > = __cordl_object
            .invoke(
                "Init",
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
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Obtain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
