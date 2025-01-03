#[cfg(feature = "System+Runtime+InteropServices+SafeBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeBuffer {
    __cordl_parent: crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,
    pub _numBytes: crate::System::UIntPtr,
}
#[cfg(feature = "System+Runtime+InteropServices+SafeBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::SafeBuffer =>
    "System.Runtime.InteropServices"."SafeBuffer"
);
#[cfg(feature = "System+Runtime+InteropServices+SafeBuffer")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::SafeBuffer {
    type Target = crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+SafeBuffer")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::SafeBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+SafeBuffer")]
impl crate::System::Runtime::InteropServices::SafeBuffer {
    pub fn AcquirePointer(
        &mut self,
        pointer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcquirePointer", (pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotInitialized() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::InvalidOperationException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::InvalidOperationException,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("NotInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleasePointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleasePointer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+SafeBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::SafeBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
