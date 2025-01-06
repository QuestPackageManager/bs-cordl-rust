#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct CaptureCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _group: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::Group,
    >,
    pub _capcount: i32,
    pub _captures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Capture>,
        >,
    >,
}
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::CaptureCollection =>
    "System.Text.RegularExpressions"."CaptureCollection"
);
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::CaptureCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::CaptureCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl crate::System::Text::RegularExpressions::CaptureCollection {}
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::CaptureCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
