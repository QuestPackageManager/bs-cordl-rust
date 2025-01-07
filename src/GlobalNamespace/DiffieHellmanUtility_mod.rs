#[cfg(feature = "DiffieHellmanUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "DiffieHellmanUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DiffieHellmanUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DiffieHellmanUtility";
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
#[cfg(feature = "DiffieHellmanUtility")]
impl std::ops::Deref for crate::GlobalNamespace::DiffieHellmanUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::DiffieHellmanUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DiffieHellmanUtility")]
impl crate::GlobalNamespace::DiffieHellmanUtility {
    pub const kMaxDiffieHellmanPublicKeyLength: i32 = 2048i32;
    pub const kMaxElipticalCurvePublicKeyLength: i32 = 256i32;
    #[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
    pub type DiffieHellmanKeyPair = crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair;
    #[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
    pub type ElipticalCurveKeyPair = crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair;
    #[cfg(feature = "DiffieHellmanUtility+KeyType")]
    pub type KeyType = crate::GlobalNamespace::DiffieHellmanUtility_KeyType;
    #[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
    pub type OperationTimer = crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer;
    pub fn GenerateDiffieHellmanKeys() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateDiffieHellmanKeys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateElipticalCurveKeys() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateElipticalCurveKeys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKeys(
        keyType: crate::GlobalNamespace::DiffieHellmanUtility_KeyType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IDiffieHellmanKeyPair>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IDiffieHellmanKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateKeys", (keyType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKeysAsync(
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        cancellationToken: crate::System::Threading::CancellationToken,
        keyType: crate::GlobalNamespace::DiffieHellmanUtility_KeyType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IDiffieHellmanKeyPair>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IDiffieHellmanKeyPair>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateKeysAsync", (taskUtility, cancellationToken, keyType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreMasterSecret_DHBasicAgreement0(
        dhBasicAgreement: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement,
        >,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreMasterSecret", (dhBasicAgreement, clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreMasterSecret_ECDHBasicAgreement1(
        ecdhBasicAgreement: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Agreement::ECDHBasicAgreement,
        >,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreMasterSecret", (ecdhBasicAgreement, clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DiffieHellmanUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DiffieHellmanUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility_DiffieHellmanKeyPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dhBasicAgreement: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement,
    >,
    pub _publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DiffieHellmanKeyPair";
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
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl std::ops::Deref
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetPreMasterSecret(
        &mut self,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetPreMasterSecret", (clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreMasterSecretAsync(
        &mut self,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        > = __cordl_object
            .invoke("GetPreMasterSecretAsync", (taskUtility, clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        privateKeyParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
        >,
        publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        privateKeyParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
        >,
        publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_publicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_publicKey", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl AsRef<crate::GlobalNamespace::IDiffieHellmanKeyPair>
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    fn as_ref(&self) -> &crate::GlobalNamespace::IDiffieHellmanKeyPair {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DiffieHellmanUtility+DiffieHellmanKeyPair")]
impl AsMut<crate::GlobalNamespace::IDiffieHellmanKeyPair>
for crate::GlobalNamespace::DiffieHellmanUtility_DiffieHellmanKeyPair {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IDiffieHellmanKeyPair {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility_ElipticalCurveKeyPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ecdhBasicAgreement: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Agreement::ECDHBasicAgreement,
    >,
    pub _publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ElipticalCurveKeyPair";
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
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl std::ops::Deref
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetPreMasterSecret(
        &mut self,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetPreMasterSecret", (clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreMasterSecretAsync(
        &mut self,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        clientPublicKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        > = __cordl_object
            .invoke("GetPreMasterSecretAsync", (taskUtility, clientPublicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        privateKeyParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        privateKeyParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyParameters, publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_publicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_publicKey", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl AsRef<crate::GlobalNamespace::IDiffieHellmanKeyPair>
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    fn as_ref(&self) -> &crate::GlobalNamespace::IDiffieHellmanKeyPair {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DiffieHellmanUtility+ElipticalCurveKeyPair")]
impl AsMut<crate::GlobalNamespace::IDiffieHellmanKeyPair>
for crate::GlobalNamespace::DiffieHellmanUtility_ElipticalCurveKeyPair {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IDiffieHellmanKeyPair {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffieHellmanUtility_KeyType {
    #[default]
    DiffieHellman = 0i32,
    ElipticalCurve = 1i32,
}
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DiffieHellmanUtility_KeyType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KeyType";
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
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::DiffieHellmanUtility_KeyType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::DiffieHellmanUtility_KeyType {
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
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::DiffieHellmanUtility_KeyType {
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
#[cfg(feature = "DiffieHellmanUtility+KeyType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::DiffieHellmanUtility_KeyType {
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
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
#[repr(C)]
#[derive(Debug)]
pub struct DiffieHellmanUtility_OperationTimer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _stopwatch: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Stopwatch>,
    pub _operationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OperationTimer";
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
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl std::ops::Deref for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        operationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operationName))?;
        Ok(__cordl_object.into())
    }
    pub fn Time(
        operation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Time", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        operationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (operationName))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DiffieHellmanUtility+OperationTimer")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::DiffieHellmanUtility_OperationTimer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
