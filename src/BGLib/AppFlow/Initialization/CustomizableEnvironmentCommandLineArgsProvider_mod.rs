#[cfg(
    feature = "cordl_class_BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CustomizableEnvironmentCommandLineArgsProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.AppFlow.Initialization";
    const CLASS_NAME: &'static str = "CustomizableEnvironmentCommandLineArgsProvider";
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
#[cfg(feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider")]
impl std::ops::Deref
    for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider")]
impl std::ops::DerefMut
    for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider")]
impl crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider {
    pub fn GetCommandLineArgs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    >, 0usize>("GetCommandLineArgs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCommandLineArgs",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
