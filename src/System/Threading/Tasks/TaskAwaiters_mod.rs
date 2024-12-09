#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskAwaiters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskAwaiters =>
    "System.Threading.Tasks"."TaskAwaiters"
);
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskAwaiters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskAwaiters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
impl crate::System::Threading::Tasks::TaskAwaiters {}
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskAwaiters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
