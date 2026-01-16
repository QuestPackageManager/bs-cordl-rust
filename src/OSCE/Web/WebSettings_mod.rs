#[cfg(feature = "cordl_class_OSCE+Web+WebSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct WebSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OSCE+Web+WebSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::OSCE::Web::WebSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OSCE.Web";
    const CLASS_NAME: &'static str = "WebSettings";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OSCE+Web+WebSettings")]
impl std::ops::Deref for crate::OSCE::Web::WebSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OSCE+Web+WebSettings")]
impl std::ops::DerefMut for crate::OSCE::Web::WebSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OSCE+Web+WebSettings")]
impl crate::OSCE::Web::WebSettings {}
#[cfg(feature = "cordl_class_OSCE+Web+WebSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::OSCE::Web::WebSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
