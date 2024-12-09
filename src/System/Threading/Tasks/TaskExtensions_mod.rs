#[cfg(feature = "System+Threading+Tasks+TaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+TaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskExtensions =>
    "System.Threading.Tasks"."TaskExtensions"
);
#[cfg(feature = "System+Threading+Tasks+TaskExtensions")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskExtensions")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskExtensions")]
impl crate::System::Threading::Tasks::TaskExtensions {}
#[cfg(feature = "System+Threading+Tasks+TaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
