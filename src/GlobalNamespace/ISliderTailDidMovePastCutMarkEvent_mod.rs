#[cfg(feature = "ISliderTailDidMovePastCutMarkEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ISliderTailDidMovePastCutMarkEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISliderTailDidMovePastCutMarkEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISliderTailDidMovePastCutMarkEvent";
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
#[cfg(feature = "ISliderTailDidMovePastCutMarkEvent")]
impl std::ops::Deref for crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISliderTailDidMovePastCutMarkEvent")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISliderTailDidMovePastCutMarkEvent")]
impl crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent {
    pub fn HandleSliderTailDidMovePastCutMark(
        &mut self,
        sliderController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderTailDidMovePastCutMark", (sliderController))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISliderTailDidMovePastCutMarkEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
