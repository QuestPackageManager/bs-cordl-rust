#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugData_RenderGraph_BufferResourceData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub count: i32,
    pub stride: i32,
    pub target: crate::UnityEngine::GraphicsBuffer_Target,
    pub usage: crate::UnityEngine::GraphicsBuffer_UsageFlags,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/BufferResourceData";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData")]
impl crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct DebugData_RenderGraph_PassData {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPassType,
    pub resourceReadLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub resourceWriteLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub culled: bool,
    pub _cordl_async: bool,
    pub nativeSubPassIndex: i32,
    pub syncToPassIndex: i32,
    pub syncFromPassIndex: i32,
    pub generateDebugData: bool,
    pub nrpInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo,
    >,
    pub scriptInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/PassData";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData")]
impl crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData {
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo"
    )]
    pub type NRPInfo =
        crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo;
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugData_RenderGraph_PassScriptInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub line: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/PassScriptInfo";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo")]
impl crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct DebugData_RenderGraph_ResourceData {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub imported: bool,
    pub creationPassIndex: i32,
    pub releasePassIndex: i32,
    pub consumerList: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub producerList: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub memoryless: bool,
    pub textureData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData,
    >,
    pub bufferData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/ResourceData";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData")]
impl crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugData_RenderGraph_TextureResourceData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub bindMS: bool,
    pub samples: i32,
    pub format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub clearBuffer: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/TextureResourceData";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
)]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
)]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub passBreakReasoning: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub attachmentInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo,
            >,
        >,
    >,
    pub passCompatibility: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo,
        >,
    >,
    pub mergedPassIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/PassData/NRPInfo/NativeRenderPassInfo";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo {
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
    )]
    pub type AttachmentInfo = crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
    )]
    pub type PassCompatibilityInfo = crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub resourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub loadReason: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub storeReason: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub storeMsaaReason: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub attachmentIndex: i32,
    pub attachment: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassAttachment,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/PassData/NRPInfo/NativeRenderPassInfo/AttachmentInfo";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+AttachmentInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub isCompatible: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/PassData/NRPInfo/NativeRenderPassInfo/PassCompatibilityInfo";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo+PassCompatibilityInfo"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_PassCompatibilityInfo {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PassData_DebugData_RenderGraph_NRPInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nativePassInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo,
    >,
    pub textureFBFetchList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub setGlobals: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub width: i32,
    pub height: i32,
    pub volumeDepth: i32,
    pub samples: i32,
    pub hasDepth: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData/PassData/NRPInfo";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo")]
