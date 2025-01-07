#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleZeroOrMinusOneIsInvalid")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeHandleZeroOrMinusOneIsInvalid {
    __cordl_parent: crate::System::Runtime::InteropServices::SafeHandle,
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleZeroOrMinusOneIsInvalid")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Microsoft.Win32.SafeHandles";
    const CLASS_NAME: &'static str = "SafeHandleZeroOrMinusOneIsInvalid";
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
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleZeroOrMinusOneIsInvalid")]
impl std::ops::Deref
for crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
    type Target = crate::System::Runtime::InteropServices::SafeHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleZeroOrMinusOneIsInvalid")]
impl std::ops::DerefMut
for crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleZeroOrMinusOneIsInvalid")]
impl crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
    pub fn New(
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ownsHandle))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ownsHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsInvalid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInvalid", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleZeroOrMinusOneIsInvalid")]
impl quest_hook::libil2cpp::ObjectType
for crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
