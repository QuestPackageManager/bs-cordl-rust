#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
#[repr(C)]
#[derive(Debug)]
pub struct UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1";
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
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl std::ops::Deref
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl std::ops::DerefMut
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    #[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
    pub type MonoScriptData = crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData;
    pub fn Get() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData,
                0usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Get", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    pub FilePathsData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub TypesData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub TotalTypes: i32,
    pub TotalFiles: i32,
    pub IsEditorOnly: bool,
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1/MonoScriptData";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
impl crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {}
