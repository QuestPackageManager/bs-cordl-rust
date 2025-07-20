#[cfg(feature = "System+Net+ReceiveState")]
#[repr(C)]
#[derive(Debug)]
pub struct ReceiveState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Resp: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
    pub ValidThrough: i32,
    pub Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub Connection: quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>,
}
#[cfg(feature = "System+Net+ReceiveState")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::ReceiveState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "ReceiveState";
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
#[cfg(feature = "System+Net+ReceiveState")]
impl std::ops::Deref for crate::System::Net::ReceiveState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ReceiveState")]
impl std::ops::DerefMut for crate::System::Net::ReceiveState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ReceiveState")]
impl crate::System::Net::ReceiveState {
    pub fn New(
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connection))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::ReceiveState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::ReceiveState as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (connection))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ReceiveState")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ReceiveState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
