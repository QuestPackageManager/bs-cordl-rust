#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegerExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::DotnetExtension::IntegerExtensions =>
    "BGLib.DotnetExtension"."IntegerExtensions"
);
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl std::ops::Deref for crate::BGLib::DotnetExtension::IntegerExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl std::ops::DerefMut for crate::BGLib::DotnetExtension::IntegerExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl crate::BGLib::DotnetExtension::IntegerExtensions {}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::DotnetExtension::IntegerExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
