#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct GeometryHolder {
    pub m_Type: i32,
    pub m_DataStart: u32,
    pub m_FakePointer0: crate::System::IntPtr,
    pub m_FakePointer1: crate::System::IntPtr,
    pub m_Blob: crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer,
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::LowLevelPhysics::GeometryHolder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.LowLevelPhysics";
    const CLASS_NAME: &'static str = "GeometryHolder";
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::LowLevelPhysics::GeometryHolder {
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LowLevelPhysics+GeometryHolder")]
impl crate::UnityEngine::LowLevelPhysics::GeometryHolder {
    #[cfg(feature = "UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
    pub type _m_Blob_e__FixedBuffer =
        crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer;
    pub fn As<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), T, 0usize>("As")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "As",
                            0usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct GeometryHolder__m_Blob_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.LowLevelPhysics";
    const CLASS_NAME: &'static str = "GeometryHolder/<m_Blob>e__FixedBuffer";
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer
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
#[cfg(feature = "cordl_class_UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LowLevelPhysics+GeometryHolder+_m_Blob_e__FixedBuffer")]
impl crate::UnityEngine::LowLevelPhysics::GeometryHolder__m_Blob_e__FixedBuffer {}
