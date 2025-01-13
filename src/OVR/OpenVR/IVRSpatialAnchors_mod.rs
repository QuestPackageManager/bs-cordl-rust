#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVRSpatialAnchors {
    pub CreateSpatialAnchorFromDescriptor: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor,
    >,
    pub CreateSpatialAnchorFromPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose,
    >,
    pub GetSpatialAnchorPose: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose,
    >,
    pub GetSpatialAnchorDescriptor: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRSpatialAnchors {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRSpatialAnchors";
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVRSpatialAnchors {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVRSpatialAnchors {
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVRSpatialAnchors {
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVRSpatialAnchors {
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::IVRSpatialAnchors {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors")]
impl crate::OVR::OpenVR::IVRSpatialAnchors {
    #[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
    pub type _CreateSpatialAnchorFromDescriptor = crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor;
    #[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
    pub type _CreateSpatialAnchorFromPose = crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose;
    #[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
    pub type _GetSpatialAnchorDescriptor = crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor;
    #[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
    pub type _GetSpatialAnchorPose = crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose;
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRSpatialAnchors/_CreateSpatialAnchorFromDescriptor";
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
impl crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor {
    pub fn BeginInvoke(
        &mut self,
        pchDescriptor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchDescriptor, pHandleOut, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("EndInvoke", (pHandleOut, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchDescriptor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("Invoke", (pchDescriptor, pHandleOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSpatialAnchors__CreateSpatialAnchorFromPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRSpatialAnchors/_CreateSpatialAnchorFromPose";
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
impl crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose {
    pub fn BeginInvoke(
        &mut self,
        unDeviceIndex: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPose: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::SpatialAnchorPose_t>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (unDeviceIndex, eOrigin, pPose, pHandleOut, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pPose: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::SpatialAnchorPose_t>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("EndInvoke", (pPose, pHandleOut, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unDeviceIndex: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPose: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::SpatialAnchorPose_t>,
        pHandleOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("Invoke", (unDeviceIndex, eOrigin, pPose, pHandleOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_CreateSpatialAnchorFromPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSpatialAnchors__CreateSpatialAnchorFromPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSpatialAnchors__GetSpatialAnchorDescriptor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRSpatialAnchors/_GetSpatialAnchorDescriptor";
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
impl crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor {
    pub fn BeginInvoke(
        &mut self,
        unHandle: u32,
        pchDescriptorOut: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        punDescriptorBufferLenInOut: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    unHandle,
                    pchDescriptorOut,
                    punDescriptorBufferLenInOut,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punDescriptorBufferLenInOut: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("EndInvoke", (punDescriptorBufferLenInOut, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unHandle: u32,
        pchDescriptorOut: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        punDescriptorBufferLenInOut: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke(
                "Invoke",
                (unHandle, pchDescriptorOut, punDescriptorBufferLenInOut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSpatialAnchors__GetSpatialAnchorPose {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRSpatialAnchors/_GetSpatialAnchorPose";
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
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
impl crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose {
    pub fn BeginInvoke(
        &mut self,
        unHandle: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPoseOut: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::SpatialAnchorPose_t,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (unHandle, eOrigin, pPoseOut, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pPoseOut: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::SpatialAnchorPose_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("EndInvoke", (pPoseOut, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        unHandle: u32,
        eOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pPoseOut: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::SpatialAnchorPose_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRSpatialAnchorError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRSpatialAnchorError = __cordl_object
            .invoke("Invoke", (unHandle, eOrigin, pPoseOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSpatialAnchors+_GetSpatialAnchorPose")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSpatialAnchors__GetSpatialAnchorPose {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
