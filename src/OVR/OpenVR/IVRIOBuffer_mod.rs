#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVRIOBuffer {
    pub Open: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRIOBuffer__Open>,
    pub Close: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRIOBuffer__Close>,
    pub Read: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRIOBuffer__Read>,
    pub Write: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRIOBuffer__Write>,
    pub PropertyContainer: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRIOBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRIOBuffer";
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVRIOBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVRIOBuffer {
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVRIOBuffer {
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVRIOBuffer {
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRIOBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
impl crate::OVR::OpenVR::IVRIOBuffer {
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
    pub type _Close = crate::OVR::OpenVR::IVRIOBuffer__Close;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
    pub type _Open = crate::OVR::OpenVR::IVRIOBuffer__Open;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
    pub type _PropertyContainer = crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
    pub type _Read = crate::OVR::OpenVR::IVRIOBuffer__Read;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
    pub type _Write = crate::OVR::OpenVR::IVRIOBuffer__Write;
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Close {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRIOBuffer__Close {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRIOBuffer/_Close";
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Close {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Close {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl crate::OVR::OpenVR::IVRIOBuffer__Close {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Close as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                3usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Close as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (ulBuffer, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Close as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::OVR::OpenVR::EIOBufferError,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Close as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulBuffer: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Close as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u64), crate::OVR::OpenVR::EIOBufferError, 1usize>("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Close as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (ulBuffer))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Close as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Close as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Close {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Open {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRIOBuffer__Open {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRIOBuffer/_Open";
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Open {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Open {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl crate::OVR::OpenVR::IVRIOBuffer__Open {
    pub fn BeginInvoke(
        &mut self,
        pchPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::OVR::OpenVR::EIOBufferMode,
        unElementSize: u32,
        unElements: u32,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::OVR::OpenVR::EIOBufferMode,
                    u32,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                7usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type
                    > ::class(), "BeginInvoke", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pchPath,
                        mode,
                        unElementSize,
                        unElements,
                        pulBuffer,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u64>,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                crate::OVR::OpenVR::EIOBufferError,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type
                    > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (pulBuffer, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::OVR::OpenVR::EIOBufferMode,
        unElementSize: u32,
        unElements: u32,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::OVR::OpenVR::EIOBufferMode,
                    u32,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                ),
                crate::OVR::OpenVR::EIOBufferError,
                5usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type
                    > ::class(), "Invoke", 5usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pchPath, mode, unElementSize, unElements, pulBuffer),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Open as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Open {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__PropertyContainer {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRIOBuffer/_PropertyContainer";
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                3usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__PropertyContainer as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (ulBuffer, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                u64,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__PropertyContainer as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self, ulBuffer: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u64), u64, 1usize>("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__PropertyContainer as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, (ulBuffer))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__PropertyContainer as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Read {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRIOBuffer__Read {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRIOBuffer/_Read";
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Read {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Read {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl crate::OVR::OpenVR::IVRIOBuffer__Read {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        pDst: crate::System::IntPtr,
        unBytes: u32,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u64,
                    crate::System::IntPtr,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type
                    > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulBuffer, pDst, unBytes, punRead, callback, object),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                crate::OVR::OpenVR::EIOBufferError,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type
                    > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (punRead, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulBuffer: u64,
        pDst: crate::System::IntPtr,
        unBytes: u32,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u64, crate::System::IntPtr, u32, quest_hook::libil2cpp::ByRefMut<u32>),
                crate::OVR::OpenVR::EIOBufferError,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type
                    > ::class(), "Invoke", 4usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (ulBuffer, pDst, unBytes, punRead))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Read as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Read {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Write {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRIOBuffer__Write {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRIOBuffer/_Write";
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
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Write {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Write {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl crate::OVR::OpenVR::IVRIOBuffer__Write {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        pSrc: crate::System::IntPtr,
        unBytes: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Write as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u64,
                    crate::System::IntPtr,
                    u32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                5usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Write as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (ulBuffer, pSrc, unBytes, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Write as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::OVR::OpenVR::EIOBufferError,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Write as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulBuffer: u64,
        pSrc: crate::System::IntPtr,
        unBytes: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Write as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u64, crate::System::IntPtr, u32),
                crate::OVR::OpenVR::EIOBufferError,
                3usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Write as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            method.invoke_unchecked(self, (ulBuffer, pSrc, unBytes))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRIOBuffer__Write as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRIOBuffer__Write as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Write {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
