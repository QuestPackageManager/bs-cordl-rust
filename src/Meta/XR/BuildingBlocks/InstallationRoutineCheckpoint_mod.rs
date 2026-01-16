#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+InstallationRoutineCheckpoint")]
#[repr(C)]
#[derive(Debug)]
pub struct InstallationRoutineCheckpoint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _installationRoutineId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _installationVariants: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::VariantCheckpoint>,
        >,
    >,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+InstallationRoutineCheckpoint")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "InstallationRoutineCheckpoint";
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
#[cfg(feature = "Meta+XR+BuildingBlocks+InstallationRoutineCheckpoint")]
impl std::ops::Deref for crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+InstallationRoutineCheckpoint")]
impl std::ops::DerefMut for crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+InstallationRoutineCheckpoint")]
impl crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint {
    pub fn New(
        installationRoutineId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        installationVariants: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::VariantCheckpoint>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (installationRoutineId, installationVariants))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        installationRoutineId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        installationVariants: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::VariantCheckpoint>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Meta::XR::BuildingBlocks::VariantCheckpoint,
                                >,
                            >,
                        >,
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (installationRoutineId, installationVariants))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InstallationRoutineId(
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
                    >("get_InstallationRoutineId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_InstallationRoutineId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_InstallationVariants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::VariantCheckpoint>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::BuildingBlocks::VariantCheckpoint,
                            >,
                        >,
                    >, 0usize>("get_InstallationVariants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_InstallationVariants",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::VariantCheckpoint>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+InstallationRoutineCheckpoint")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
