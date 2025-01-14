#[cfg(feature = "System+TupleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TupleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+TupleExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TupleExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TupleExtensions";
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
#[cfg(feature = "System+TupleExtensions")]
impl std::ops::Deref for crate::System::TupleExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl std::ops::DerefMut for crate::System::TupleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl crate::System::TupleExtensions {
    pub fn Deconstruct<T1, T2>(
        value: quest_hook::libil2cpp::Gc<crate::System::Tuple_2<T1, T2>>,
        item1: quest_hook::libil2cpp::ByRefMut<T1>,
        item2: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Tuple_2<T1, T2>>,
                    quest_hook::libil2cpp::ByRefMut<T1>,
                    quest_hook::libil2cpp::ByRefMut<T2>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Deconstruct")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Deconstruct", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, item1, item2))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TupleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
