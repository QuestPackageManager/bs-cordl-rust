#[cfg(feature = "EncryptionUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptionUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "EncryptionUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EncryptionUtility => ""
    ."EncryptionUtility"
);
#[cfg(feature = "EncryptionUtility")]
impl std::ops::Deref for crate::GlobalNamespace::EncryptionUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EncryptionUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::EncryptionUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EncryptionUtility")]
impl crate::GlobalNamespace::EncryptionUtility {
    pub const kBlockSize: i32 = 16i32;
    pub const kExtraSize: i32 = 62i32;
    pub const kIvSize: i32 = 16i32;
    pub const kKeySize: i32 = 32i32;
    pub const kMacHashSize: i32 = 10i32;
    pub const kMacKeySize: i32 = 64i32;
    pub const kMasterKeySize: i32 = 48i32;
    pub const kMaxPadding: i32 = 32i32;
    pub const kRandomBlockSize: i32 = 32i32;
    pub const kSequenceNumberSize: i32 = 4i32;
    #[cfg(feature = "EncryptionUtility+EncryptionState")]
    pub type EncryptionState = crate::GlobalNamespace::EncryptionUtility_EncryptionState;
    #[cfg(feature = "EncryptionUtility+IEncryptionState")]
    type IEncryptionState = crate::GlobalNamespace::EncryptionUtility_IEncryptionState;
    pub fn CreateEncryptionState(
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptionState",
                (preMasterSecret, serverSeed, clientSeed, isClient),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEncryptionStateAsync(
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptionStateAsync",
                (taskUtility, preMasterSecret, serverSeed, clientSeed, isClient),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptData(
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_EncryptionState,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        extraPrefixedData: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncryptData", (state, data, offset, length, extraPrefixedData))?;
        Ok(__cordl_ret.into())
    }
    pub fn FastCopyBlock(
        inArr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        outArr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FastCopyBlock", (inArr, inOff, outArr, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn FastCopyMac(
        inArr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        outArr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FastCopyMac", (inArr, inOff, outArr, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidLength(length: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidLength", (length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDecryptData(
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EncryptionUtility_EncryptionState,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryDecryptData", (state, data, offset, length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EncryptionUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::EncryptionUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptionUtility_EncryptionState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _isValid: bool,
    pub _lastSentSequenceNum: i32,
    pub _hasReceivedSequenceNum: bool,
    pub _lastReceivedSequenceNum: u32,
    pub _receivedSequenceNumBuffer: *mut quest_hook::libil2cpp::Il2CppArray<bool>,
    pub sendKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub receiveKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _sendMacKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _receiveMacKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _sendMacQueue: *mut crate::System::Collections::Concurrent::ConcurrentQueue_1<
        *mut crate::Org::BouncyCastle::Crypto::Macs::HMac,
    >,
    pub _receiveMacQueue: *mut crate::System::Collections::Concurrent::ConcurrentQueue_1<
        *mut crate::Org::BouncyCastle::Crypto::Macs::HMac,
    >,
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EncryptionUtility_EncryptionState => ""
    ."EncryptionUtility/EncryptionState"
);
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl std::ops::Deref for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl std::ops::DerefMut for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    pub const kReceivedSequencNumBufferLength: i32 = 64i32;
    pub fn ComputeReceiveMac(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ComputeReceiveMac", (data, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeSendMac(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ComputeSendMac", (data, offset, count))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn EncryptData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        extraPrefixBytes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptData", (data, offset, length, extraPrefixBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextSentSequenceNum(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetNextSentSequenceNum", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidSequenceNum(
        &mut self,
        sequenceNum: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidSequenceNum", (sequenceNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeSeed(
        baseSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        serverSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeSeed", (baseSeed, serverSeed, clientSeed))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (preMasterSecret, serverSeed, clientSeed, isClient))?;
        Ok(__cordl_object.into())
    }
    pub fn PRF(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PRF", (key, seed, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn PRF_Hash(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PRF_Hash", (key, seed, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn PutSequenceNum(
        &mut self,
        sequenceNum: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PutSequenceNum", (sequenceNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDecryptData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryDecryptData", (data, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        preMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        serverSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clientSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        isClient: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (preMasterSecret, serverSeed, clientSeed, isClient))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isValid", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl AsRef<crate::GlobalNamespace::EncryptionUtility_IEncryptionState>
for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    fn as_ref(&self) -> &crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl AsMut<crate::GlobalNamespace::EncryptionUtility_IEncryptionState>
for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EncryptionUtility+EncryptionState")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::EncryptionUtility_EncryptionState {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptionUtility_IEncryptionState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EncryptionUtility_IEncryptionState => ""
    ."EncryptionUtility/IEncryptionState"
);
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
impl std::ops::Deref for crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
impl std::ops::DerefMut for crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
impl crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
    pub fn EncryptData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        extraPrefixBytes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptData", (data, offset, length, extraPrefixBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDecryptData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryDecryptData", (data, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_isValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isValid", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EncryptionUtility+IEncryptionState")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::EncryptionUtility_IEncryptionState {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
