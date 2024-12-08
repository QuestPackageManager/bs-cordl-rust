#[cfg(feature = "IScoreController")]
#[repr(C)]
#[derive(Debug)]
pub struct IScoreController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IScoreController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IScoreController => ""."IScoreController"
);
#[cfg(feature = "IScoreController")]
impl std::ops::Deref for IScoreController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreController")]
impl std::ops::DerefMut for IScoreController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreController")]
impl IScoreController {
    pub fn SetEnabled(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnabled", (enabled))?;
        Ok(__cordl_ret)
    }
    pub fn add_multiplierDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_multiplierDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_scoreDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scoreDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_scoringForNoteFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scoringForNoteFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_scoringForNoteStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scoringForNoteStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_immediateMaxPossibleModifiedScore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_immediateMaxPossibleModifiedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_immediateMaxPossibleMultipliedScore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_immediateMaxPossibleMultipliedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_invalidated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_invalidated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_modifiedScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_modifiedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multipliedScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_multipliedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_multiplierDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_multiplierDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_scoreDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scoreDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_scoringForNoteFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scoringForNoteFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_scoringForNoteStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scoringForNoteStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IScoreController")]
impl quest_hook::libil2cpp::ObjectType for IScoreController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
