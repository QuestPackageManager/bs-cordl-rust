#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+IDebugUIPanel")]
#[repr(C)]
#[derive(Debug)]
pub struct IDebugUIPanel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+IDebugUIPanel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.UserInterface";
    const CLASS_NAME: &'static str = "IDebugUIPanel";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+IDebugUIPanel")]
impl std::ops::Deref
for crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+IDebugUIPanel")]
impl std::ops::DerefMut
for crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+IDebugUIPanel")]
impl crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
    pub fn GetInspector(
        &mut self,
        instance: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
                        >,
                        2usize,
                    >("GetInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInspector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (instance, category))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInspector(
        &mut self,
        instance: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
                        >,
                        2usize,
                    >("RegisterInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterInspector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (instance, category))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterInspector(
        &mut self,
        instance: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
        allCategories: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("UnregisterInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnregisterInspector", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instance, category, allCategories))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+IDebugUIPanel")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
