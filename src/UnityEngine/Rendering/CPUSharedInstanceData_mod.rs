#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CPUSharedInstanceData {
    pub m_StructData: crate::Unity::Collections::NativeArray_1<i32>,
    pub m_InstanceIndices: crate::Unity::Collections::NativeList_1<i32>,
    pub instances: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::SharedInstanceHandle,
    >,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub materialIDArrays:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::SmallIntegerArray>,
    pub meshIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub localAABBs: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::AABB>,
    pub flags: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::CPUSharedInstanceFlags,
    >,
    pub lodGroupAndMasks: crate::Unity::Collections::NativeArray_1<u32>,
    pub gameObjectLayers: crate::Unity::Collections::NativeArray_1<i32>,
    pub refCounts: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CPUSharedInstanceData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CPUSharedInstanceData";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CPUSharedInstanceData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CPUSharedInstanceData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CPUSharedInstanceData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::CPUSharedInstanceData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::CPUSharedInstanceData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CPUSharedInstanceData")]
impl crate::UnityEngine::Rendering::CPUSharedInstanceData {
    pub const k_InvalidIndex: i32 = -1i32;
    pub const k_InvalidLODGroupAndMask: u32 = 4294967295u32;
    #[cfg(feature = "UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
    pub type ReadOnly = crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly;
    pub fn Add(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Add")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Add",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddNoGrow(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddNoGrow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddNoGrow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddUnsafe(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddUnsafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddUnsafe", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
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
        let __cordl_ret: crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly =
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
    pub fn EnsureFreeInstances(
        &mut self,
        instancesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "EnsureFreeInstances",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnsureFreeInstances",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instancesCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFreeInstancesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetFreeInstancesCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFreeInstancesCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_Flags(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CPUSharedInstanceFlags> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        crate::UnityEngine::Rendering::CPUSharedInstanceFlags,
                        1usize,
                    >("Get_Flags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_Flags", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::CPUSharedInstanceFlags =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_GameObjectLayer(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        i32,
                        1usize,
                    >("Get_GameObjectLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_GameObjectLayer", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_LODGroupAndMask(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        u32,
                        1usize,
                    >("Get_LODGroupAndMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_LODGroupAndMask", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_LocalAABB(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::AABB,
                        >,
                        1usize,
                    >("Get_LocalAABB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_LocalAABB", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB> =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_MaterialIDs(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SmallIntegerArray>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SmallIntegerArray,
                        >,
                        1usize,
                    >("Get_MaterialIDs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_MaterialIDs", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SmallIntegerArray,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_MeshID(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        i32,
                        1usize,
                    >("Get_MeshID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_MeshID", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_RefCount(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        i32,
                        1usize,
                    >("Get_RefCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_RefCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_RendererGroupID(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        i32,
                        1usize,
                    >("Get_RendererGroupID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Get_RendererGroupID", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Grow(
        &mut self,
        newCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Grow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Grow",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (newCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexToSharedInstance(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::SharedInstanceHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::SharedInstanceHandle,
                        1usize,
                    >("IndexToSharedInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexToSharedInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::SharedInstanceHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        initCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (initCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstanceToIndex(
        &mut self,
        instanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUInstanceData,
        >,
        instance: crate::UnityEngine::Rendering::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUInstanceData,
                        >,
                        crate::UnityEngine::Rendering::InstanceHandle,
                    ), i32, 2usize>("InstanceToIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstanceToIndex",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (instanceData, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsFreeInstanceHandle(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        bool,
                        1usize,
                    >("IsFreeInstanceHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsFreeInstanceHandle", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidIndex(&mut self, index: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("IsValidIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsValidIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidInstance(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        bool,
                        1usize,
                    >("IsValidInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValidInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Remove",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        rendererGroupID: i32,
        materialIDs: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SmallIntegerArray,
        >,
        meshID: i32,
        localAABB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
        transformUpdateFlags: crate::UnityEngine::Rendering::TransformUpdateFlags,
        instanceFlags: crate::UnityEngine::Rendering::InstanceFlags,
        lodGroupAndMask: u32,
        gameObjectLayer: i32,
        refCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::SharedInstanceHandle,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SmallIntegerArray,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
                        crate::UnityEngine::Rendering::TransformUpdateFlags,
                        crate::UnityEngine::Rendering::InstanceFlags,
                        u32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 10usize>("Set")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Set",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    instance,
                    rendererGroupID,
                    materialIDs,
                    meshID,
                    localAABB,
                    transformUpdateFlags,
                    instanceFlags,
                    lodGroupAndMask,
                    gameObjectLayer,
                    refCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDefault(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_Flags(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        instanceFlags: crate::UnityEngine::Rendering::CPUSharedInstanceFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::SharedInstanceHandle,
                        crate::UnityEngine::Rendering::CPUSharedInstanceFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>("Set_Flags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Set_Flags",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, instanceFlags))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_GameObjectLayer(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        gameObjectLayer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Set_GameObjectLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Set_GameObjectLayer", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, gameObjectLayer))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_LODGroupAndMask(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        lodGroupAndMask: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle, u32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Set_LODGroupAndMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Set_LODGroupAndMask", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, lodGroupAndMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_LocalAABB(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        localAABB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::SharedInstanceHandle,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Set_LocalAABB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Set_LocalAABB",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, localAABB))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_MaterialIDs(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        materialIDs: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SmallIntegerArray,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::SharedInstanceHandle,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SmallIntegerArray,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Set_MaterialIDs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Set_MaterialIDs",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, materialIDs))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_MeshID(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        meshID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Set_MeshID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Set_MeshID", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, meshID))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_RefCount(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        refCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Set_RefCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Set_RefCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, refCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn Set_RendererGroupID(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
        rendererGroupID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Set_RendererGroupID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Set_RendererGroupID", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, rendererGroupID))? };
        Ok(__cordl_ret.into())
    }
    pub fn SharedInstanceToIndex(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        i32,
                        1usize,
                    >("SharedInstanceToIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SharedInstanceToIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_handlesLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_handlesLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_handlesLength",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instancesCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_instancesCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_instancesCapacity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instancesLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_instancesLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_instancesLength",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_instancesCapacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_instancesCapacity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_instancesCapacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_instancesLength(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_instancesLength",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_instancesLength",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CPUSharedInstanceData")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::CPUSharedInstanceData {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CPUSharedInstanceData")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::CPUSharedInstanceData {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct CPUSharedInstanceData_ReadOnly {
    pub instanceIndices: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub instances: crate::Unity::Collections::NativeArray_1_ReadOnly<
        crate::UnityEngine::Rendering::SharedInstanceHandle,
    >,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub materialIDArrays: crate::Unity::Collections::NativeArray_1_ReadOnly<
        crate::UnityEngine::Rendering::SmallIntegerArray,
    >,
    pub meshIDs: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub localAABBs:
        crate::Unity::Collections::NativeArray_1_ReadOnly<crate::UnityEngine::Rendering::AABB>,
    pub flags: crate::Unity::Collections::NativeArray_1_ReadOnly<
        crate::UnityEngine::Rendering::CPUSharedInstanceFlags,
    >,
    pub lodGroupAndMasks: crate::Unity::Collections::NativeArray_1_ReadOnly<u32>,
    pub gameObjectLayers: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub refCounts: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CPUSharedInstanceData/ReadOnly";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CPUSharedInstanceData+ReadOnly")]
impl crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly {
    pub fn IndexToSharedInstance(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::SharedInstanceHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::SharedInstanceHandle,
                        1usize,
                    >("IndexToSharedInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexToSharedInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::SharedInstanceHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstanceToIndex(
        &mut self,
        instanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
        >,
        instance: crate::UnityEngine::Rendering::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
                        >,
                        crate::UnityEngine::Rendering::InstanceHandle,
                    ), i32, 2usize>("InstanceToIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstanceToIndex",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (instanceData, instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidIndex(&mut self, index: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("IsValidIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsValidIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidSharedInstance(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        bool,
                        1usize,
                    >("IsValidSharedInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValidSharedInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn SharedInstanceToIndex(
        &mut self,
        instance: crate::UnityEngine::Rendering::SharedInstanceHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SharedInstanceHandle),
                        i32,
                        1usize,
                    >("SharedInstanceToIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SharedInstanceToIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        instanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUSharedInstanceData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::CPUSharedInstanceData,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instanceData))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_handlesLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_handlesLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_handlesLength",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instancesLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_instancesLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_instancesLength",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
