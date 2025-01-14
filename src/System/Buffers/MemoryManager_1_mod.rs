#[cfg(feature = "System+Buffers+MemoryManager_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryManager_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Buffers+MemoryManager_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Buffers::MemoryManager_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Buffers";
    const CLASS_NAME: &'static str = "MemoryManager`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Buffers",
                        "MemoryManager`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "System+Buffers+MemoryManager_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Buffers::MemoryManager_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetSpan(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Span_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::Span_1<T>, 0usize>("GetSpan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSpan", 0usize
                )
            });
        let __cordl_ret: crate::System::Span_1<T> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Pin(
        &mut self,
        elementIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Buffers::MemoryHandle>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), crate::System::Buffers::MemoryHandle, 1usize>("Pin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Pin", 1usize
                )
            });
        let __cordl_ret: crate::System::Buffers::MemoryHandle = unsafe {
            method.invoke_unchecked(self, (elementIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetArray(
        &mut self,
        segment: quest_hook::libil2cpp::ByRefMut<crate::System::ArraySegment_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::System::ArraySegment_1<T>>),
                bool,
                1usize,
            >("TryGetArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetArray", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (segment)) };
        Ok(__cordl_ret.into())
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
