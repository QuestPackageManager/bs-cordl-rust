#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct LimitedInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    pub _in: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _limit: i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1";
    const CLASS_NAME: &'static str = "LimitedInputStream";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    pub fn New(
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inStream, limit))?;
        Ok(__cordl_object.into())
    }
    pub fn SetParentEofDetect(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetParentEofDetect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetParentEofDetect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (on))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>, i32),
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
            method.invoke_unchecked(self, (inStream, limit))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Limit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Limit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Limit", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
