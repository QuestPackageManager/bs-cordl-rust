#[cfg(feature = "cordl_class_System+Xml+IDtdParserAdapterV1")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdParserAdapterV1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Xml+IDtdParserAdapterV1")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::IDtdParserAdapterV1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "IDtdParserAdapterV1";
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
#[cfg(feature = "cordl_class_System+Xml+IDtdParserAdapterV1")]
impl std::ops::Deref for crate::System::Xml::IDtdParserAdapterV1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Xml+IDtdParserAdapterV1")]
impl std::ops::DerefMut for crate::System::Xml::IDtdParserAdapterV1 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl crate::System::Xml::IDtdParserAdapterV1 {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Namespaces(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_Namespaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Namespaces", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Normalization(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_Normalization")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Normalization", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_V1CompatibilityMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_V1CompatibilityMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_V1CompatibilityMode", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Xml+IDtdParserAdapterV1")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsRef<crate::System::Xml::IDtdParserAdapter>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_ref(&self) -> &crate::System::Xml::IDtdParserAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsMut<crate::System::Xml::IDtdParserAdapter>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdParserAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsRef<crate::System::Xml::IDtdParserAdapterWithValidation>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_ref(&self) -> &crate::System::Xml::IDtdParserAdapterWithValidation {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsMut<crate::System::Xml::IDtdParserAdapterWithValidation>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdParserAdapterWithValidation {
        unsafe { std::mem::transmute(self) }
    }
}
