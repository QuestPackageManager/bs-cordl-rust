#[cfg(feature = "BGNet+Core+ITaskUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ITaskUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Core::ITaskUtility => "BGNet.Core"
    ."ITaskUtility"
);
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl std::ops::Deref for crate::BGNet::Core::ITaskUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl std::ops::DerefMut for crate::BGNet::Core::ITaskUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl crate::BGNet::Core::ITaskUtility {
    pub fn CancellationTokenWithDelay(
        &mut self,
        timeSpan: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::CancellationToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::CancellationToken = __cordl_object
            .invoke("CancellationTokenWithDelay", (timeSpan))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith<T1, T2>(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T1>>,
        continuation: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::System::Threading::Tasks::Task_1<T1>,
                *mut crate::System::Threading::Tasks::Task_1<T2>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T2>>,
    >
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T2>,
        > = __cordl_object.invoke("ContinueWith", (task, continuation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Delay(
        &mut self,
        timeSpan: crate::System::TimeSpan,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("Delay", (timeSpan, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Action0(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("Run", (action, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_1(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::System::Threading::Tasks::Task>,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("Run", (func, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_2<T>(
        &mut self,
        func: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = __cordl_object.invoke("Run", (func, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_Task0(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Wait", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_Task_1_1<T>(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Wait", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Core::ITaskUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
