#[cfg(feature = "UnityScenesHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityScenesHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityScenesHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for UnityScenesHelper => ""."UnityScenesHelper"
);
#[cfg(feature = "UnityScenesHelper")]
impl std::ops::Deref for UnityScenesHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl std::ops::DerefMut for UnityScenesHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl UnityScenesHelper {}
#[cfg(feature = "UnityScenesHelper")]
impl quest_hook::libil2cpp::ObjectType for UnityScenesHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}