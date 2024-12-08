#[cfg(feature = "System+PasteArguments")]
#[repr(C)]
#[derive(Debug)]
pub struct PasteArguments {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+PasteArguments")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::PasteArguments => "System"
    ."PasteArguments"
);
#[cfg(feature = "System+PasteArguments")]
impl std::ops::Deref for crate::System::PasteArguments {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+PasteArguments")]
impl std::ops::DerefMut for crate::System::PasteArguments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+PasteArguments")]
impl crate::System::PasteArguments {}
#[cfg(feature = "System+PasteArguments")]
impl quest_hook::libil2cpp::ObjectType for crate::System::PasteArguments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
