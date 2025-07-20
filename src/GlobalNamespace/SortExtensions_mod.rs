#[cfg(feature = "SortExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SortExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SortExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SortExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SortExtensions";
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
#[cfg(feature = "SortExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SortExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SortExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SortExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SortExtensions")]
impl crate::GlobalNamespace::SortExtensions {
    pub fn InsertSorted<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        item: T,
        getSortIndex: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            T,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<T, i32>>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InsertSorted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertSorted", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (list, item, getSortIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sort<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        getSortIndex: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<T, i32>>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Sort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Sort",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (list, getSortIndex))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SortExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SortExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
