#[cfg(feature = "System+MemoryDebugView_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryDebugView_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+MemoryDebugView_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MemoryDebugView_1 < T > => "System"
    ."MemoryDebugView`1" < T >
);
#[cfg(feature = "System+MemoryDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::MemoryDebugView_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::MemoryDebugView_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::MemoryDebugView_1<T> {}
#[cfg(feature = "System+MemoryDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::MemoryDebugView_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
