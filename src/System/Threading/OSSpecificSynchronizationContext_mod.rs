#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct OSSpecificSynchronizationContext {
    __cordl_parent: crate::System::Threading::SynchronizationContext,
    pub m_OSSynchronizationContext: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::OSSpecificSynchronizationContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "OSSpecificSynchronizationContext";
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
    pub fn CreateCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SynchronizationContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        > = __cordl_object.invoke("CreateCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Get() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::OSSpecificSynchronizationContext,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::OSSpecificSynchronizationContext,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOSContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetOSContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvocationEntry(
        arg: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvocationEntry", (arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        osContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (osContext))?;
        Ok(__cordl_object.into())
    }
    pub fn Post(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::System::Threading::SendOrPostCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Post", (d, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn PostInternal(
        osSynchronizationContext: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        callback: crate::System::IntPtr,
        arg: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PostInternal", (osSynchronizationContext, callback, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::System::Threading::SendOrPostCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (d, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        osContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (osContext))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Delegate: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SendOrPostCallback,
    >,
    pub m_State: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "InvocationContext";
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
#[cfg(feature = "System+Threading+OSSpecificSynchronizationContext+InvocationContext")]
impl std::ops::Deref
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        d: quest_hook::libil2cpp::Gc<crate::System::Threading::SendOrPostCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (d, state))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::System::Threading::SendOrPostCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (d, state))?;
        Ok(__cordl_ret.into())
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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::OSSpecificSynchronizationContext_InvocationEntryDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "InvocationEntryDelegate";
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
