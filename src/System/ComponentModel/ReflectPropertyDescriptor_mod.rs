#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectPropertyDescriptor {
    __cordl_parent: crate::System::ComponentModel::PropertyDescriptor,
    pub state: crate::System::Collections::Specialized::BitVector32,
    pub componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub ambientValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub propInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    pub getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub shouldSerializeMethod: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::MethodInfo,
    >,
    pub resetMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub realChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventDescriptor,
    >,
    pub receiverType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::ReflectPropertyDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "ReflectPropertyDescriptor";
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
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::ReflectPropertyDescriptor {
    type Target = crate::System::ComponentModel::PropertyDescriptor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl std::ops::DerefMut for crate::System::ComponentModel::ReflectPropertyDescriptor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl crate::System::ComponentModel::ReflectPropertyDescriptor {
    pub fn CanResetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("CanResetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanResetValue", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (component))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderCanResetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::IExtenderProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ExtenderCanResetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderCanResetValue", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (provider, component))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderGetReceiverType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        0usize,
                    >("ExtenderGetReceiverType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderGetReceiverType", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderGetType(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::IExtenderProvider,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("ExtenderGetType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderGetType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, (provider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderGetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::IExtenderProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("ExtenderGetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderGetValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (provider, component))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderResetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        notifyDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::IExtenderProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::PropertyDescriptor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExtenderResetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderResetValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (provider, component, notifyDesc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderSetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        notifyDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::IExtenderProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::PropertyDescriptor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ExtenderSetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderSetValue", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (provider, component, value, notifyDesc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderShouldSerializeValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::ComponentModel::IExtenderProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ExtenderShouldSerializeValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtenderShouldSerializeValue", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (provider, component))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Collections::IList>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FillAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FillAttributes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (attributes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (component))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentClass, name, _cordl_type, attributes))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PropertyInfo_MethodInfo_MethodInfo_Il2CppArray1(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        propInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    componentClass,
                    name,
                    _cordl_type,
                    propInfo,
                    getMethod,
                    setMethod,
                    attrs,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_MethodInfo_MethodInfo_Il2CppArray2(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        receiverType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    componentClass,
                    name,
                    _cordl_type,
                    receiverType,
                    getMethod,
                    setMethod,
                    attrs,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OnValueChanged(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnValueChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnValueChanged", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (component, e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (component))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (component, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerializeValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("ShouldSerializeValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShouldSerializeValue", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (component))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (componentClass, name, _cordl_type, attributes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PropertyInfo_MethodInfo_MethodInfo_Il2CppArray1(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        propInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::PropertyInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MethodInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MethodInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        componentClass,
                        name,
                        _cordl_type,
                        propInfo,
                        getMethod,
                        setMethod,
                        attrs,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_MethodInfo_MethodInfo_Il2CppArray2(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        receiverType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MethodInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MethodInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        componentClass,
                        name,
                        _cordl_type,
                        receiverType,
                        getMethod,
                        setMethod,
                        attrs,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AmbientValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_AmbientValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AmbientValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ComponentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        0usize,
                    >("get_ComponentType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ComponentType", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_DefaultValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DefaultValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                        0usize,
                    >("get_GetMethodValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetMethodValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsExtender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsExtender")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsExtender", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsReadOnly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsReadOnly", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        0usize,
                    >("get_PropertyType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PropertyType", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ResetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                        0usize,
                    >("get_ResetMethodValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ResetMethodValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                        0usize,
                    >("get_SetMethodValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SetMethodValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldSerializeMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                        0usize,
                    >("get_ShouldSerializeMethodValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ShouldSerializeMethodValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectPropertyDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
