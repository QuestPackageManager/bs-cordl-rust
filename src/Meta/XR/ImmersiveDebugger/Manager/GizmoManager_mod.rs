#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GizmoManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub GizmosDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::System::ValueTuple_2<
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Gizmo::GizmoRendererManager,
                        >,
                    >,
                >,
            >,
        >,
    >,
    pub _uiPanel:
        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel>,
    pub _instanceCache:
        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache>,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.Manager";
    const CLASS_NAME: &'static str = "GizmoManager";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
impl crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager {
    pub fn AddGizmo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        gizmoAttribute: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
        instanceCache: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
        >,
        gizmoRendererManager: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Meta::XR::ImmersiveDebugger::Gizmo::GizmoRendererManager,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Gizmo::GizmoRendererManager,
                            >,
                        >,
                    ), bool, 5usize>("AddGizmo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddGizmo",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _cordl_type,
                    member,
                    gizmoAttribute,
                    instanceCache,
                    gizmoRendererManager,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCountPerType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<crate::System::Type>), i32, 1usize>(
                        "GetCountPerType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCountPerType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTypeFromHierarchy(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item>,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ProcessTypeFromHierarchy"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessTypeFromHierarchy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (item, memberInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTypeFromInspector(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        handle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        memberAttribute: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::DebugMember>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessTypeFromInspector"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessTypeFromInspector",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (_cordl_type, handle, memberInfo, memberAttribute))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveGizmosForType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveGizmosForType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveGizmosForType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
        >,
        cache: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (panel, cache))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TelemetryAnnotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
impl AsRef<crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager>
    for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager
{
    fn as_ref(&self) -> &crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+GizmoManager")]
impl AsMut<crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager>
    for crate::Meta::XR::ImmersiveDebugger::Manager::GizmoManager
{
    fn as_mut(&mut self) -> &mut crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager {
        unsafe { std::mem::transmute(self) }
    }
}
