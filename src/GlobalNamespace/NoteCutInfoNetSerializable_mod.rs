#[cfg(feature = "NoteCutInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutInfoNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub saberSpeed: f32,
    pub cutWasOk: bool,
    pub saberDir: crate::GlobalNamespace::Vector3Serializable,
    pub cutPoint: crate::GlobalNamespace::Vector3Serializable,
    pub cutNormal: crate::GlobalNamespace::Vector3Serializable,
    pub notePosition: crate::GlobalNamespace::Vector3Serializable,
    pub noteScale: crate::GlobalNamespace::Vector3Serializable,
    pub noteRotation: crate::GlobalNamespace::QuaternionSerializable,
    pub gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
    pub colorType: crate::GlobalNamespace::ColorType,
    pub noteTime: f32,
    pub noteLineIndex: i32,
    pub lineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub timeToNextColorNote: f32,
    pub moveVec: crate::GlobalNamespace::Vector3Serializable,
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutInfoNetSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutInfoNetSerializable";
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
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutInfoNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl crate::GlobalNamespace::NoteCutInfoNetSerializable {
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
    pub fn Init_ByRefMut_NoteData_Quaternion_Vector3_0(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteCutInfo,
        >,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        notePosition: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        noteScale: crate::UnityEngine::Vector3,
        moveVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutInfoNetSerializable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        > = __cordl_object
            .invoke(
                "Init",
                (noteCutInfo, noteData, notePosition, noteRotation, noteScale, moveVec),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_f32__cordl_bool_Vector3_NoteData_GameplayType_ColorType_NoteLineLayer_i32_f32_f32_Vector3_Quaternion_Vector3_Vector3_1(
        &mut self,
        saberSpeed: f32,
        cutWasOk: bool,
        saberDir: crate::UnityEngine::Vector3,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        colorType: crate::GlobalNamespace::ColorType,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
        noteLineIndex: i32,
        noteTime: f32,
        timeToNextColorNote: f32,
        notePosition: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        noteScale: crate::UnityEngine::Vector3,
        moveVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutInfoNetSerializable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        > = __cordl_object
            .invoke(
                "Init",
                (
                    saberSpeed,
                    cutWasOk,
                    saberDir,
                    cutPoint,
                    cutNormal,
                    gameplayType,
                    colorType,
                    lineLayer,
                    noteLineIndex,
                    noteTime,
                    timeToNextColorNote,
                    notePosition,
                    noteRotation,
                    noteScale,
                    moveVec,
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
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteCutInfoNetSerializable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
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
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
