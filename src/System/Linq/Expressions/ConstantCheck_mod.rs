#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstantCheck {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::ConstantCheck =>
    "System.Linq.Expressions"."ConstantCheck"
);
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl std::ops::Deref for crate::System::Linq::Expressions::ConstantCheck {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ConstantCheck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl crate::System::Linq::Expressions::ConstantCheck {}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ConstantCheck {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
