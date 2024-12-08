#[cfg(feature = "System+Threading+SynchronizationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct SynchronizationContext {
    __cordl_parent: crate::System::Object,
    pub _props: crate::System::Threading::SynchronizationContextProperties,
}
#[cfg(feature = "System+Threading+SynchronizationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::SynchronizationContext =>
    "System.Threading"."SynchronizationContext"
);
#[cfg(feature = "System+Threading+SynchronizationContext")]
impl std::ops::Deref for crate::System::Threading::SynchronizationContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SynchronizationContext")]
impl std::ops::DerefMut for crate::System::Threading::SynchronizationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SynchronizationContext")]
impl crate::System::Threading::SynchronizationContext {
    pub fn CreateCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SynchronizationContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::SynchronizationContext = __cordl_object
            .invoke("CreateCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsWaitNotificationRequired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsWaitNotificationRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OperationCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OperationCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn OperationStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OperationStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn Post(
        &mut self,
        d: *mut crate::System::Threading::SendOrPostCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Post", (d, state))?;
        Ok(__cordl_ret)
    }
    pub fn Send(
        &mut self,
        d: *mut crate::System::Threading::SendOrPostCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (d, state))?;
        Ok(__cordl_ret)
    }
    pub fn Wait(
        &mut self,
        waitHandles: *mut quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        waitAll: bool,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Wait", (waitHandles, waitAll, millisecondsTimeout))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+SynchronizationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::SynchronizationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}