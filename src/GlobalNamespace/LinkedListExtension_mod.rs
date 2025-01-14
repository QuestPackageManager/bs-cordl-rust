#[cfg(feature = "LinkedListExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct LinkedListExtension {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LinkedListExtension")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LinkedListExtension {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LinkedListExtension";
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
#[cfg(feature = "LinkedListExtension")]
impl std::ops::Deref for crate::GlobalNamespace::LinkedListExtension {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LinkedListExtension")]
impl std::ops::DerefMut for crate::GlobalNamespace::LinkedListExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LinkedListExtension")]
impl crate::GlobalNamespace::LinkedListExtension {
    pub fn Index<T>(
        searchNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<T>,
                >),
                i32,
                1usize,
            >("Index")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Index", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (searchNode)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LinkedListExtension")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LinkedListExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
