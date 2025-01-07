#[cfg(feature = "OVRExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRExtensions";
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
#[cfg(feature = "OVRExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OVRExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExtensions")]
impl crate::GlobalNamespace::OVRExtensions {
    pub fn ConvertToHMDMatrix34(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::HmdMatrix34_t> {
        let __cordl_ret: crate::OVR::OpenVR::HmdMatrix34_t = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToHMDMatrix34", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        gradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
        otherGradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyFrom", (gradient, otherGradient))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        gradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
        otherGradient: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Equals", (gradient, otherGradient))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindChildRecursive(
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindChildRecursive", (parent, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromColorf(
        c: crate::GlobalNamespace::OVRPlugin_Colorf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromColorf", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedXQuatf(
        q: crate::GlobalNamespace::OVRPlugin_Quatf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFlippedXQuatf", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedXVector2f(
        v: crate::GlobalNamespace::OVRPlugin_Vector2f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFlippedXVector2f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedXVector3f(
        v: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFlippedXVector3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedZQuatf(
        q: crate::GlobalNamespace::OVRPlugin_Quatf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFlippedZQuatf", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFlippedZVector3f(
        v: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFlippedZVector3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromOVRPose(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        pose: crate::GlobalNamespace::OVRPose,
        isLocal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromOVRPose", (t, pose, isLocal))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromQuatf(
        q: crate::GlobalNamespace::OVRPlugin_Quatf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromQuatf", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSize3f(
        v: crate::GlobalNamespace::OVRPlugin_Size3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSize3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSizef(
        v: crate::GlobalNamespace::OVRPlugin_Sizef,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSizef", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromVector2f(
        v: crate::GlobalNamespace::OVRPlugin_Vector2f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromVector2f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromVector3f(
        v: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromVector3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromVector4f(
        v: crate::GlobalNamespace::OVRPlugin_Vector4f,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromVector4f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToColorf(
        c: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Colorf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Colorf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToColorf", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedXQuatf(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Quatf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Quatf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFlippedXQuatf", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedXVector3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFlippedXVector3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedZQuatf(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Quatf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Quatf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFlippedZQuatf", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFlippedZVector3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFlippedZVector3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFrustum(
        f: crate::GlobalNamespace::OVRPlugin_Frustumf,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTracker_Frustum> {
        let __cordl_ret: crate::GlobalNamespace::OVRTracker_Frustum = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFrustum", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHeadSpacePose_OVRPose0(
        trackingSpacePose: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHeadSpacePose", (trackingSpacePose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHeadSpacePose_Transform_Camera1(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHeadSpacePose", (transform, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToNativeArray<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToNativeArray", (enumerable, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToNonAlloc<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVREnumerable_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVREnumerable_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToNonAlloc", (enumerable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRPose_OVRPlugin_Posef1(
        p: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToOVRPose", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRPose_Transform__cordl_bool0(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        isLocal: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToOVRPose", (t, isLocal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToQuatf(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Quatf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Quatf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToQuatf", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSize3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Size3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Size3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSize3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSizef(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Sizef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Sizef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSizef", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSpaceStorageLocation(
        storageLocation: crate::GlobalNamespace::OVRSpace_StorageLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSpaceStorageLocation", (storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTrackingSpacePose(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTrackingSpacePose", (transform, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToVector2f(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector2f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector2f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToVector2f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToVector3f(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToVector3f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToVector4f(
        v: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector4f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector4f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToVector4f", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToWorldSpacePose_Camera1(
        trackingSpacePose: crate::GlobalNamespace::OVRPose,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToWorldSpacePose", (trackingSpacePose, mainCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToWorldSpacePose_OVRPose0(
        trackingSpacePose: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToWorldSpacePose", (trackingSpacePose))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
