#[cfg(feature = "System+Xml+Linq+LineInfoAnnotation")]
#[repr(C)]
#[derive(Debug)]
pub struct LineInfoAnnotation {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub lineNumber: i32,
    pub linePosition: i32,
}
#[cfg(feature = "System+Xml+Linq+LineInfoAnnotation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::LineInfoAnnotation =>
    "System.Xml.Linq"."LineInfoAnnotation"
);
#[cfg(feature = "System+Xml+Linq+LineInfoAnnotation")]
impl std::ops::Deref for crate::System::Xml::Linq::LineInfoAnnotation {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+LineInfoAnnotation")]
impl std::ops::DerefMut for crate::System::Xml::Linq::LineInfoAnnotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+LineInfoAnnotation")]
impl crate::System::Xml::Linq::LineInfoAnnotation {
    pub fn New(
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lineNumber, linePosition))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lineNumber, linePosition))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+LineInfoAnnotation")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Linq::LineInfoAnnotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
