#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
#[repr(C)]
#[derive(Debug)]
pub struct CRC32C {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::Utils::CRC32C {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "CRC32C";
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
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl std::ops::Deref for crate::LiteNetLib::Utils::CRC32C {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::CRC32C {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl crate::LiteNetLib::Utils::CRC32C {
    pub const ChecksumSize: i32 = 4i32;
    pub const Poly: u32 = 2197175160u32;
    pub fn Compute(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                u32,
                3usize,
            >("Compute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compute", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (input, offset, length))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::CRC32C {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
