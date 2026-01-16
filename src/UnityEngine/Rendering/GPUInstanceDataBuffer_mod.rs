#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct GPUInstanceDataBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub instanceNumInfo: crate::UnityEngine::Rendering::InstanceNumInfo,
    pub instancesNumPrefixSum: crate::Unity::Collections::NativeArray_1<i32>,
    pub instancesSpan: crate::Unity::Collections::NativeArray_1<i32>,
    pub byteSize: i32,
    pub perInstanceComponentCount: i32,
    pub version: i32,
    pub layoutVersion: i32,
    pub gpuBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    pub validComponentsIndicesGpuBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GraphicsBuffer,
    >,
    pub componentAddressesGpuBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GraphicsBuffer,
    >,
    pub componentInstanceIndexRangesGpuBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GraphicsBuffer,
    >,
    pub componentByteCountsGpuBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GraphicsBuffer,
    >,
    pub descriptions: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
    >,
    pub defaultMetadata: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::MetadataValue,
    >,
    pub gpuBufferComponentAddress: crate::Unity::Collections::NativeArray_1<i32>,
    pub nameToMetadataMap: crate::Unity::Collections::NativeParallelHashMap_2<i32, i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    #[cfg(
        feature = "UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
    )]
    pub type ConvertCPUInstancesToGPUInstancesJob = crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob;
    #[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
    pub type ReadOnly = crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly;
    pub fn AsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
                        0usize,
                    >("AsReadOnly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AsReadOnly", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CPUInstanceArrayToGPUInstanceArray(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUInstanceIndex,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CPUInstanceArrayToGPUInstanceArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CPUInstanceArrayToGPUInstanceArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, gpuInstanceIndices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CPUInstanceToGPUInstance_ByRefMut_InstanceHandle0(
        instancesNumPrefixSum: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<i32>,
        >,
        instance: crate::UnityEngine::Rendering::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::GPUInstanceIndex> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<i32>,
                            >,
                            crate::UnityEngine::Rendering::InstanceHandle,
                        ),
                        crate::UnityEngine::Rendering::GPUInstanceIndex,
                        2usize,
                    >("CPUInstanceToGPUInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CPUInstanceToGPUInstance", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUInstanceIndex = unsafe {
            cordl_method_info.invoke_unchecked((), (instancesNumPrefixSum, instance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CPUInstanceToGPUInstance_InstanceHandle1(
        &mut self,
        instance: crate::UnityEngine::Rendering::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::GPUInstanceIndex> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::InstanceHandle),
                        crate::UnityEngine::Rendering::GPUInstanceIndex,
                        1usize,
                    >("CPUInstanceToGPUInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CPUInstanceToGPUInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUInstanceIndex = unsafe {
            cordl_method_info.invoke_unchecked(self, (instance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GPUInstanceToCPUInstance(
        &mut self,
        gpuInstanceIndex: crate::UnityEngine::Rendering::GPUInstanceIndex,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::InstanceHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::GPUInstanceIndex),
                        crate::UnityEngine::Rendering::InstanceHandle,
                        1usize,
                    >("GPUInstanceToCPUInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GPUInstanceToCPUInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::InstanceHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (gpuInstanceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGpuAddress_Il2CppString0(
        &mut self,
        strName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assertOnFail: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        i32,
                        2usize,
                    >("GetGpuAddress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGpuAddress", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (strName, assertOnFail))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGpuAddress_i32_1(
        &mut self,
        propertyID: i32,
        assertOnFail: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, bool), i32, 2usize>("GetGpuAddress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGpuAddress", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (propertyID, assertOnFail))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyIndex(
        &mut self,
        propertyID: i32,
        assertOnFail: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, bool), i32, 2usize>("GetPropertyIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPropertyIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (propertyID, assertOnFail))?
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
    pub fn NextVersion() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("NextVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextVersion", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
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
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_valid", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    pub instancesNumPrefixSum: crate::Unity::Collections::NativeArray_1<i32>,
    pub instances: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::GPUInstanceIndex,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBuffer/ConvertCPUInstancesToGPUInstancesJob";
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
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
impl crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    pub const k_BatchSize: i32 = 512i32;
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUInstanceDataBuffer+ConvertCPUInstancesToGPUInstancesJob"
)]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ConvertCPUInstancesToGPUInstancesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUInstanceDataBuffer_ReadOnly {
    pub instancesNumPrefixSum: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBuffer/ReadOnly";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBuffer+ReadOnly")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly {
    pub fn CPUInstanceArrayToGPUInstanceArray(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUInstanceIndex,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CPUInstanceArrayToGPUInstanceArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CPUInstanceArrayToGPUInstanceArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, gpuInstanceIndices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CPUInstanceToGPUInstance(
        &mut self,
        instance: crate::UnityEngine::Rendering::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::GPUInstanceIndex> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::InstanceHandle),
                        crate::UnityEngine::Rendering::GPUInstanceIndex,
                        1usize,
                    >("CPUInstanceToGPUInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CPUInstanceToGPUInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUInstanceIndex = unsafe {
            cordl_method_info.invoke_unchecked(self, (instance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >),
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
            cordl_method_info.invoke_unchecked(self, (buffer))?
        };
        Ok(__cordl_ret.into())
    }
}
