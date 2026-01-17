#[cfg(feature = "cordl_class_ModestTree+Util+ValuePair")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ValuePair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_ModestTree+Util+ValuePair")]
unsafe impl quest_hook::libil2cpp::Type for crate::ModestTree::Util::ValuePair {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "ModestTree.Util";
    const CLASS_NAME: &'static str = "ValuePair";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl std::ops::Deref for crate::ModestTree::Util::ValuePair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl std::ops::DerefMut for crate::ModestTree::Util::ValuePair {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl crate::ModestTree::Util::ValuePair {
    pub fn New_T1_T2_0<T1, T2>(
        first: T1,
        second: T2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::ModestTree::Util::ValuePair_2<T1, T2>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (T1, T2),
                        quest_hook::libil2cpp::Gc<
                            crate::ModestTree::Util::ValuePair_2<T1, T2>,
                        >,
                        2usize,
                    >("New")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "New",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::ModestTree::Util::ValuePair_2<T1, T2>> =
            unsafe { cordl_method_info.invoke_unchecked((), (first, second))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_T3_1<T1, T2, T3>(
        first: T1,
        second: T2,
        third: T3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::ModestTree::Util::ValuePair_3<T1, T2, T3>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (T1, T2, T3),
                        quest_hook::libil2cpp::Gc<
                            crate::ModestTree::Util::ValuePair_3<T1, T2, T3>,
                        >,
                        3usize,
                    >("New")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "New",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::ModestTree::Util::ValuePair_3<T1, T2, T3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (first, second, third))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_T3_T4_2<T1, T2, T3, T4>(
        first: T1,
        second: T2,
        third: T3,
        fourth: T4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::ModestTree::Util::ValuePair_4<T1, T2, T3, T4>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (T1, T2, T3, T4),
                        quest_hook::libil2cpp::Gc<
                            crate::ModestTree::Util::ValuePair_4<T1, T2, T3, T4>,
                        >,
                        4usize,
                    >("New")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "New",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::ModestTree::Util::ValuePair_4<T1, T2, T3, T4>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (first, second, third, fourth))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ModestTree+Util+ValuePair")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::Util::ValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
