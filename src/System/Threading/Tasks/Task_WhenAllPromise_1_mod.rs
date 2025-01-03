#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_WhenAllPromise_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<
        *mut quest_hook::libil2cpp::Il2CppArray<T>,
    >,
    pub m_tasks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Threading::Tasks::Task_1<T>,
        >,
    >,
    pub m_count: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Task_WhenAllPromise_1 < T > =>
    "System.Threading.Tasks"."Task/WhenAllPromise`1" < T >
);
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::Task_WhenAllPromise_1<T> {
    type Target = crate::System::Threading::Tasks::Task_1<
        *mut quest_hook::libil2cpp::Il2CppArray<T>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::Task_WhenAllPromise_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::Task_WhenAllPromise_1<T> {
    pub fn Invoke(
        &mut self,
        ignored: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ignored))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tasks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InvokeMayRunArbitraryCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldNotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ShouldNotifyDebuggerOfWaitCompletion", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::Task_WhenAllPromise_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::GlobalNamespace::Task_WhenAllPromise_1<T> {
    fn as_ref(&self) -> &crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::GlobalNamespace::Task_WhenAllPromise_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
