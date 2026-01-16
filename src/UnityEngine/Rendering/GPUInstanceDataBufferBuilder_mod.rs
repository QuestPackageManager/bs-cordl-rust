#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct GPUInstanceDataBufferBuilder {
    pub m_Components: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferBuilder";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder {
    pub fn AddComponent__cordl_bool_InstanceType_InstanceComponentGroup0<T>(
        &mut self,
        propertyID: i32,
        isOverriden: bool,
        isPerInstance: bool,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
        componentGroup: crate::UnityEngine::Rendering::InstanceComponentGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_method::<(
                        i32,
                        bool,
                        bool,
                        crate::UnityEngine::Rendering::InstanceType,
                        crate::UnityEngine::Rendering::InstanceComponentGroup,
                    ), quest_hook::libil2cpp::Void, 5usize>("AddComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddComponent",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    propertyID,
                    isOverriden,
                    isPerInstance,
                    instanceType,
                    componentGroup,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddComponent_i32__cordl_bool_InstanceType_InstanceComponentGroup1(
        &mut self,
        propertyID: i32,
        isOverriden: bool,
        byteSize: i32,
        isPerInstance: bool,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
        componentGroup: crate::UnityEngine::Rendering::InstanceComponentGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        bool,
                        i32,
                        bool,
                        crate::UnityEngine::Rendering::InstanceType,
                        crate::UnityEngine::Rendering::InstanceComponentGroup,
                    ), quest_hook::libil2cpp::Void, 6usize>("AddComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddComponent",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    propertyID,
                    isOverriden,
                    byteSize,
                    isPerInstance,
                    instanceType,
                    componentGroup,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
        instanceNumInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceNumInfo,
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
                            crate::UnityEngine::Rendering::InstanceNumInfo,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                        1usize,
                    >("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Build",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (instanceNumInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMetadataValue(
        &mut self,
        nameID: i32,
        gpuAddress: i32,
        isOverridden: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::MetadataValue> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, bool),
                        crate::UnityEngine::Rendering::MetadataValue,
                        3usize,
                    >("CreateMetadataValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMetadataValue", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::MetadataValue = unsafe {
            cordl_method_info.invoke_unchecked(self, (nameID, gpuAddress, isOverridden))?
        };
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
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferBuilder")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferBuilder
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
