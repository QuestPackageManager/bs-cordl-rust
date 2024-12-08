#[cfg(feature = "System+SpanHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct SpanHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+SpanHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::SpanHelpers => "System"."SpanHelpers"
);
#[cfg(feature = "System+SpanHelpers")]
impl std::ops::Deref for crate::System::SpanHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl std::ops::DerefMut for crate::System::SpanHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl crate::System::SpanHelpers {}
#[cfg(feature = "System+SpanHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::SpanHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}