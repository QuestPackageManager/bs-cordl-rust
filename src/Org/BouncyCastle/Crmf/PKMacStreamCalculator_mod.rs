#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacStreamCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct PKMacStreamCalculator {
    __cordl_parent: crate::System::Object,
    pub _stream: *mut crate::Org::BouncyCastle::Crypto::IO::MacSink,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacStreamCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crmf::PKMacStreamCalculator
    => "Org.BouncyCastle.Crmf"."PKMacStreamCalculator"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacStreamCalculator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::PKMacStreamCalculator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacStreamCalculator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::PKMacStreamCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacStreamCalculator")]
impl crate::Org::BouncyCastle::Crmf::PKMacStreamCalculator {
    pub fn _ctor(
        &mut self,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mac))?;
        Ok(__cordl_ret)
    }
    pub fn get_Stream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_Stream", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mac))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacStreamCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::PKMacStreamCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
