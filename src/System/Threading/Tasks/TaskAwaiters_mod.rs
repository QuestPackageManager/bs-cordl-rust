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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskAwaiters")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskAwaiters {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task,
                        >),
                        crate::System::Threading::Tasks::ForceAsyncAwaiter,
                        1usize,
                    >("ForceAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ForceAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Threading::Tasks::ForceAsyncAwaiter = unsafe {
            method.invoke_unchecked((), (task))?
        };
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
