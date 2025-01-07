#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryMarshal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::MemoryMarshal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "MemoryMarshal";
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
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::MemoryMarshal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::MemoryMarshal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl crate::System::Runtime::InteropServices::MemoryMarshal {
    pub fn AsBytes_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsBytes", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsBytes_Span_1_0<T>(
        span: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Span_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsBytes", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsMemory<T>(
        memory: crate::System::ReadOnlyMemory_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Memory_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Memory_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsMemory", (memory))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateReadOnlySpan<T>(
        reference: quest_hook::libil2cpp::ByRefMut<T>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::ReadOnlySpan_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateReadOnlySpan", (reference, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonNullPinnableReference_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNonNullPinnableReference", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonNullPinnableReference_Span_1_0<T>(
        span: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNonNullPinnableReference", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReference_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReference", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReference_Span_1_0<T>(
        span: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReference", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetArray<T>(
        memory: crate::System::ReadOnlyMemory_1<T>,
        segment: quest_hook::libil2cpp::ByRefMut<crate::System::ArraySegment_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetArray", (memory, segment))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MemoryMarshal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::MemoryMarshal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
