#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct OSSpecificSynchronizationContext {
    __cordl_parent: crate::System::Threading::SynchronizationContext,
    pub m_OSSynchronizationContext: *mut crate::System::Object,
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::OSSpecificSynchronizationContext => "System.Threading"
    ."OSSpecificSynchronizationContext"
);
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
impl std::ops::Deref for crate::System::Threading::OSSpecificSynchronizationContext {
    type Target = crate::System::Threading::SynchronizationContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
impl std::ops::DerefMut for crate::System::Threading::OSSpecificSynchronizationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
impl crate::System::Threading::OSSpecificSynchronizationContext {
    #[cfg(
        feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext"
    )]
    pub type InvocationContext = crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext;
    #[cfg(
        feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
    )]
    pub type InvocationEntryDelegate = crate::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate;
    #[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+__c")]
    pub type __c = crate::System::Threading::OSSpecificSynchronizationContext___c;
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
    pub fn New(
        osContext: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (osContext))?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        osContext: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (osContext))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::OSSpecificSynchronizationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct OSSpecificSynchronizationContext_InvocationContext {
    __cordl_parent: crate::System::Object,
    pub m_Delegate: *mut crate::System::Threading::SendOrPostCallback,
    pub m_State: *mut crate::System::Object,
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::OSSpecificSynchronizationContext_InvocationContext =>
    "System.Threading"."OSSpecificSynchronizationContext/InvocationContext"
);
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
impl std::ops::Deref
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
impl std::ops::DerefMut
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
impl crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        d: *mut crate::System::Threading::SendOrPostCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (d, state))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        d: *mut crate::System::Threading::SendOrPostCallback,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (d, state))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct OSSpecificSynchronizationContext_InvocationEntryDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate =>
    "System.Threading"."OSSpecificSynchronizationContext/InvocationEntryDelegate"
);
#[cfg(
    feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
)]
impl std::ops::Deref
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
)]
impl std::ops::DerefMut
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
)]
impl crate::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate {
    pub fn Invoke(
        &mut self,
        arg: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (arg))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Threading+OSSpecificSynchronizationContext+InvocationEntryDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
