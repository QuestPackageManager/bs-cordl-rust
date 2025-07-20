#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegerExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::DotnetExtension::IntegerExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.DotnetExtension";
    const CLASS_NAME: &'static str = "IntegerExtensions";
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
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl std::ops::Deref for crate::BGLib::DotnetExtension::IntegerExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl std::ops::DerefMut for crate::BGLib::DotnetExtension::IntegerExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl crate::BGLib::DotnetExtension::IntegerExtensions {
    pub fn ToUInt(
        number: i32,
        uNumber: quest_hook::libil2cpp::ByRefMut<u32>,
        isNegative: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ToUInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToUInt",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (number, uNumber, isNegative))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+DotnetExtension+IntegerExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::DotnetExtension::IntegerExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
