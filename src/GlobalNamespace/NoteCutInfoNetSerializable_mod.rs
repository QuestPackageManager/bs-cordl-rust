#[cfg(feature = "NoteCutInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutInfoNetSerializable {
    __cordl_parent: PoolableSerializable,
    pub saberSpeed: f32,
    pub cutWasOk: bool,
    pub saberDir: Vector3Serializable,
    pub cutPoint: Vector3Serializable,
    pub cutNormal: Vector3Serializable,
    pub notePosition: Vector3Serializable,
    pub noteScale: Vector3Serializable,
    pub noteRotation: QuaternionSerializable,
    pub gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
    pub colorType: ColorType,
    pub noteTime: f32,
    pub noteLineIndex: i32,
    pub lineLayer: NoteLineLayer,
    pub timeToNextColorNote: f32,
    pub moveVec: Vector3Serializable,
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteCutInfoNetSerializable => ""
    ."NoteCutInfoNetSerializable"
);
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl std::ops::Deref for NoteCutInfoNetSerializable {
    type Target = PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl std::ops::DerefMut for NoteCutInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl NoteCutInfoNetSerializable {
    pub fn Init_ByRefMut_NoteData_Quaternion_Vector3_0(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
        noteData: *mut NoteData,
        notePosition: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        noteScale: crate::UnityEngine::Vector3,
        moveVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<*mut NoteCutInfoNetSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteCutInfoNetSerializable = __cordl_object
            .invoke(
                "Init",
                (noteCutInfo, noteData, notePosition, noteRotation, noteScale, moveVec),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init_f32__cordl_bool_Vector3_NoteData_GameplayType_ColorType_NoteLineLayer_i32_f32_f32_Vector3_Quaternion_Vector3_Vector3_1(
        &mut self,
        saberSpeed: f32,
        cutWasOk: bool,
        saberDir: crate::UnityEngine::Vector3,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        colorType: ColorType,
        lineLayer: NoteLineLayer,
        noteLineIndex: i32,
        noteTime: f32,
        timeToNextColorNote: f32,
        notePosition: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        noteScale: crate::UnityEngine::Vector3,
        moveVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<*mut NoteCutInfoNetSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteCutInfoNetSerializable = __cordl_object
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
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NoteCutInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType for NoteCutInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
