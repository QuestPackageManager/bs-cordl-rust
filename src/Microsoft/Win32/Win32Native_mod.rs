#[cfg(feature = "Microsoft+Win32+Win32Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Win32Native {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::Win32Native =>
    "Microsoft.Win32"."Win32Native"
);
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl std::ops::Deref for crate::Microsoft::Win32::Win32Native {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl std::ops::DerefMut for crate::Microsoft::Win32::Win32Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl crate::Microsoft::Win32::Win32Native {
    pub fn GetMessage(
        hr: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetMessage", (hr))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeHRFromErrorCode(errorCode: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeHRFromErrorCode", (errorCode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl quest_hook::libil2cpp::ObjectType for crate::Microsoft::Win32::Win32Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
