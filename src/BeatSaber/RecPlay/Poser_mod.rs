#[cfg(feature = "BeatSaber+RecPlay+Poser")]
#[repr(C)]
#[derive(Debug)]
pub struct Poser {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::Poser => "BeatSaber.RecPlay"
    ."Poser"
);
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::Poser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::Poser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl crate::BeatSaber::RecPlay::Poser {}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::Poser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
