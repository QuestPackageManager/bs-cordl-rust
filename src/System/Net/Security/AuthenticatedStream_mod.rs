#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticatedStream {
    __cordl_parent: crate::System::IO::Stream,
    pub _InnerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _LeaveStreamOpen: bool,
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Security::AuthenticatedStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Security";
    const CLASS_NAME: &'static str = "AuthenticatedStream";
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
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl std::ops::Deref for crate::System::Net::Security::AuthenticatedStream {
    type Target = crate::System::IO::Stream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl std::ops::DerefMut for crate::System::Net::Security::AuthenticatedStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl crate::System::Net::Security::AuthenticatedStream {
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disposing))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leaveInnerStreamOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerStream, leaveInnerStreamOpen))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leaveInnerStreamOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (innerStream, leaveInnerStreamOpen))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InnerStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                0usize,
            >("get_InnerStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_InnerStream", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsAuthenticated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsAuthenticated", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Security::AuthenticatedStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
