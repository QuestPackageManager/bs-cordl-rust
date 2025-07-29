#[cfg(feature = "cordl_class_System+Text+RegularExpressions+CaptureCollection")]
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
#[cfg(feature = "cordl_class_System+Text+RegularExpressions+CaptureCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::CaptureCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "CaptureCollection";
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
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::CaptureCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::CaptureCollection {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+CaptureCollection")]
impl crate::System::Text::RegularExpressions::CaptureCollection {}
#[cfg(feature = "cordl_class_System+Text+RegularExpressions+CaptureCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::CaptureCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