impl crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo {
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo+NativeRenderPassInfo"
    )]
    pub type NativeRenderPassInfo = crate::UnityEngine::Rendering::RenderGraphModule::NRPInfo_PassData_DebugData_RenderGraph_NativeRenderPassInfo;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData+NRPInfo"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::PassData_DebugData_RenderGraph_NRPInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraph {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nativeCompiler: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler,
    >,
    pub _nativeRenderPassesEnabled_k__BackingField: bool,
    pub m_Resources: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
    >,
    pub m_RenderGraphPool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphObjectPool,
    >,
    pub m_builderInstance: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilders,
    >,
    pub m_RenderPasses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
            >,
        >,
    >,
    pub m_RendererLists: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    >,
    pub m_DebugParameters: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDebugParams,
    >,
    pub m_FrameInformationLogger: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphLogger,
    >,
    pub m_DefaultResources: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDefaultResources,
    >,
    pub m_DefaultProfilingSamplers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        >,
    >,
    pub m_RenderGraphContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
    >,
    pub m_PreviousCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub m_ImmediateModeResourceList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub m_CompilationCache: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RenderGraphCompilationCache,
    >,
    pub m_TempMRTArrays: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                >,
            >,
        >,
    >,
    pub m_CullingStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<i32>,
    >,
    pub m_CurrentExecutionName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_ExecutionCount: i32,
    pub m_CurrentFrameIndex: i32,
    pub m_CurrentImmediatePassIndex: i32,
    pub m_ExecutionExceptionWasRaised: bool,
    pub m_HasRenderGraphBegun: bool,
    pub m_RendererListCulling: bool,
    pub m_EnableCompilationCaching: bool,
    pub m_DefaultCompiledGraph: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
    >,
    pub m_CurrentCompiledGraph: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
    >,
    pub m_CaptureDebugDataForExecution: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_DebugData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
            >,
        >,
    >,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub registeredGlobals: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    >,
    pub k_PassNameDebugIgnoreList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph {
    pub const k_BeginProfilingSamplerPassName: &'static str = "BeginProfile";
    pub const k_EndProfilingSamplerPassName: &'static str = "EndProfile";
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
    pub type CompiledGraph =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
    pub type CompiledPassInfo =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo")]
    pub type CompiledResourceInfo =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
    pub type DebugData = crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
    type ICompiledGraph =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
    )]
    pub type OnExecutionRegisteredDelegate =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate"
    )]
    pub type OnGraphRegisteredDelegate =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData")]
    pub type ProfilingScopePassData =
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ProfilingScopePassData;
    pub fn AddComputePass_Il2CppString_i32_0<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder,
        >,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<PassData>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder,
                        >,
                        4usize,
                    >("AddComputePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddComputePass", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (passName, passData, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddComputePass_ProfilingSampler_Il2CppString_i32_1<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder,
        >,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<PassData>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ProfilingSampler,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder,
                        >,
                        5usize,
                    >("AddComputePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddComputePass", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IComputeRenderGraphBuilder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (passName, passData, sampler, file, line))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddPassDebugMetadata(
        &mut self,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "AddPassDebugMetadata"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddPassDebugMetadata",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderPass, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddRasterRenderPass_Il2CppString_i32_0<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder,
        >,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<PassData>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder,
                    >, 4usize>("AddRasterRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddRasterRenderPass",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (passName, passData, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddRasterRenderPass_ProfilingSampler_Il2CppString_i32_1<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder,
        >,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<PassData>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder,
                    >, 5usize>("AddRasterRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddRasterRenderPass",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (passName, passData, sampler, file, line))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRenderPass_Il2CppString_i32_1<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<PassData>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder,
                        4usize,
                    >("AddRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddRenderPass", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder =
            unsafe { cordl_method_info.invoke_unchecked(self, (passName, passData, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddRenderPass_ProfilingSampler_Il2CppString_i32_0<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<PassData>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ProfilingSampler,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder,
                        5usize,
                    >("AddRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddRenderPass", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphBuilder = unsafe {
            cordl_method_info.invoke_unchecked(self, (passName, passData, sampler, file, line))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddUnsafePass_Il2CppString_i32_0<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IUnsafeRenderGraphBuilder,
        >,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<PassData>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::IUnsafeRenderGraphBuilder,
                    >, 4usize>("AddUnsafePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddUnsafePass",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IUnsafeRenderGraphBuilder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (passName, passData, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddUnsafePass_ProfilingSampler_Il2CppString_i32_1<PassData>(
        &mut self,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passData: quest_hook::libil2cpp::ByRefMut<PassData>,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IUnsafeRenderGraphBuilder,
        >,
    >
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<PassData>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::IUnsafeRenderGraphBuilder,
                    >, 5usize>("AddUnsafePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddUnsafePass",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::IUnsafeRenderGraphBuilder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (passName, passData, sampler, file, line))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllGlobals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2_ValueCollection<
                i32,
                crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2_ValueCollection<
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    >, 0usize>("AllGlobals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllGlobals",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2_ValueCollection<
                i32,
                crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn AllocateCulledPassResources(
        &mut self,
        passInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AllocateCulledPassResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateCulledPassResources", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn AreRendererListsEmpty(
        &mut self,
        rendererLists: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        >,
                    >), bool, 1usize>("AreRendererListsEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AreRendererListsEmpty",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererLists))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginProfilingSampler(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "BeginProfilingSampler"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginProfilingSampler",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginRecording(
        &mut self,
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphParameters,
                    >), quest_hook::libil2cpp::Void, 1usize>("BeginRecording")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginRecording",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CleanupDebugData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CleanupDebugData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CleanupDebugData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearCompiledGraph_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearCompiledGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearCompiledGraph",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearCompiledGraph_RenderGraph_CompiledGraph__cordl_bool1(
        &mut self,
        compiledGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
        >,
        useCompilationCaching: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ClearCompiledGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearCompiledGraph", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (compiledGraph, useCompilationCaching))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearGlobalBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearGlobalBindings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearGlobalBindings",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearPassDebugMetadata(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ClearPassDebugMetadata",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearPassDebugMetadata",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderPasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearRenderPasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderPasses",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CompileNativeRenderGraph(
        &mut self,
        graphHash: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler,
                        >,
                        1usize,
                    >("CompileNativeRenderGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompileNativeRenderGraph", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (graphHash))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompilePassImmediatly(
        &mut self,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                        >),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                        >,
                        1usize,
                    >("CompilePassImmediatly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompilePassImmediatly", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompileRenderGraph(
        &mut self,
        graphHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("CompileRenderGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompileRenderGraph",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (graphHash))? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeGraphHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("ComputeGraphHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeGraphHash",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CountReferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CountReferences")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CountReferences",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBuffer_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "CreateBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBuffer_ByRefMut1(
        &mut self,
        graphicsBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "CreateBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (graphicsBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGizmoRendererList(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
        gizmoSubset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GizmoSubset>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GizmoSubset,
                            >,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        2usize,
                    >("CreateGizmoRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateGizmoRendererList", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, gizmoSubset))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererList_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RendererUtils::RendererListDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererUtils::RendererListDesc,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("CreateRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererList_ByRefMut1(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererListParams>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererListParams,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("CreateRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererLists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CreateRendererLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererLists",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShadowRendererList(
        &mut self,
        shadowDrawingSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowDrawingSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ShadowDrawingSettings,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("CreateShadowRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShadowRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (shadowDrawingSettings))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSharedTexture(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
        explicitRelease: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        >,
                        bool,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 2usize>(
                        "CreateSharedTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateSharedTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc, explicitRelease))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_ByRefMut0(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("CreateSkyboxRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSkyboxRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Matrix4x4_Matrix4x4_1(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        viewMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        3usize,
                    >("CreateSkyboxRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSkyboxRendererList", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (camera, projectionMatrix, viewMatrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Matrix4x4_Matrix4x4_Matrix4x4_Matrix4x4_2(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
        projectionMatrixL: crate::UnityEngine::Matrix4x4,
        viewMatrixL: crate::UnityEngine::Matrix4x4,
        projectionMatrixR: crate::UnityEngine::Matrix4x4,
        viewMatrixR: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        5usize,
                    >("CreateSkyboxRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSkyboxRendererList", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    camera,
                    projectionMatrixL,
                    viewMatrixL,
                    projectionMatrixR,
                    viewMatrixR,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextureIfInvalid(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
        texture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "CreateTextureIfInvalid"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTextureIfInvalid",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc, texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTexture_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "CreateTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTexture_TextureHandle1(
        &mut self,
        texture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle),
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        1usize,
                    >("CreateTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIOverlayRendererList_ByRefMut0(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("CreateUIOverlayRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateUIOverlayRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIOverlayRendererList_ByRefMut1(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
        uiSubset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::UISubset>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::UISubset,
                            >,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        2usize,
                    >("CreateUIOverlayRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateUIOverlayRendererList", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, uiSubset))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWireOverlayRendererList(
        &mut self,
        camera: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        1usize,
                    >("CreateWireOverlayRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateWireOverlayRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CullRendererLists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CullRendererLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CullRendererLists",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CullUnusedPasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CullUnusedPasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CullUnusedPasses",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EndFrame(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EndFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndFrame",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EndProfilingSampler(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "EndProfilingSampler"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndProfilingSampler",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler, file, line))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndRecordingAndExecute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "EndRecordingAndExecute",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndRecordingAndExecute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCompiledPass(
        &mut self,
        passInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ExecuteCompiledPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteCompiledPass", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteNativeRenderGraph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ExecuteNativeRenderGraph",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteNativeRenderGraph",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecutePassImmediately(
        &mut self,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ExecutePassImmediately"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecutePassImmediately",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteRenderGraph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ExecuteRenderGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteRenderGraph",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FindTextureProducer(
        &mut self,
        consumerPass: i32,
        info: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        i32,
                        3usize,
                    >("FindTextureProducer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindTextureProducer", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (consumerPass, info, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCompilerDebugData(
        &mut self,
        debugData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "GenerateCompilerDebugData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateCompilerDebugData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (debugData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDebugData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("GenerateDebugData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateDebugData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBufferDesc(
        &mut self,
        graphicsBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc, 1usize>(
                        "GetBufferDesc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBufferDesc",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc =
            unsafe { cordl_method_info.invoke_unchecked(self, (graphicsBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompiledPassInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DynamicArray_1<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                            >,
                        >,
                        0usize,
                    >("GetCompiledPassInfos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCompiledPassInfos", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDebugData(
        &mut self,
        executionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
                        >,
                        1usize,
                    >("GetDebugData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDebugData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (executionName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultProfilingSampler(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProfilingSampler,
                        >,
                        1usize,
                    >("GetDefaultProfilingSampler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultProfilingSampler", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProfilingSampler,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstValidConsumerIndex(
        &mut self,
        passIndex: i32,
        info: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                            >,
                        ),
                        i32,
                        2usize,
                    >("GetFirstValidConsumerIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFirstValidConsumerIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (passIndex, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstValidWriteIndex(
        &mut self,
        info: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                        >),
                        i32,
                        1usize,
                    >("GetFirstValidWriteIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFirstValidWriteIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobal(
        &mut self,
        globalPropertyId: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        1usize,
                    >("GetGlobal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGlobal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (globalPropertyId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetImportedFallback(
        &mut self,
        desc: crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        fallback: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    ), bool, 2usize>("GetImportedFallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetImportedFallback",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc, fallback))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLatestProducerIndex(
        &mut self,
        passIndex: i32,
        info: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                            >,
                        ),
                        i32,
                        2usize,
                    >("GetLatestProducerIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLatestProducerIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (passIndex, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLatestValidReadIndex(
        &mut self,
        info: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                        >),
                        i32,
                        1usize,
                    >("GetLatestValidReadIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLatestValidReadIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLatestValidWriteIndex(
        &mut self,
        info: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                        >),
                        i32,
                        1usize,
                    >("GetLatestValidWriteIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLatestValidWriteIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRegisteredRenderGraphs() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                        >,
                    >, 0usize>("GetRegisteredRenderGraphs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRegisteredRenderGraphs",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTargetInfo(
        &mut self,
        texture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle),
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
                        1usize,
                    >("GetRenderTargetInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderTargetInfo", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureDesc(
        &mut self,
        texture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle),
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        1usize,
                    >("GetTextureDesc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureDesc", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetWidgetList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
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
                                crate::UnityEngine::Rendering::DebugUI_Widget,
                            >,
                        >,
                    >, 0usize>("GetWidgetList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetWidgetList",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportBackbuffer_RenderTargetIdentifier1(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        1usize,
                    >("ImportBackbuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ImportBackbuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportBackbuffer_RenderTargetInfo_ImportResourceParams0(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        info: crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
        importParams: crate::UnityEngine::Rendering::RenderGraphModule::ImportResourceParams,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
                        crate::UnityEngine::Rendering::RenderGraphModule::ImportResourceParams,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 3usize>(
                        "ImportBackbuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImportBackbuffer",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, info, importParams))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportBuffer(
        &mut self,
        graphicsBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        forceRelease: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        bool,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 2usize>(
                        "ImportBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImportBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (graphicsBuffer, forceRelease))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportRayTracingAccelerationStructure(
        &mut self,
        accelStruct: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
            >,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle,
                        2usize,
                    >("ImportRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ImportRayTracingAccelerationStructure", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::RayTracingAccelerationStructureHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (accelStruct, name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ImportTexture_ImportResourceParams1(
        &mut self,
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        importParams: crate::UnityEngine::Rendering::RenderGraphModule::ImportResourceParams,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderGraphModule::ImportResourceParams,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 2usize>(
                        "ImportTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImportTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, importParams))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportTexture_RTHandle0(
        &mut self,
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >),
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        1usize,
                    >("ImportTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ImportTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportTexture_RenderTargetInfo_ImportResourceParams2(
        &mut self,
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        info: crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
        importParams: crate::UnityEngine::Rendering::RenderGraphModule::ImportResourceParams,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
                        crate::UnityEngine::Rendering::RenderGraphModule::ImportResourceParams,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 3usize>(
                        "ImportTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImportTexture",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, info, importParams))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportTexture__cordl_bool3(
        &mut self,
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        isBuiltin: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        bool,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 2usize>(
                        "ImportTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ImportTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, isBuiltin))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvalidateContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InvalidateContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvalidateContext",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsGlobal(&mut self, globalPropertyId: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("IsGlobal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsGlobal",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (globalPropertyId))? };
        Ok(__cordl_ret.into())
    }
    pub fn LogCulledPasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LogCulledPasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LogCulledPasses",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LogFrameInformation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LogFrameInformation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LogFrameInformation",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LogRenderPassBegin(
        &mut self,
        passInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("LogRenderPassBegin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LogRenderPassBegin", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn LogRendererListsCreation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "LogRendererListsCreation",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LogRendererListsCreation",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object.into())
    }
    pub fn OnPassAdded(
        &mut self,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                    >), quest_hook::libil2cpp::Void, 1usize>("OnPassAdded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnPassAdded",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn PostRenderPassExecute(
        &mut self,
        passInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("PostRenderPassExecute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PostRenderPassExecute", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passInfo, pass, rgContext))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreRenderPassExecute(
        &mut self,
        passInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("PreRenderPassExecute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreRenderPassExecute", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passInfo, pass, rgContext))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreRenderPassSetRenderTargets(
        &mut self,
        passInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("PreRenderPassSetRenderTargets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreRenderPassSetRenderTargets", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passInfo, pass, rgContext))? };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshSharedTextureDesc(
        &mut self,
        handle: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "RefreshSharedTextureDesc"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RefreshSharedTextureDesc",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle, desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDebug(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Panel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Panel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RegisterDebug")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterDebug", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseImmediateModeResources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ReleaseImmediateModeResources",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReleaseImmediateModeResources",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseSharedTexture(
        &mut self,
        texture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReleaseSharedTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseSharedTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestCaptureDebugData(
        &mut self,
        executionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                    >("RequestCaptureDebugData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RequestCaptureDebugData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (executionName))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobal(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        globalPropertyId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (h, globalPropertyId))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryCullPassAtIndex(
        &mut self,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("TryCullPassAtIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryCullPassAtIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterDebug(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UnRegisterDebug")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnRegisterDebug",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAllSharedResourceLastFrameIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UpdateAllSharedResourceLastFrameIndex",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateAllSharedResourceLastFrameIndex",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCurrentCompiledGraph(
        &mut self,
        graphHash: i32,
        forceNoCaching: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, bool), bool, 2usize>("UpdateCurrentCompiledGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateCurrentCompiledGraph",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (graphHash, forceNoCaching))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateImportedResourceLifeTime(
        &mut self,
        data: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData,
        >,
        passList: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UpdateImportedResourceLifeTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateImportedResourceLifeTime", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data, passList))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePassSynchronization(
        &mut self,
        currentPassInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
        producerPassInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
        currentPassIndex: i32,
        lastProducer: i32,
        intLastSyncIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("UpdatePassSynchronization")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdatePassSynchronization", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    currentPassInfo,
                    producerPassInfo,
                    currentPassIndex,
                    lastProducer,
                    intLastSyncIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateResourceAllocationAndSynchronization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UpdateResourceAllocationAndSynchronization",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateResourceAllocationAndSynchronization",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateResourceSynchronization(
        &mut self,
        lastGraphicsPipeSync: quest_hook::libil2cpp::ByRefMut<i32>,
        lastComputePipeSync: quest_hook::libil2cpp::ByRefMut<i32>,
        currentPassIndex: i32,
        resource: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("UpdateResourceSynchronization")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateResourceSynchronization", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    lastGraphicsPipeSync,
                    lastComputePipeSync,
                    currentPassIndex,
                    resource,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onDebugDataCaptured(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_onDebugDataCaptured")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_onDebugDataCaptured", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onExecutionRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_onExecutionRegistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_onExecutionRegistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onExecutionUnregistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_onExecutionUnregistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_onExecutionUnregistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onGraphRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_onGraphRegistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_onGraphRegistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_onGraphUnregistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_onGraphUnregistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_onGraphUnregistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_areAnySettingsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_areAnySettingsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_areAnySettingsActive",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_debugParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDebugParams,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDebugParams,
                    >, 0usize>("get_debugParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_debugParams",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDebugParams,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultResources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDefaultResources,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDefaultResources,
                        >,
                        0usize,
                    >("get_defaultResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_defaultResources", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphDefaultResources,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_enableValidityChecks() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_enableValidityChecks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_enableValidityChecks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isRenderGraphViewerActive() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_isRenderGraphViewerActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isRenderGraphViewerActive",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
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
                    >("get_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_name", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_nativeRenderPassesEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_nativeRenderPassesEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_nativeRenderPassesEnabled",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDebugDataCaptured(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_onDebugDataCaptured")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_onDebugDataCaptured", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onExecutionRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_onExecutionRegistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_onExecutionRegistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onExecutionUnregistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_onExecutionUnregistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_onExecutionUnregistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onGraphRegistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_onGraphRegistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_onGraphRegistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onGraphUnregistered(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_onGraphUnregistered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_onGraphUnregistered", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_enableValidityChecks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_enableValidityChecks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_enableValidityChecks",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isRenderGraphViewerActive(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_isRenderGraphViewerActive",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isRenderGraphViewerActive",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                    >("set_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_name", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_nativeRenderPassesEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_nativeRenderPassesEnabled",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_nativeRenderPassesEnabled",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraph_CompiledGraph {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub compiledResourcesInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::DynamicArray_1<
                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                >,
            >,
        >,
    >,
    pub compiledPassInfos: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::DynamicArray_1<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo,
        >,
    >,
    pub lastExecutionFrame: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/CompiledGraph";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InitResourceInfosData(
        &mut self,
        resourceInfos: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DynamicArray_1<
                                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InitResourceInfosData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitResourceInfosData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resourceInfos, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeCompilationData(
        &mut self,
        passes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                >,
            >,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InitializeCompilationData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeCompilationData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (passes, resources))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
impl AsRef<crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph>
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph
{
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledGraph")]
impl AsMut<crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph>
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledGraph
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RenderGraph_CompiledPassInfo {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub index: i32,
    pub resourceCreateList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub resourceReleaseList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub fence: crate::UnityEngine::Rendering::GraphicsFence,
    pub refCount: i32,
    pub syncToPassIndex: i32,
    pub syncFromPassIndex: i32,
    pub enableAsyncCompute: bool,
    pub allowPassCulling: bool,
    pub needGraphicsFence: bool,
    pub culled: bool,
    pub culledByRendererList: bool,
    pub hasSideEffect: bool,
    pub enableFoveatedRasterization: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/CompiledPassInfo";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledPassInfo")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledPassInfo {
    pub fn Reset(
        &mut self,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass, index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RenderGraph_CompiledResourceInfo {
    pub producers: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub consumers: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub refCount: i32,
    pub imported: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/CompiledResourceInfo";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+CompiledResourceInfo")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_CompiledResourceInfo {
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraph_DebugData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub passList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData,
        >,
    >,
    pub resourceLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData,
                >,
            >,
        >,
    >,
    pub isNRPCompiler: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/DebugData";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData {
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+BufferResourceData"
    )]
    pub type BufferResourceData =
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_BufferResourceData;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassData")]
    pub type PassData =
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassData;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+PassScriptInfo")]
    pub type PassScriptInfo =
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_PassScriptInfo;
    #[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+ResourceData")]
    pub type ResourceData =
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_ResourceData;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData+TextureResourceData"
    )]
    pub type TextureResourceData =
        crate::UnityEngine::Rendering::RenderGraphModule::DebugData_RenderGraph_TextureResourceData;
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+DebugData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
#[derive(Debug)]
#[repr(C)]
pub struct RenderGraph_ICompiledGraph {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/ICompiledGraph";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+ICompiledGraph")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ICompiledGraph
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraph_OnExecutionRegisteredDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/OnExecutionRegisteredDelegate";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
)]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate
{
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
)]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate {
    pub fn BeginInvoke(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        executionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, 4usize>(
                        "BeginInvoke",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginInvoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (graph, executionName, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        executionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (graph, executionName))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnExecutionRegisteredDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnExecutionRegisteredDelegate
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraph_OnGraphRegisteredDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/OnGraphRegisteredDelegate";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate
{
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate {
    pub fn BeginInvoke(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, 3usize>(
                        "BeginInvoke",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginInvoke",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> =
            unsafe { cordl_method_info.invoke_unchecked(self, (graph, callback, object))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), quest_hook::libil2cpp::Void, 1usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (graph))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+OnGraphRegisteredDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_OnGraphRegisteredDelegate
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraph_ProfilingScopePassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ProfilingScopePassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "RenderGraph/ProfilingScopePassData";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ProfilingScopePassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ProfilingScopePassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData")]
impl crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ProfilingScopePassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+RenderGraph+ProfilingScopePassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_ProfilingScopePassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
