#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompilerService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::LowLevel::BurstCompilerService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.LowLevel";
    const CLASS_NAME: &'static str = "BurstCompilerService";
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
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl std::ops::Deref for crate::Unity::Burst::LowLevel::BurstCompilerService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl std::ops::DerefMut for crate::Unity::Burst::LowLevel::BurstCompilerService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl crate::Unity::Burst::LowLevel::BurstCompilerService {
    #[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
    pub type BurstLogType = crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType;
    pub fn CompileAsyncDelegateMethod(
        delegateMethod: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        compilerOptions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileAsyncDelegateMethod", (delegateMethod, compilerOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAsyncCompiledAsyncDelegateMethod(
        userID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAsyncCompiledAsyncDelegateMethod", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentExecutionMode() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentExecutionMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisassembly(
        m: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        compilerOptions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDisassembly", (m, compilerOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateSharedMemory(
        key: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Hash128>,
        size_of: u32,
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreateSharedMemory", (key, size_of, alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBurstLibrary(
        fullPathToLibBurstGenerated: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBurstLibrary", (fullPathToLibBurstGenerated))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        logType: crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        lineNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (userData, logType, message, filename, lineNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn RuntimeLog(
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        logType: crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        lineNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RuntimeLog", (userData, logType, message, filename, lineNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurrentExecutionMode(
        environment: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCurrentExecutionMode", (environment))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::LowLevel::BurstCompilerService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BurstCompilerService_BurstLogType {
    #[default]
    Error = 2i32,
    Info = 0i32,
    Warning = 1i32,
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.LowLevel";
    const CLASS_NAME: &'static str = "BurstCompilerService/BurstLogType";
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
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType {
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
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType {
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
#[cfg(feature = "Unity+Burst+LowLevel+BurstCompilerService+BurstLogType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::LowLevel::BurstCompilerService_BurstLogType {
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
