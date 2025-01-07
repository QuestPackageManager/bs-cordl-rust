#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskAwaiters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::TaskAwaiters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskAwaiters";
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
impl crate::System::Threading::Tasks::TaskAwaiters {
    pub fn ForceAsync(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::ForceAsyncAwaiter,
    > {
        let __cordl_ret: crate::System::Threading::Tasks::ForceAsyncAwaiter = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForceAsync", (task))?;
        Ok(__cordl_ret.into())
    }
}
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
