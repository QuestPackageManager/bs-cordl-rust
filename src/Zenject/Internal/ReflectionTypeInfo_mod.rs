#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ReflectionTypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub BaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub InjectProperties: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
            >,
        >,
    >,
    pub InjectFields: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo>,
        >,
    >,
    pub InjectConstructor: quest_hook::libil2cpp::Gc<
        crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
    >,
    pub InjectMethods: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::Internal::ReflectionTypeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionTypeInfo";
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
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo {
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
    pub type InjectConstructorInfo =
        crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
    pub type InjectFieldInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
    pub type InjectMethodInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
    pub type InjectParameterInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
    pub type InjectPropertyInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo;
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
        injectMethods: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
                >,
            >,
        >,
        injectFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
                >,
            >,
        >,
        injectProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                _cordl_type,
                baseType,
                injectConstructor,
                injectMethods,
                injectFields,
                injectProperties,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
        injectMethods: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
                >,
            >,
        >,
        injectFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
                >,
            >,
        >,
        injectProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
                                >,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    _cordl_type,
                    baseType,
                    injectConstructor,
                    injectMethods,
                    injectFields,
                    injectProperties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ReflectionTypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ReflectionTypeInfo_InjectConstructorInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConstructorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    pub Parameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionTypeInfo/InjectConstructorInfo";
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
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    pub fn New(
        constructorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructorInfo, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        constructorInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::ConstructorInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (constructorInfo, parameters))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ReflectionTypeInfo_InjectFieldInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    pub InjectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionTypeInfo/InjectFieldInfo";
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
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    pub fn New(
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fieldInfo, injectableInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fieldInfo, injectableInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ReflectionTypeInfo_InjectMethodInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub MethodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub Parameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionTypeInfo/InjectMethodInfo";
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
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    pub fn New(
        methodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (methodInfo, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        methodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MethodInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (methodInfo, parameters))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ReflectionTypeInfo_InjectParameterInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ParameterInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    pub InjectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionTypeInfo/InjectParameterInfo";
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
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    pub fn New(
        parameterInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameterInfo, injectableInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        parameterInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameterInfo, injectableInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ReflectionTypeInfo_InjectPropertyInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub PropertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    pub InjectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionTypeInfo/InjectPropertyInfo";
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
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    pub fn New(
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyInfo, injectableInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyInfo, injectableInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
