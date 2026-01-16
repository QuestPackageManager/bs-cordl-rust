#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DebuggableAttribute {
    __cordl_parent: crate::System::Attribute,
    pub m_debuggingModes: crate::System::Diagnostics::DebuggableAttribute_DebuggingModes,
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Diagnostics::DebuggableAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics";
    const CLASS_NAME: &'static str = "DebuggableAttribute";
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
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::DebuggableAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::DebuggableAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl crate::System::Diagnostics::DebuggableAttribute {
    #[cfg(feature = "System+Diagnostics+DebuggableAttribute+DebuggingModes")]
    pub type DebuggingModes = crate::System::Diagnostics::DebuggableAttribute_DebuggingModes;
    pub fn New(
        modes: crate::System::Diagnostics::DebuggableAttribute_DebuggingModes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (modes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        modes: crate::System::Diagnostics::DebuggableAttribute_DebuggingModes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Diagnostics::DebuggableAttribute_DebuggingModes),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (modes))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::DebuggableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute+DebuggingModes")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DebuggableAttribute_DebuggingModes {
    #[default]
    Default = 1i32,
    DisableOptimizations = 256i32,
    EnableEditAndContinue = 4i32,
    IgnoreSymbolStoreSequencePoints = 2i32,
    None = 0i32,
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute+DebuggingModes")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Diagnostics::DebuggableAttribute_DebuggingModes
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Diagnostics";
    const CLASS_NAME: &'static str = "DebuggableAttribute/DebuggingModes";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute+DebuggingModes")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Diagnostics::DebuggableAttribute_DebuggingModes
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute+DebuggingModes")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Diagnostics::DebuggableAttribute_DebuggingModes
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute+DebuggingModes")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Diagnostics::DebuggableAttribute_DebuggingModes
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+DebuggableAttribute+DebuggingModes")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Diagnostics::DebuggableAttribute_DebuggingModes
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
