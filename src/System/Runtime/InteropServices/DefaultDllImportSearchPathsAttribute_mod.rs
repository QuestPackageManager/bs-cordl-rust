#[cfg(
    feature = "cordl_class_System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultDllImportSearchPathsAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _paths: crate::System::Runtime::InteropServices::DllImportSearchPath,
}
#[cfg(
    feature = "cordl_class_System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "DefaultDllImportSearchPathsAttribute";
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
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    pub fn New(
        paths: crate::System::Runtime::InteropServices::DllImportSearchPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (paths))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        paths: crate::System::Runtime::InteropServices::DllImportSearchPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Runtime::InteropServices::DllImportSearchPath),
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
            cordl_method_info.invoke_unchecked(self, (paths))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
