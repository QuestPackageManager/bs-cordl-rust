#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_WhenAllPromise {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >,
    pub m_tasks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Threading::Tasks::Task,
    >,
    pub m_count: i32,
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Task_WhenAllPromise =>
    "System.Threading.Tasks"."Task/WhenAllPromise"
);
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl std::ops::Deref for crate::GlobalNamespace::Task_WhenAllPromise {
    type Target = crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl std::ops::DerefMut for crate::GlobalNamespace::Task_WhenAllPromise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl crate::GlobalNamespace::Task_WhenAllPromise {
    pub fn Invoke(
        &mut self,
        ignored: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ignored))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        tasks: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tasks))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        tasks: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tasks))?;
        Ok(__cordl_ret)
    }
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InvokeMayRunArbitraryCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldNotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ShouldNotifyDebuggerOfWaitCompletion", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Task_WhenAllPromise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
