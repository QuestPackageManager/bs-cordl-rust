#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct GPUInstanceDataBufferGrower {
    pub m_SrcBuffer:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::GPUInstanceDataBuffer>,
    pub m_DstBuffer:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::GPUInstanceDataBuffer>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferGrower";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower {
    #[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs")]
    pub type CopyInstancesKernelIDs =
        crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_CopyInstancesKernelIDs;
    #[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
    pub type GPUResources = crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources;
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitToGpu(
        &mut self,
        gpuResources: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::GPUInstanceDataBuffer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                        1usize,
                    >("SubmitToGpu")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubmitToGpu", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (gpuResources))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sourceBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
        instanceNumInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceNumInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::InstanceNumInfo,
                        >,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (sourceBuffer, instanceNumInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs"
)]
#[repr(C)]
#[derive(Debug)]
pub struct GPUInstanceDataBufferGrower_CopyInstancesKernelIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_CopyInstancesKernelIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferGrower/CopyInstancesKernelIDs";
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
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_CopyInstancesKernelIDs
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_CopyInstancesKernelIDs
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_CopyInstancesKernelIDs {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+CopyInstancesKernelIDs"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_CopyInstancesKernelIDs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct GPUInstanceDataBufferGrower_GPUResources {
    pub cs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub kernelId: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferGrower/GPUResources";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources {
    pub fn CreateResources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CreateResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateResources",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadShaders(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                    >), quest_hook::libil2cpp::Void, 1usize>("LoadShaders")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadShaders",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resources))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferGrower+GPUResources")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
