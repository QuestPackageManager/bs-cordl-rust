#[cfg(feature = "ISliderHeadDidMovePastCutMarkEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ISliderHeadDidMovePastCutMarkEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISliderHeadDidMovePastCutMarkEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ISliderHeadDidMovePastCutMarkEvent => ""
    ."ISliderHeadDidMovePastCutMarkEvent"
);
#[cfg(feature = "ISliderHeadDidMovePastCutMarkEvent")]
impl std::ops::Deref for ISliderHeadDidMovePastCutMarkEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISliderHeadDidMovePastCutMarkEvent")]
impl std::ops::DerefMut for ISliderHeadDidMovePastCutMarkEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISliderHeadDidMovePastCutMarkEvent")]
impl ISliderHeadDidMovePastCutMarkEvent {
    pub fn HandleSliderStartDidMovePastCutMark(
        &mut self,
        sliderController: *mut SliderController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderStartDidMovePastCutMark", (sliderController))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISliderHeadDidMovePastCutMarkEvent")]
impl quest_hook::libil2cpp::ObjectType for ISliderHeadDidMovePastCutMarkEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}