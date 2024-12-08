#[cfg(feature = "System+Buffers+MemoryManager_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryManager_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Buffers+MemoryManager_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::MemoryManager_1 < T > =>
    "System.Buffers"."MemoryManager`1" < T >
);
#[cfg(feature = "System+Buffers+MemoryManager_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Buffers::MemoryManager_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+MemoryManager_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Buffers::MemoryManager_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+MemoryManager_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::Buffers::MemoryManager_1<T> {
    pub fn Pin(
        &mut self,
        elementIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Buffers::MemoryHandle>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Buffers::MemoryHandle = __cordl_object
            .invoke("Pin", (elementIndex))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetArray(
        &mut self,
        segment: quest_hook::libil2cpp::ByRefMut<crate::System::ArraySegment_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetArray", (segment))?;
        Ok(__cordl_ret)
    }
    pub fn GetSpan(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Span_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Span_1<T> = __cordl_object
            .invoke("GetSpan", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Buffers+MemoryManager_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::MemoryManager_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
