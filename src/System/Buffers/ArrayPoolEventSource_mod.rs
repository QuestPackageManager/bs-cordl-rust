#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayPoolEventSource {
    __cordl_parent: crate::System::Diagnostics::Tracing::EventSource,
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Buffers::ArrayPoolEventSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Buffers";
    const CLASS_NAME: &'static str = "ArrayPoolEventSource";
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
#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
impl std::ops::Deref for crate::System::Buffers::ArrayPoolEventSource {
    type Target = crate::System::Diagnostics::Tracing::EventSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
impl std::ops::DerefMut for crate::System::Buffers::ArrayPoolEventSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
impl crate::System::Buffers::ArrayPoolEventSource {
    #[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
    pub type BufferAllocatedReason = crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason;
    pub fn BufferAllocated(
        &mut self,
        bufferId: i32,
        bufferSize: i32,
        poolId: i32,
        bucketId: i32,
        reason: crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BufferAllocated",
                (bufferId, bufferSize, poolId, bucketId, reason),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BufferRented(
        &mut self,
        bufferId: i32,
        bufferSize: i32,
        poolId: i32,
        bucketId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BufferRented", (bufferId, bufferSize, poolId, bucketId))?;
        Ok(__cordl_ret.into())
    }
    pub fn BufferReturned(
        &mut self,
        bufferId: i32,
        bufferSize: i32,
        poolId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BufferReturned", (bufferId, bufferSize, poolId))?;
        Ok(__cordl_ret.into())
    }
    pub fn BufferTrimPoll(
        &mut self,
        milliseconds: i32,
        pressure: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BufferTrimPoll", (milliseconds, pressure))?;
        Ok(__cordl_ret.into())
    }
    pub fn BufferTrimmed(
        &mut self,
        bufferId: i32,
        bufferSize: i32,
        poolId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BufferTrimmed", (bufferId, bufferSize, poolId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Buffers::ArrayPoolEventSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayPoolEventSource_BufferAllocatedReason {
    #[default]
    OverMaximumSize = 1i32,
    PoolExhausted = 2i32,
    Pooled = 0i32,
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Buffers";
    const CLASS_NAME: &'static str = "BufferAllocatedReason";
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
#[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason {
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
#[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason {
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
#[cfg(feature = "System+Buffers+ArrayPoolEventSource+BufferAllocatedReason")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason {
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
