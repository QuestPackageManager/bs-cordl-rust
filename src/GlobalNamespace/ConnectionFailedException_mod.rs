#[cfg(feature = "ConnectionFailedException")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionFailedException {
    __cordl_parent: crate::System::Exception,
    pub reason: crate::GlobalNamespace::ConnectionFailedReason,
}
#[cfg(feature = "ConnectionFailedException")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectionFailedException {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectionFailedException";
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
#[cfg(feature = "ConnectionFailedException")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectionFailedException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedException")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectionFailedException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedException")]
impl crate::GlobalNamespace::ConnectionFailedException {
    pub fn New_ConnectionFailedReason0(
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reason))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        reason: crate::GlobalNamespace::ConnectionFailedReason,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reason, message))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ConnectionFailedReason0(
        &mut self,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reason))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        reason: crate::GlobalNamespace::ConnectionFailedReason,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reason, message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ConnectionFailedException")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectionFailedException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
