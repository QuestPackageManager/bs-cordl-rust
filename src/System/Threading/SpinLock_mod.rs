#[cfg(feature = "System+Threading+SpinLock")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpinLock {
    pub m_owner: i32,
}
#[cfg(feature = "System+Threading+SpinLock")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::SpinLock {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "SpinLock";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Threading+SpinLock")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Threading::SpinLock {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+SpinLock")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Threading::SpinLock {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Threading+SpinLock")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Threading::SpinLock {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Threading+SpinLock")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Threading::SpinLock {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Threading+SpinLock")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Threading::SpinLock {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+SpinLock")]
impl crate::System::Threading::SpinLock {
    #[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
    pub type SystemThreading_SpinLockDebugView = crate::System::Threading::SpinLock_SystemThreading_SpinLockDebugView;
    pub fn ContinueTryEnter(
        &mut self,
        millisecondsTimeout: i32,
        lockTaken: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ContinueTryEnter",
            (millisecondsTimeout, lockTaken),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueTryEnterWithThreadTracking(
        &mut self,
        millisecondsTimeout: i32,
        startTime: u32,
        lockTaken: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ContinueTryEnterWithThreadTracking",
            (millisecondsTimeout, startTime, lockTaken),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DecrementWaiters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DecrementWaiters",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Enter(
        &mut self,
        lockTaken: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Enter",
            (lockTaken),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Exit(
        &mut self,
        useMemoryBarrier: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Exit",
            (useMemoryBarrier),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitSlowPath(
        &mut self,
        useMemoryBarrier: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ExitSlowPath",
            (useMemoryBarrier),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnter(
        &mut self,
        millisecondsTimeout: i32,
        lockTaken: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryEnter",
            (millisecondsTimeout, lockTaken),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        enableThreadOwnerTracking: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (enableThreadOwnerTracking),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsHeldByCurrentThread(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsHeldByCurrentThread",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsThreadOwnerTrackingEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsThreadOwnerTrackingEnabled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct SpinLock_SystemThreading_SpinLockDebugView {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::SpinLock_SystemThreading_SpinLockDebugView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "SystemThreading_SpinLockDebugView";
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
#[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
impl std::ops::Deref
for crate::System::Threading::SpinLock_SystemThreading_SpinLockDebugView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
impl std::ops::DerefMut
for crate::System::Threading::SpinLock_SystemThreading_SpinLockDebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
impl crate::System::Threading::SpinLock_SystemThreading_SpinLockDebugView {}
#[cfg(feature = "System+Threading+SpinLock+SystemThreading_SpinLockDebugView")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::SpinLock_SystemThreading_SpinLockDebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
