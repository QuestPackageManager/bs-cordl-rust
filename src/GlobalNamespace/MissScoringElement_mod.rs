#[cfg(feature = "MissScoringElement")]
#[repr(C)]
#[derive(Debug)]
pub struct MissScoringElement {
    __cordl_parent: crate::GlobalNamespace::ScoringElement,
    pub _multiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
    pub _wouldBeCorrectCutBestPossibleMultiplierEventType: crate::GlobalNamespace::ScoreMultiplierCounter_MultiplierEventType,
}
#[cfg(feature = "MissScoringElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissScoringElement => ""
    ."MissScoringElement"
);
#[cfg(feature = "MissScoringElement")]
impl std::ops::Deref for crate::GlobalNamespace::MissScoringElement {
    type Target = crate::GlobalNamespace::ScoringElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissScoringElement")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissScoringElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissScoringElement")]
impl crate::GlobalNamespace::MissScoringElement {
    #[cfg(feature = "MissScoringElement+Pool")]
    pub type Pool = crate::GlobalNamespace::MissScoringElement_Pool;
    pub fn Init(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_cutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cutScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_executionOrder", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissScoringElement")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissScoringElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissScoringElement+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct MissScoringElement_Pool {
    __cordl_parent: crate::GlobalNamespace::ScoringElement_Pool_1<
        *mut crate::GlobalNamespace::MissScoringElement,
    >,
}
#[cfg(feature = "MissScoringElement+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissScoringElement_Pool => ""
    ."MissScoringElement/Pool"
);
#[cfg(feature = "MissScoringElement+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::MissScoringElement_Pool {
    type Target = crate::GlobalNamespace::ScoringElement_Pool_1<
        *mut crate::GlobalNamespace::MissScoringElement,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissScoringElement+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissScoringElement_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissScoringElement+Pool")]
impl crate::GlobalNamespace::MissScoringElement_Pool {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MissScoringElement+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissScoringElement_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
