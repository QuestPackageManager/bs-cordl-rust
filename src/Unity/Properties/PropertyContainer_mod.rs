#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyContainer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Properties::PropertyContainer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "PropertyContainer";
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
#[cfg(feature = "Unity+Properties+PropertyContainer")]
impl std::ops::Deref for crate::Unity::Properties::PropertyContainer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyContainer")]
impl std::ops::DerefMut for crate::Unity::Properties::PropertyContainer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyContainer")]
impl crate::Unity::Properties::PropertyContainer {
    #[cfg(feature = "Unity+Properties+PropertyContainer+GetPropertyVisitor")]
    pub type GetPropertyVisitor = crate::Unity::Properties::PropertyContainer_GetPropertyVisitor;
    #[cfg(feature = "Unity+Properties+PropertyContainer+GetValueVisitor_1")]
    pub type GetValueVisitor_1<TSrcValue: quest_hook::libil2cpp::Type> = crate::Unity::Properties::PropertyContainer_GetValueVisitor_1<
        TSrcValue,
    >;
    pub fn Accept<TContainer>(
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBagVisitor,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        parameters: crate::Unity::Properties::VisitParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IPropertyBagVisitor,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            crate::Unity::Properties::VisitParameters,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Accept",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (visitor, container, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryAccept_ByRefMut_VisitParameters1<TContainer>(
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBagVisitor,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        returnCode: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Properties::VisitReturnCode,
        >,
        parameters: crate::Unity::Properties::VisitParameters,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IPropertyBagVisitor,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::VisitReturnCode,
                            >,
                            crate::Unity::Properties::VisitParameters,
                        ),
                        bool,
                        4usize,
                    >("TryAccept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAccept", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (visitor, container, returnCode, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryAccept_VisitParameters0<TContainer>(
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBagVisitor,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        parameters: crate::Unity::Properties::VisitParameters,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IPropertyBagVisitor,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            crate::Unity::Properties::VisitParameters,
                        ),
                        bool,
                        3usize,
                    >("TryAccept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAccept", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (visitor, container, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetProperty_ByRefMut1<TContainer>(
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        path: quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty>,
        >,
        returnCode: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Properties::VisitReturnCode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::PropertyPath,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Unity::Properties::IProperty,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::VisitReturnCode,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryGetProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetProperty", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (container, path, property, returnCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetProperty_ByRefMut_ByRefMut_ByRefMut0<TContainer>(
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        path: quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::PropertyPath,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Unity::Properties::IProperty,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetProperty", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (container, path, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_ByRefMut_ByRefMut1<TContainer, TValue>(
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        path: quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
        returnCode: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Properties::VisitReturnCode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::PropertyPath,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::VisitReturnCode,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryGetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetValue", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (container, path, value, returnCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_Il2CppString0<TContainer, TValue>(
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                        ),
                        bool,
                        3usize,
                    >("TryGetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetValue", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (container, name, value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Properties::PropertyContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer+GetPropertyVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyContainer_GetPropertyVisitor {
    __cordl_parent: crate::Unity::Properties::PathVisitor,
    pub Property: quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty>,
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer+GetPropertyVisitor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Properties::PropertyContainer_GetPropertyVisitor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "PropertyContainer/GetPropertyVisitor";
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
#[cfg(feature = "Unity+Properties+PropertyContainer+GetPropertyVisitor")]
impl std::ops::Deref for crate::Unity::Properties::PropertyContainer_GetPropertyVisitor {
    type Target = crate::Unity::Properties::PathVisitor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyContainer+GetPropertyVisitor")]
impl std::ops::DerefMut
for crate::Unity::Properties::PropertyContainer_GetPropertyVisitor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyContainer+GetPropertyVisitor")]
impl crate::Unity::Properties::PropertyContainer_GetPropertyVisitor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VisitPath<TContainer, TValue>(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TValue>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::Property_2<TContainer, TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("VisitPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitPath", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (property, container, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer+GetPropertyVisitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::PropertyContainer_GetPropertyVisitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer+GetValueVisitor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyContainer_GetValueVisitor_1<TSrcValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Unity::Properties::PathVisitor,
    pub Value: TSrcValue,
    __cordl_phantom_TSrcValue: std::marker::PhantomData<TSrcValue>,
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer+GetValueVisitor_1")]
unsafe impl<TSrcValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Unity::Properties::PropertyContainer_GetValueVisitor_1<TSrcValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "PropertyContainer/GetValueVisitor`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "PropertyContainer/GetValueVisitor`1",
                    )
                    .unwrap()
                    .make_generic::<(TSrcValue)>()
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
#[cfg(feature = "Unity+Properties+PropertyContainer+GetValueVisitor_1")]
impl<TSrcValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::PropertyContainer_GetValueVisitor_1<TSrcValue> {
    type Target = crate::Unity::Properties::PathVisitor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyContainer+GetValueVisitor_1")]
impl<TSrcValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::PropertyContainer_GetValueVisitor_1<TSrcValue> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyContainer+GetValueVisitor_1")]
impl<
    TSrcValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::PropertyContainer_GetValueVisitor_1<TSrcValue> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSrcValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSrcValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VisitPath<TContainer, TValue>(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TValue>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSrcValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::Property_2<TContainer, TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("VisitPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitPath", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (property, container, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSrcValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyContainer+GetValueVisitor_1")]
impl<TSrcValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::PropertyContainer_GetValueVisitor_1<TSrcValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
