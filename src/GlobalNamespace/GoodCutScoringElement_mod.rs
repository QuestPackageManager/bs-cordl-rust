#[cfg(feature = "GoodCutScoringElement")]
#[repr(C)]
#[derive(Debug)]
pub struct GoodCutScoringElement {
    __cordl_parent: crate::GlobalNamespace::ScoringElement,
    pub _cutScoreBuffer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CutScoreBuffer,
    >,
    pub _multiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    pub _wouldBeCorrectCutBestPossibleMultiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
}
#[cfg(feature = "GoodCutScoringElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GoodCutScoringElement => ""
    ."GoodCutScoringElement"
);
#[cfg(feature = "GoodCutScoringElement")]
impl std::ops::Deref for crate::GlobalNamespace::GoodCutScoringElement {
    type Target = crate::GlobalNamespace::ScoringElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl std::ops::DerefMut for crate::GlobalNamespace::GoodCutScoringElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl crate::GlobalNamespace::GoodCutScoringElement {
    #[cfg(feature = "GoodCutScoringElement+Pool")]
    pub type Pool = crate::GlobalNamespace::GoodCutScoringElement_Pool;
    pub fn HandleCutScoreBufferDidFinish(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CutScoreBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidFinish", (cutScoreBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteCutInfo: crate::GlobalNamespace::NoteCutInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reinitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reinitialize", ())?;
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
    pub fn get_cutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cutScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutScoreBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyCutScoreBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyCutScoreBuffer,
        > = __cordl_object.invoke("get_cutScoreBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_executionOrder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplierEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType = __cordl_object
            .invoke("get_multiplierEventType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wouldBeCorrectCutBestPossibleMultiplierEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType = __cordl_object
            .invoke("get_wouldBeCorrectCutBestPossibleMultiplierEventType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GoodCutScoringElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl AsRef<crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver>
for crate::GlobalNamespace::GoodCutScoringElement {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GoodCutScoringElement")]
impl AsMut<crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver>
for crate::GlobalNamespace::GoodCutScoringElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct GoodCutScoringElement_Pool {
    __cordl_parent: crate::GlobalNamespace::ScoringElement_Pool_1<
        *mut crate::GlobalNamespace::GoodCutScoringElement,
    >,
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GoodCutScoringElement_Pool =>
    ""."GoodCutScoringElement/Pool"
);
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    type Target = crate::GlobalNamespace::ScoringElement_Pool_1<
        *mut crate::GlobalNamespace::GoodCutScoringElement,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl crate::GlobalNamespace::GoodCutScoringElement_Pool {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "GoodCutScoringElement+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GoodCutScoringElement_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
