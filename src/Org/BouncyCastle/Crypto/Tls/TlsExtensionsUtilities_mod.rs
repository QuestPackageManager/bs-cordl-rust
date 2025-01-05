#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsExtensionsUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsExtensionsUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsExtensionsUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsExtensionsUtilities =>
    "Org.BouncyCastle.Crypto.Tls"."TlsExtensionsUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsExtensionsUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsExtensionsUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsExtensionsUtilities")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsExtensionsUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsExtensionsUtilities")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsExtensionsUtilities {
    pub fn AddClientCertificateTypeExtensionClient(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        certificateTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddClientCertificateTypeExtensionClient",
                (extensions, certificateTypes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddClientCertificateTypeExtensionServer(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        certificateType: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddClientCertificateTypeExtensionServer",
                (extensions, certificateType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEncryptThenMacExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddEncryptThenMacExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddExtendedMasterSecretExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddExtendedMasterSecretExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddHeartbeatExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        heartbeatExtension: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::HeartbeatExtension,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddHeartbeatExtension", (extensions, heartbeatExtension))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddMaxFragmentLengthExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        maxFragmentLength: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddMaxFragmentLengthExtension", (extensions, maxFragmentLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPaddingExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        dataLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddPaddingExtension", (extensions, dataLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddServerCertificateTypeExtensionClient(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        certificateTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddServerCertificateTypeExtensionClient",
                (extensions, certificateTypes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddServerCertificateTypeExtensionServer(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        certificateType: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddServerCertificateTypeExtensionServer",
                (extensions, certificateType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddServerNameExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        serverNameList: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ServerNameList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddServerNameExtension", (extensions, serverNameList))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStatusRequestExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        statusRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatusRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStatusRequestExtension", (extensions, statusRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTruncatedHMacExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTruncatedHMacExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCertificateTypeExtensionClient(
        certificateTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCertificateTypeExtensionClient", (certificateTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCertificateTypeExtensionServer(
        certificateType: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCertificateTypeExtensionServer", (certificateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEmptyExtensionData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEmptyExtensionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEncryptThenMacExtension() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEncryptThenMacExtension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateExtendedMasterSecretExtension() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateExtendedMasterSecretExtension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHeartbeatExtension(
        heartbeatExtension: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::HeartbeatExtension,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHeartbeatExtension", (heartbeatExtension))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMaxFragmentLengthExtension(
        maxFragmentLength: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMaxFragmentLengthExtension", (maxFragmentLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePaddingExtension(
        dataLength: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePaddingExtension", (dataLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateServerNameExtension(
        serverNameList: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ServerNameList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateServerNameExtension", (serverNameList))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStatusRequestExtension(
        statusRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatusRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStatusRequestExtension", (statusRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTruncatedHMacExtension() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTruncatedHMacExtension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureExtensionsInitialised(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureExtensionsInitialised", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClientCertificateTypeExtensionClient(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClientCertificateTypeExtensionClient", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClientCertificateTypeExtensionServer(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClientCertificateTypeExtensionServer", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeartbeatExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::HeartbeatExtension,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::HeartbeatExtension,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHeartbeatExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxFragmentLengthExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaxFragmentLengthExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPaddingExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPaddingExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerCertificateTypeExtensionClient(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServerCertificateTypeExtensionClient", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerCertificateTypeExtensionServer(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServerCertificateTypeExtensionServer", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerNameExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ServerNameList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ServerNameList,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServerNameExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStatusRequestExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatusRequest,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatusRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStatusRequestExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasEncryptThenMacExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasEncryptThenMacExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasExtendedMasterSecretExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasExtendedMasterSecretExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasTruncatedHMacExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasTruncatedHMacExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadCertificateTypeExtensionClient(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadCertificateTypeExtensionClient", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCertificateTypeExtensionServer(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadCertificateTypeExtensionServer", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadEmptyExtensionData(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadEmptyExtensionData", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadEncryptThenMacExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadEncryptThenMacExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadExtendedMasterSecretExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadExtendedMasterSecretExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadHeartbeatExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::HeartbeatExtension,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::HeartbeatExtension,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadHeartbeatExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadMaxFragmentLengthExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadMaxFragmentLengthExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPaddingExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadPaddingExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadServerNameExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ServerNameList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ServerNameList,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadServerNameExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStatusRequestExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatusRequest,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatusRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadStatusRequestExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadTruncatedHMacExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadTruncatedHMacExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsExtensionsUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsExtensionsUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
