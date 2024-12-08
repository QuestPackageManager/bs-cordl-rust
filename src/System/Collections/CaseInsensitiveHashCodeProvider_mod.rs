#[cfg(feature = "System+Collections+CaseInsensitiveHashCodeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct CaseInsensitiveHashCodeProvider {
    __cordl_parent: crate::System::Object,
    pub _compareInfo: *mut crate::System::Globalization::CompareInfo,
}
#[cfg(feature = "System+Collections+CaseInsensitiveHashCodeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::CaseInsensitiveHashCodeProvider => "System.Collections"
    ."CaseInsensitiveHashCodeProvider"
);
#[cfg(feature = "System+Collections+CaseInsensitiveHashCodeProvider")]
impl std::ops::Deref for crate::System::Collections::CaseInsensitiveHashCodeProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveHashCodeProvider")]
impl std::ops::DerefMut for crate::System::Collections::CaseInsensitiveHashCodeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveHashCodeProvider")]
impl crate::System::Collections::CaseInsensitiveHashCodeProvider {
    pub fn GetHashCode(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_CultureInfo1(
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CultureInfo1(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveHashCodeProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::CaseInsensitiveHashCodeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}