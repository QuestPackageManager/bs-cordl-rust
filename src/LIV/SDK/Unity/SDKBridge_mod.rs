#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKBridge {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
unsafe impl quest_hook::libil2cpp::Type for crate::LIV::SDK::Unity::SDKBridge {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LIV.SDK.Unity";
    const CLASS_NAME: &'static str = "SDKBridge";
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
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKBridge {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKBridge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl crate::LIV::SDK::Unity::SDKBridge {
    #[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
    pub type SDKInjection_1<T: quest_hook::libil2cpp::Type> = crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<
        T,
    >;
    pub fn AddObjectToChannel(
        slot: i32,
        obj: crate::System::IntPtr,
        objectsize: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddObjectToChannel", (slot, obj, objectsize, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddObjectToCompositorChannel(
        slot: i32,
        obj: crate::System::IntPtr,
        objectsize: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddObjectToCompositorChannel", (slot, obj, objectsize, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddObjectToFrame(
        obj: crate::System::IntPtr,
        objectsize: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddObjectToFrame", (obj, objectsize, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddString(
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        slot: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddString", (tag, value, slot))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStringToChannel(
        slot: i32,
        str: crate::System::IntPtr,
        length: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStringToChannel", (slot, str, length, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStructToFrame<T>(
        mystruct: quest_hook::libil2cpp::ByRefMut<T>,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStructToFrame", (mystruct, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStructToGlobalChannel<T>(
        mystruct: quest_hook::libil2cpp::ByRefMut<T>,
        channel: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStructToGlobalChannel", (mystruct, channel, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStructToLocalChannel<T>(
        mystruct: quest_hook::libil2cpp::ByRefMut<T>,
        channel: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStructToLocalChannel", (mystruct, channel, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTexture_SDKTexture1(
        texture: crate::LIV::SDK::Unity::SDKTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTexture", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTexture_u64_0(
        texture: crate::LIV::SDK::Unity::SDKTexture,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTexture", (texture, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFrame(
        frame: crate::LIV::SDK::Unity::SDKOutputFrame,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFrame", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChannelObject(
        slot: i32,
        tag: u64,
        timestamp: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetChannelObject", (slot, tag, timestamp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCompositorChannelObject(
        slot: i32,
        tag: u64,
        timestamp: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCompositorChannelObject", (slot, tag, timestamp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentTime() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentTimeTicks() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentTimeTicks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIsCaptureActive() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsCaptureActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectTime(
        objectptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectTime", (objectptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectTimeStamp(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectTimeStamp", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderEventFunc() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderEventFunc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResolution(
        sdkResolution: quest_hook::libil2cpp::ByRefMut<
            crate::LIV::SDK::Unity::SDKResolution,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResolution", (sdkResolution))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStructFromGlobalChannel<T>(
        mystruct: quest_hook::libil2cpp::ByRefMut<T>,
        channel: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStructFromGlobalChannel", (mystruct, channel, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStructFromLocalChannel<T>(
        mystruct: quest_hook::libil2cpp::ByRefMut<T>,
        channel: i32,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStructFromLocalChannel", (mystruct, channel, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetViewfinderTexture() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKTexture,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKTexture = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetViewfinderTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetViewportTexture() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetViewportTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEvent() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IssuePluginEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGroundPlane(
        groundPlane: crate::LIV::SDK::Unity::SDKPlane,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGroundPlane", (groundPlane))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubmitApplicationOutput(
        applicationOutput: crate::LIV::SDK::Unity::SDKApplicationOutput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubmitApplicationOutput", (applicationOutput))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tag(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tag", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInputFrame(
        setframe: quest_hook::libil2cpp::ByRefMut<crate::LIV::SDK::Unity::SDKInputFrame>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateInputFrame", (setframe))?;
        Ok(__cordl_ret.into())
    }
    pub fn addtexture(
        sourcetexture: crate::System::IntPtr,
        tag: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("addtexture", (sourcetexture, tag))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsActive() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn updatinputframe(
        InputFrame: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("updatinputframe", (InputFrame))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKBridge {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SDKBridge_SDKInjection_1<T: quest_hook::libil2cpp::Type> {
    pub active: bool,
    pub action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub data: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LIV.SDK.Unity";
    const CLASS_NAME: &'static str = "SDKInjection`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LIV.SDK.Unity",
                        "SDKInjection`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
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
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
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
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
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
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKBridge+SDKInjection_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LIV::SDK::Unity::SDKBridge_SDKInjection_1<T> {}
