#[cfg(feature = "cordl_class_UnityEngine+Rendering+GenerateHLSL")]
#[repr(C)]
#[derive(Debug)]
pub struct GenerateHLSL {
    __cordl_parent: crate::System::Attribute,
    pub packingRules: crate::UnityEngine::Rendering::PackingRules,
    pub containsPackedFields: bool,
    pub needAccessors: bool,
    pub needSetters: bool,
    pub needParamDebug: bool,
    pub paramDefinesStart: i32,
    pub omitStructDeclaration: bool,
    pub generateCBuffer: bool,
    pub constantRegister: i32,
    pub sourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GenerateHLSL")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::GenerateHLSL {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GenerateHLSL";
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
#[cfg(feature = "UnityEngine+Rendering+GenerateHLSL")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GenerateHLSL {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GenerateHLSL")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GenerateHLSL {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GenerateHLSL")]
impl crate::UnityEngine::Rendering::GenerateHLSL {
    pub fn New(
        rules: crate::UnityEngine::Rendering::PackingRules,
        needAccessors: bool,
        needSetters: bool,
        needParamDebug: bool,
        paramDefinesStart: i32,
        omitStructDeclaration: bool,
        containsPackedFields: bool,
        generateCBuffer: bool,
        constantRegister: i32,
        sourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    rules,
                    needAccessors,
                    needSetters,
                    needParamDebug,
                    paramDefinesStart,
                    omitStructDeclaration,
                    containsPackedFields,
                    generateCBuffer,
                    constantRegister,
                    sourcePath,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        rules: crate::UnityEngine::Rendering::PackingRules,
        needAccessors: bool,
        needSetters: bool,
        needParamDebug: bool,
        paramDefinesStart: i32,
        omitStructDeclaration: bool,
        containsPackedFields: bool,
        generateCBuffer: bool,
        constantRegister: i32,
        sourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::PackingRules,
                            bool,
                            bool,
                            bool,
                            i32,
                            bool,
                            bool,
                            bool,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        rules,
                        needAccessors,
                        needSetters,
                        needParamDebug,
                        paramDefinesStart,
                        omitStructDeclaration,
                        containsPackedFields,
                        generateCBuffer,
                        constantRegister,
                        sourcePath,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GenerateHLSL")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::GenerateHLSL {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
