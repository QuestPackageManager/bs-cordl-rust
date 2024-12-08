#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
#[repr(C)]
#[derive(Debug)]
pub struct PoseSampler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PoseSampler =>
    "BeatSaber.RecPlay"."PoseSampler"
);
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::PoseSampler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::PoseSampler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl crate::BeatSaber::RecPlay::PoseSampler {}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::PoseSampler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
