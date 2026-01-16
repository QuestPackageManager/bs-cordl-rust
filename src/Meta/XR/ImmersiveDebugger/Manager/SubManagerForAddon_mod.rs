#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
#[repr(C)]
#[derive(Debug)]
pub struct SubManagerForAddon {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                >,
            >,
        >,
    >,
    pub _uiPanel: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
    >,
    pub InstanceCache: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
    >,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.Manager";
    const CLASS_NAME: &'static str = "SubManagerForAddon";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
impl std::ops::Deref
for crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
impl std::ops::DerefMut
for crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
impl crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    pub fn GetCountPerType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        i32,
                        1usize,
                    >("GetCountPerType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCountPerType", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ProcessType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTypeFromHierarchy(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
        >,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessTypeFromHierarchy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTypeFromHierarchy", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (item, memberInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTypeFromInspector(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        handle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        memberAttribute: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::DebugMember,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::DebugMember,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ProcessTypeFromInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTypeFromInspector", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_type, handle, memberInfo, memberAttribute),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterSpecialisedWidget(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IMember,
        >,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        memberAttribute: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::DebugMember,
        >,
        handle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::IMember,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::DebugMember,
                            >,
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                        ),
                        bool,
                        4usize,
                    >("RegisterSpecialisedWidget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterSpecialisedWidget", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (member, memberInfo, memberAttribute, handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
        >,
        cache: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (panel, cache))?
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
    pub fn get_TelemetryAnnotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_TelemetryAnnotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_TelemetryAnnotation", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
impl AsRef<crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager>
for crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    fn as_ref(&self) -> &crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+SubManagerForAddon")]
impl AsMut<crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager>
for crate::Meta::XR::ImmersiveDebugger::Manager::SubManagerForAddon {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager {
        unsafe { std::mem::transmute(self) }
    }
}
