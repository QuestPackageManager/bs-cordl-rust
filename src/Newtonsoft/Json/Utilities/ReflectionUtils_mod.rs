#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+ReflectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+ReflectionUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "ReflectionUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    pub fn CanReadMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        bool,
                    ), bool, 2usize>("CanReadMemberValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanReadMemberValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (member, nonPublic))? };
        Ok(__cordl_ret.into())
    }
    pub fn CanSetMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        nonPublic: bool,
        canSetReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        bool,
                        bool,
                    ), bool, 3usize>("CanSetMemberValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanSetMemberValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (member, nonPublic, canSetReadOnly))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureNotByRefType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("EnsureNotByRefType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureNotByRefType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureNotNullableType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("EnsureNotNullableType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureNotNullableType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblyDelimiterIndex(
        fullyQualifiedTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::System::Nullable_1<i32>,
                        1usize,
                    >("GetAssemblyDelimiterIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAssemblyDelimiterIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<i32> =
            unsafe { cordl_method_info.invoke_unchecked((), (fullyQualifiedTypeName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_Il2CppObject0<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        T,
                        1usize,
                    >("GetAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: T =
            unsafe { cordl_method_info.invoke_unchecked((), (attributeProvider))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute__cordl_bool1<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        bool,
                    ), T, 2usize>("GetAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAttribute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: T =
            unsafe { cordl_method_info.invoke_unchecked((), (attributeProvider, inherit))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes_Type__cordl_bool1(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::System::Attribute>>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                        >,
                    >, 3usize>("GetAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAttributes",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::System::Attribute>>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (attributeProvider, attributeType, inherit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes__cordl_bool0<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>, 2usize>(
                        "GetAttributes",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAttributes",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>> =
            unsafe { cordl_method_info.invoke_unchecked((), (attributeProvider, inherit))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseDefinition(
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::PropertyInfo,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                        1usize,
                    >("GetBaseDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBaseDefinition", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo> =
            unsafe { cordl_method_info.invoke_unchecked((), (propertyInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetChildPrivateFields(
        initialFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
            >,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::System::Reflection::BindingFlags,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetChildPrivateFields"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetChildPrivateFields",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (initialFields, _cordl_type, bindingAttr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetChildPrivateProperties(
        initialProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
            >,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::System::Reflection::BindingFlags,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetChildPrivateProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetChildPrivateProperties",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (initialProperties, _cordl_type, bindingAttr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCollectionItemType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetCollectionItemType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCollectionItemType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultConstructor_Type0(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::ConstructorInfo,
                        >,
                        1usize,
                    >("GetDefaultConstructor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultConstructor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo> =
            unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultConstructor__cordl_bool1(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::ConstructorInfo,
                        >,
                        2usize,
                    >("GetDefaultConstructor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultConstructor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo> =
            unsafe { cordl_method_info.invoke_unchecked((), (t, nonPublic))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultValue(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetDefaultValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDictionaryKeyValueTypes(
        dictionaryType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        keyType: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::System::Type>>,
        valueType: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::System::Type>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetDictionaryKeyValueTypes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDictionaryKeyValueTypes",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dictionaryType, keyType, valueType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFields(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::System::Reflection::BindingFlags,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
                        >,
                    >, 2usize>("GetFields")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFields",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (targetType, bindingAttr))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldsAndProperties(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::System::Reflection::BindingFlags,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        >,
                    >, 2usize>("GetFieldsAndProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFieldsAndProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type, bindingAttr))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFullyQualifiedTypeName(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        binder: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ISerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::ISerializationBinder,
                        >,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 2usize>(
                        "GetFullyQualifiedTypeName",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFullyQualifiedTypeName",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (t, binder))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberInfoFromType(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>, 2usize>(
                        "GetMemberInfoFromType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMemberInfoFromType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo> =
            unsafe { cordl_method_info.invoke_unchecked((), (targetType, memberInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberUnderlyingType(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::MemberInfo,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetMemberUnderlyingType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMemberUnderlyingType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked((), (member))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 2usize>(
                        "GetMemberValue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMemberValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (member, target))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectType(
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetObjectType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetObjectType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked((), (v))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::System::Reflection::BindingFlags,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
                        >,
                    >, 2usize>("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (targetType, bindingAttr))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeName(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        assemblyFormat: crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling,
        binder: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ISerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling,
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::ISerializationBinder,
                        >,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 3usize>(
                        "GetTypeName",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTypeName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (t, assemblyFormat, binder))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasDefaultConstructor(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                        bool,
                        2usize,
                    >("HasDefaultConstructor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasDefaultConstructor", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (t, nonPublic))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImplementsGenericDefinition_ByRefMut1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericInterfaceDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        implementingType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    ), bool, 3usize>("ImplementsGenericDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImplementsGenericDefinition",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (_cordl_type, genericInterfaceDefinition, implementingType),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ImplementsGenericDefinition_Type_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericInterfaceDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), bool, 2usize>("ImplementsGenericDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImplementsGenericDefinition",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, genericInterfaceDefinition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InheritsGenericDefinitionInternal(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericClassDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        implementingType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    ), bool, 3usize>("InheritsGenericDefinitionInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InheritsGenericDefinitionInternal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_cordl_type, genericClassDefinition, implementingType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InheritsGenericDefinition_ByRefMut1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericClassDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        implementingType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    ), bool, 3usize>("InheritsGenericDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InheritsGenericDefinition",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_cordl_type, genericClassDefinition, implementingType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InheritsGenericDefinition_Type_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericClassDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), bool, 2usize>("InheritsGenericDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InheritsGenericDefinition",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, genericClassDefinition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsByRefLikeType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("IsByRefLikeType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsByRefLikeType", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericDefinition(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericInterfaceDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), bool, 2usize>("IsGenericDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsGenericDefinition",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, genericInterfaceDefinition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsIndexedProperty(
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::PropertyInfo,
                        >),
                        bool,
                        1usize,
                    >("IsIndexedProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsIndexedProperty", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (property))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsMethodOverridden(
        currentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodDeclaringType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 3usize>("IsMethodOverridden")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsMethodOverridden",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (currentType, methodDeclaringType, method))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNullable(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("IsNullable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsNullable", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsNullableType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("IsNullableType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsNullableType", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOverridenGenericMember(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        crate::System::Reflection::BindingFlags,
                    ), bool, 2usize>("IsOverridenGenericMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsOverridenGenericMember",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (memberInfo, bindingAttr))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsPublic(
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::PropertyInfo,
                        >),
                        bool,
                        1usize,
                    >("IsPublic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsPublic", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (property))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsVirtual(
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::PropertyInfo,
                        >),
                        bool,
                        1usize,
                    >("IsVirtual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsVirtual", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (propertyInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAssemblyDetails(
        fullyQualifiedTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("RemoveAssemblyDetails")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveAssemblyDetails", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (fullyQualifiedTypeName))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFlag(
        bindingAttr: crate::System::Reflection::BindingFlags,
        flag: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::BindingFlags> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Reflection::BindingFlags,
                        crate::System::Reflection::BindingFlags,
                    ), crate::System::Reflection::BindingFlags, 2usize>(
                        "RemoveFlag"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RemoveFlag",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Reflection::BindingFlags =
            unsafe { cordl_method_info.invoke_unchecked((), (bindingAttr, flag))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetMemberValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMemberValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (member, target, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SplitFullyQualifiedTypeName(
        fullyQualifiedTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                        1usize,
                    >("SplitFullyQualifiedTypeName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SplitFullyQualifiedTypeName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (fullyQualifiedTypeName))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+ReflectionUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
