#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayPoolEventSource {
    __cordl_parent: crate::System::Diagnostics::Tracing::EventSource,
}
#[cfg(feature = "System+Buffers+ArrayPoolEventSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::ArrayPoolEventSource =>
    "System.Buffers"."ArrayPoolEventSource"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Buffers::ArrayPoolEventSource_BufferAllocatedReason => "System.Buffers"
    ."ArrayPoolEventSource/BufferAllocatedReason"
);
