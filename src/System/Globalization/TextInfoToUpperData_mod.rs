#[cfg(feature = "cordl_class_System+Globalization+TextInfoToUpperData")]
#[repr(C)]
#[derive(Debug)]
pub struct TextInfoToUpperData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Globalization+TextInfoToUpperData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TextInfoToUpperData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TextInfoToUpperData";
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
#[cfg(feature = "cordl_class_System+Globalization+TextInfoToUpperData")]
impl std::ops::Deref for crate::System::Globalization::TextInfoToUpperData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Globalization+TextInfoToUpperData")]
impl std::ops::DerefMut for crate::System::Globalization::TextInfoToUpperData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TextInfoToUpperData")]
impl crate::System::Globalization::TextInfoToUpperData {}
#[cfg(feature = "cordl_class_System+Globalization+TextInfoToUpperData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::TextInfoToUpperData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
