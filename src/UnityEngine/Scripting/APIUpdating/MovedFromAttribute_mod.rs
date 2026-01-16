#[cfg(feature = "cordl_class_UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MovedFromAttribute {
    __cordl_parent: crate::System::Attribute,
    pub data: crate::UnityEngine::Scripting::APIUpdating::MovedFromAttributeData,
}
#[cfg(feature = "cordl_class_UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Scripting.APIUpdating";
    const CLASS_NAME: &'static str = "MovedFromAttribute";
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
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl std::ops::Deref for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    pub fn New_Il2CppString1(
        sourceNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sourceNamespace))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Il2CppString_Il2CppString_Il2CppString0(
        autoUpdateAPI: bool,
        sourceNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceAssembly: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceClassName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                autoUpdateAPI,
                sourceNamespace,
                sourceAssembly,
                sourceClassName,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        sourceNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sourceNamespace))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppString_Il2CppString_Il2CppString0(
        &mut self,
        autoUpdateAPI: bool,
        sourceNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceAssembly: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceClassName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        bool,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    autoUpdateAPI,
                    sourceNamespace,
                    sourceAssembly,
                    sourceClassName,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
