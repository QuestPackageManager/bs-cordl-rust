#[cfg(feature = "System+Data+Operators")]
#[repr(C)]
#[derive(Debug)]
pub struct Operators {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Data+Operators")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Operators => "System.Data"
    ."Operators"
);
#[cfg(feature = "System+Data+Operators")]
impl std::ops::Deref for crate::System::Data::Operators {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Operators")]
impl std::ops::DerefMut for crate::System::Data::Operators {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Operators")]
impl crate::System::Data::Operators {}
#[cfg(feature = "System+Data+Operators")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Operators {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
