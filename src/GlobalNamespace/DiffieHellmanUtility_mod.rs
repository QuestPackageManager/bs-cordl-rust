#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility_DiffieHellmanKeyPair {
    __cordl_parent: crate::System::Object,
    pub _dhBasicAgreement: *mut crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement,
    pub _publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair => ""
    ."DiffieHellmanUtility/DiffieHellmanKeyPair"
);
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl std::ops::Deref
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    #[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::GlobalNamespace::DiffieHellmanKeyPair___c__DisplayClass5_0;
    pub fn GetPreMasterSecret(
        &mut self,
        clientPublicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetPreMasterSecret", (clientPublicKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreMasterSecretAsync(
        &mut self,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        clientPublicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("GetPreMasterSecretAsync", (taskUtility, clientPublicKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        privateKeyParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        privateKeyParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_publicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_publicKey", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DiffieHellmanUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "DiffieHellmanUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for DiffieHellmanUtility => ""."DiffieHellmanUtility"
);
#[cfg(feature = "DiffieHellmanUtility")]
impl std::ops::Deref for DiffieHellmanUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility")]
impl std::ops::DerefMut for DiffieHellmanUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility")]
impl DiffieHellmanUtility {
    pub const kMaxDiffieHellmanPublicKeyLength: i32 = 2048i32;
    pub const kMaxElipticalCurvePublicKeyLength: i32 = 256i32;
    #[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
    pub type OperationTimer = crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer;
    #[cfg(feature = "DiffieHellmanUtility+KeyType")]
    pub type KeyType = crate::GlobalNamespace::DiffieHellmanUtility_KeyType;
    #[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
    pub type DiffieHellmanKeyPair = crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair;
    #[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
    pub type ElipticalCurveKeyPair = crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair;
}
#[cfg(feature = "DiffieHellmanUtility")]
impl quest_hook::libil2cpp::ObjectType for DiffieHellmanUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility_ElipticalCurveKeyPair {
    __cordl_parent: crate::System::Object,
    pub _ecdhBasicAgreement: *mut crate::Org::BouncyCastle::Crypto::Agreement::ECDHBasicAgreement,
    pub _publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair => ""
    ."DiffieHellmanUtility/ElipticalCurveKeyPair"
);
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl std::ops::Deref
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    #[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::GlobalNamespace::ElipticalCurveKeyPair___c__DisplayClass5_0;
    pub fn GetPreMasterSecret(
        &mut self,
        clientPublicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetPreMasterSecret", (clientPublicKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreMasterSecretAsync(
        &mut self,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        clientPublicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("GetPreMasterSecretAsync", (taskUtility, clientPublicKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        privateKeyParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        privateKeyParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_publicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_publicKey", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffieHellmanUtility_KeyType {
    DiffieHellman = 0i32,
    ElipticalCurve = 1i32,
}
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DiffieHellmanUtility_KeyType =>
    ""."DiffieHellmanUtility/KeyType"
);
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility_OperationTimer {
    __cordl_parent: crate::System::Object,
    pub _stopwatch: *mut crate::System::Diagnostics::Stopwatch,
    pub _operationName: *mut crate::System::String,
}
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DiffieHellmanUtility_OperationTimer => ""
    ."DiffieHellmanUtility/OperationTimer"
);
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl std::ops::Deref for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl std::ops::DerefMut for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        operationName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operationName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        operationName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operationName))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
