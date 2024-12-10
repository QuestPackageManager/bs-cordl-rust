#[cfg(feature = "IReadonlyCutScoreBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct IReadonlyCutScoreBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IReadonlyCutScoreBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IReadonlyCutScoreBuffer => ""
    ."IReadonlyCutScoreBuffer"
);
#[cfg(feature = "IReadonlyCutScoreBuffer")]
impl std::ops::Deref for crate::GlobalNamespace::IReadonlyCutScoreBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IReadonlyCutScoreBuffer")]
impl std::ops::DerefMut for crate::GlobalNamespace::IReadonlyCutScoreBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IReadonlyCutScoreBuffer")]
impl crate::GlobalNamespace::IReadonlyCutScoreBuffer {
    pub fn RegisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidFinishReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidFinishReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_afterCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_afterCutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_afterCutSwingRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_afterCutSwingRating", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_beforeCutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeCutSwingRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beforeCutSwingRating", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_centerDistanceCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_centerDistanceCutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxPossibleCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxPossibleCutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteCutInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteCutInfo = __cordl_object
            .invoke("get_noteCutInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteScoreDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ScoreModel_NoteScoreDefinition>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
        > = __cordl_object.invoke("get_noteScoreDefinition", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IReadonlyCutScoreBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IReadonlyCutScoreBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
