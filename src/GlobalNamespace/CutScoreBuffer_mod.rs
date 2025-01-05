#[cfg(feature = "CutScoreBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CutScoreBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _saberSwingRatingCounter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberSwingRatingCounter,
    >,
    pub _noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
    pub _noteScoreDefinition: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
    >,
    pub _afterCutScore: i32,
    pub _beforeCutScore: i32,
    pub _centerDistanceCutScore: i32,
    pub _initialized: bool,
    pub _isFinished: bool,
    pub _didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver,
            >,
        >,
    >,
    pub _didChangeEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver,
            >,
        >,
    >,
}
#[cfg(feature = "CutScoreBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CutScoreBuffer => ""
    ."CutScoreBuffer"
);
#[cfg(feature = "CutScoreBuffer")]
impl std::ops::Deref for crate::GlobalNamespace::CutScoreBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl std::ops::DerefMut for crate::GlobalNamespace::CutScoreBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl crate::GlobalNamespace::CutScoreBuffer {
    pub fn HandleSaberSwingRatingCounterDidChange(
        &mut self,
        swingRatingCounter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounter,
        >,
        rating: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSaberSwingRatingCounterDidChange",
                (swingRatingCounter, rating),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSaberSwingRatingCounterDidFinish(
        &mut self,
        swingRatingCounter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSaberSwingRatingCounterDidFinish", (swingRatingCounter))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Init", (noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshScores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshScores", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_executionOrder", ())?;
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
#[cfg(feature = "CutScoreBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CutScoreBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsRef<crate::GlobalNamespace::IReadonlyCutScoreBuffer>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReadonlyCutScoreBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsMut<crate::GlobalNamespace::IReadonlyCutScoreBuffer>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReadonlyCutScoreBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsRef<crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsMut<crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsRef<crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CutScoreBuffer")]
impl AsMut<crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver>
for crate::GlobalNamespace::CutScoreBuffer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
