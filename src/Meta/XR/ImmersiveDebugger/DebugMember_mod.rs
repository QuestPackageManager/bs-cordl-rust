#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+DebugMember")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugMember {
    __cordl_parent: crate::UnityEngine::Scripting::PreserveAttribute,
    pub GizmoType: crate::Meta::XR::ImmersiveDebugger::DebugGizmoType,
    pub ShowGizmoByDefault: bool,
    pub Color: crate::UnityEngine::Color,
    pub Tweakable: bool,
    pub Min: f32,
    pub Max: f32,
    pub Category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub DisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+DebugMember")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::DebugMember {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "DebugMember";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+DebugMember")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::DebugMember {
    type Target = crate::UnityEngine::Scripting::PreserveAttribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+DebugMember")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::DebugMember {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+DebugMember")]
impl crate::Meta::XR::ImmersiveDebugger::DebugMember {
    pub const DisplayNameTooltip: &'static str = "Optional name override to be used in the Inspector Panel";
    pub fn New_DebugColor0(
        color: crate::Meta::XR::ImmersiveDebugger::DebugColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (color))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        colorString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorString))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DebugColor0(
        &mut self,
        color: crate::Meta::XR::ImmersiveDebugger::DebugColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::ImmersiveDebugger::DebugColor),
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
            cordl_method_info.invoke_unchecked(self, (color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        colorString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
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
            cordl_method_info.invoke_unchecked(self, (colorString))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+DebugMember")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::DebugMember {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
