#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerSupportsDeferredConvertListToArray"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeContainerSupportsDeferredConvertListToArray {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerSupportsDeferredConvertListToArray"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::NativeContainerSupportsDeferredConvertListToArray
    => "Unity.Collections.LowLevel.Unsafe"
    ."NativeContainerSupportsDeferredConvertListToArray"
);
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerSupportsDeferredConvertListToArray"
)]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::NativeContainerSupportsDeferredConvertListToArray {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerSupportsDeferredConvertListToArray"
)]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::NativeContainerSupportsDeferredConvertListToArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerSupportsDeferredConvertListToArray"
)]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeContainerSupportsDeferredConvertListToArray {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerSupportsDeferredConvertListToArray"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::NativeContainerSupportsDeferredConvertListToArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}