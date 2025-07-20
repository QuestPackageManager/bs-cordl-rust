#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConventionFilterTypesBinder {
    __cordl_parent: crate::Zenject::ConventionAssemblySelectionBinder,
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ConventionFilterTypesBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ConventionFilterTypesBinder";
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
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl std::ops::Deref for crate::Zenject::ConventionFilterTypesBinder {
    type Target = crate::Zenject::ConventionAssemblySelectionBinder;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl std::ops::DerefMut for crate::Zenject::ConventionFilterTypesBinder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl crate::Zenject::ConventionFilterTypesBinder {
    pub fn DerivingFromOrEqual_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        0usize,
                    >("DerivingFromOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DerivingFromOrEqual", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DerivingFromOrEqual_Type1(
        &mut self,
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("DerivingFromOrEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DerivingFromOrEqual", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (parentType))? };
        Ok(__cordl_ret.into())
    }
    pub fn DerivingFrom_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        0usize,
                    >("DerivingFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DerivingFrom", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DerivingFrom_Type1(
        &mut self,
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("DerivingFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DerivingFrom", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (parentType))? };
        Ok(__cordl_ret.into())
    }
    pub fn InNamespace(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("InNamespace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InNamespace", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ns))? };
        Ok(__cordl_ret.into())
    }
    pub fn InNamespaces_IEnumerable_1_1(
        &mut self,
        namespaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("InNamespaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InNamespaces", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (namespaces))? };
        Ok(__cordl_ret.into())
    }
    pub fn InNamespaces_Il2CppArray0(
        &mut self,
        namespaces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("InNamespaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InNamespaces", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (namespaces))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInNamespace(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        requiredNs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        2usize,
                    >("IsInNamespace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsInNamespace", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, requiredNs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchingRegex_Il2CppString0(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("MatchingRegex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatchingRegex", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchingRegex_Il2CppString_RegexOptions1(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Text::RegularExpressions::RegexOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        2usize,
                    >("MatchingRegex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatchingRegex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (pattern, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchingRegex_Regex2(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Text::RegularExpressions::Regex,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("MatchingRegex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatchingRegex", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (regex))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::ConventionBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::Type>, bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                                bool,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("Where")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Where",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (predicate))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithAttributeWhere<T>(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("WithAttributeWhere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithAttributeWhere", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (predicate))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithAttribute_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        0usize,
                    >("WithAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithAttribute", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn WithAttribute_Type1(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("WithAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (attribute))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithPrefix(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("WithPrefix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithPrefix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (prefix))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithSuffix(
        &mut self,
        suffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("WithSuffix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithSuffix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (suffix))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithoutAttribute_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        0usize,
                    >("WithoutAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithoutAttribute", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn WithoutAttribute_Type1(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConventionFilterTypesBinder,
                        >,
                        1usize,
                    >("WithoutAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WithoutAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (attribute))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::ConventionBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::ConventionBindInfo>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (bindInfo))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConventionFilterTypesBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
