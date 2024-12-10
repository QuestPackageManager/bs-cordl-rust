#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
#[repr(C)]
#[derive(Debug)]
pub struct Utility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Utility =>
    "UnityEngine.UIElements.UIR"."Utility"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Utility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Utility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl crate::UnityEngine::UIElements::UIR::Utility {
    #[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBufferType")]
    pub type GPUBufferType = crate::UnityEngine::UIElements::UIR::Utility_GPUBufferType;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
    pub type GPUBuffer_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<
        T,
    >;
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIR::Utility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBufferType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utility_GPUBufferType {
    Index = 1i32,
    Vertex = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBufferType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Utility_GPUBufferType => "UnityEngine.UIElements.UIR"
    ."Utility/GPUBufferType"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Utility_GPUBuffer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub buffer: crate::System::IntPtr,
    pub elemCount: i32,
    pub elemStride: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1 < T > =>
    "UnityEngine.UIElements.UIR"."Utility/GPUBuffer`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        elementCount: i32,
        _cordl_type: crate::UnityEngine::UIElements::UIR::Utility_GPUBufferType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementCount, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateRanges(
        &mut self,
        ranges: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::GfxUpdateBufferRange,
        >,
        rangesMin: i32,
        rangesMax: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRanges", (ranges, rangesMin, rangesMax))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        elementCount: i32,
        _cordl_type: crate::UnityEngine::UIElements::UIR::Utility_GPUBufferType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementCount, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BufferPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("get_BufferPointer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ElementStride(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ElementStride", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
