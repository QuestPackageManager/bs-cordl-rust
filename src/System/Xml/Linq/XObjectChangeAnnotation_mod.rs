#[cfg(feature = "cordl_class_System+Xml+Linq+XObjectChangeAnnotation")]
#[repr(C)]
#[derive(Debug)]
pub struct XObjectChangeAnnotation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub changing: quest_hook::libil2cpp::Gc<
        crate::System::EventHandler_1<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObjectChangeEventArgs>,
        >,
    >,
    pub changed: quest_hook::libil2cpp::Gc<
        crate::System::EventHandler_1<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XObjectChangeEventArgs>,
        >,
    >,
}
#[cfg(feature = "cordl_class_System+Xml+Linq+XObjectChangeAnnotation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Linq::XObjectChangeAnnotation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Linq";
    const CLASS_NAME: &'static str = "XObjectChangeAnnotation";
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
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl std::ops::Deref for crate::System::Xml::Linq::XObjectChangeAnnotation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XObjectChangeAnnotation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl crate::System::Xml::Linq::XObjectChangeAnnotation {}
#[cfg(feature = "cordl_class_System+Xml+Linq+XObjectChangeAnnotation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XObjectChangeAnnotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
