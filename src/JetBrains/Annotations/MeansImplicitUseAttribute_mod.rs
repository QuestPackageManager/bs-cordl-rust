#[cfg(feature = "cordl_class_JetBrains+Annotations+MeansImplicitUseAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MeansImplicitUseAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _UseKindFlags_k__BackingField: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    pub _TargetFlags_k__BackingField: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
}
#[cfg(feature = "cordl_class_JetBrains+Annotations+MeansImplicitUseAttribute")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::JetBrains::Annotations::MeansImplicitUseAttribute
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "JetBrains.Annotations";
    const CLASS_NAME: &'static str = "MeansImplicitUseAttribute";
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
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ImplicitUseKindFlags1(
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useKindFlags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ImplicitUseKindFlags_ImplicitUseTargetFlags3(
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useKindFlags, targetFlags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ImplicitUseTargetFlags2(
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (targetFlags))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
    pub fn _ctor_ImplicitUseKindFlags1(
        &mut self,
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::JetBrains::Annotations::ImplicitUseKindFlags),
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (useKindFlags))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ImplicitUseKindFlags_ImplicitUseTargetFlags3(
        &mut self,
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::JetBrains::Annotations::ImplicitUseKindFlags,
                        crate::JetBrains::Annotations::ImplicitUseTargetFlags,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (useKindFlags, targetFlags))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ImplicitUseTargetFlags2(
        &mut self,
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::JetBrains::Annotations::ImplicitUseTargetFlags),
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (targetFlags))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::JetBrains::Annotations::ImplicitUseTargetFlags> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::JetBrains::Annotations::ImplicitUseTargetFlags,
                        0usize,
                    >("get_TargetFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_TargetFlags", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::JetBrains::Annotations::ImplicitUseTargetFlags =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UseKindFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::JetBrains::Annotations::ImplicitUseKindFlags> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::JetBrains::Annotations::ImplicitUseKindFlags, 0usize>(
                        "get_UseKindFlags",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_UseKindFlags",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::JetBrains::Annotations::ImplicitUseKindFlags =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_TargetFlags(
        &mut self,
        value: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::JetBrains::Annotations::ImplicitUseTargetFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_TargetFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_TargetFlags", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_UseKindFlags(
        &mut self,
        value: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::JetBrains::Annotations::ImplicitUseKindFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_UseKindFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_UseKindFlags", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_JetBrains+Annotations+MeansImplicitUseAttribute")]
impl quest_hook::libil2cpp::ObjectType
    for crate::JetBrains::Annotations::MeansImplicitUseAttribute
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
