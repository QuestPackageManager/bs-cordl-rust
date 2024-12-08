#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
#[repr(C)]
#[derive(Debug)]
pub struct XObjectChangeAnnotation {
    __cordl_parent: crate::System::Object,
    pub changing: *mut crate::System::EventHandler_1<
        *mut crate::System::Xml::Linq::XObjectChangeEventArgs,
    >,
    pub changed: *mut crate::System::EventHandler_1<
        *mut crate::System::Xml::Linq::XObjectChangeEventArgs,
    >,
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XObjectChangeAnnotation =>
    "System.Xml.Linq"."XObjectChangeAnnotation"
);
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl std::ops::Deref for crate::System::Xml::Linq::XObjectChangeAnnotation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XObjectChangeAnnotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl crate::System::Xml::Linq::XObjectChangeAnnotation {}
#[cfg(feature = "System+Xml+Linq+XObjectChangeAnnotation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XObjectChangeAnnotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
