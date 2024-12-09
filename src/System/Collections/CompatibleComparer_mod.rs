#[cfg(feature = "System+Collections+CompatibleComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct CompatibleComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _hcp: *mut crate::System::Collections::IHashCodeProvider,
    pub _comparer: *mut crate::System::Collections::IComparer,
}
#[cfg(feature = "System+Collections+CompatibleComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::CompatibleComparer =>
    "System.Collections"."CompatibleComparer"
);
#[cfg(feature = "System+Collections+CompatibleComparer")]
impl std::ops::Deref for crate::System::Collections::CompatibleComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+CompatibleComparer")]
impl std::ops::DerefMut for crate::System::Collections::CompatibleComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+CompatibleComparer")]
impl crate::System::Collections::CompatibleComparer {
    pub fn Compare(
        &mut self,
        a: *mut quest_hook::libil2cpp::Il2CppObject,
        b: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        a: *mut quest_hook::libil2cpp::Il2CppObject,
        b: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        hashCodeProvider: *mut crate::System::Collections::IHashCodeProvider,
        comparer: *mut crate::System::Collections::IComparer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashCodeProvider, comparer))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        hashCodeProvider: *mut crate::System::Collections::IHashCodeProvider,
        comparer: *mut crate::System::Collections::IComparer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashCodeProvider, comparer))?;
        Ok(__cordl_ret)
    }
    pub fn get_Comparer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IComparer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IComparer = __cordl_object
            .invoke("get_Comparer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HashCodeProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::IHashCodeProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IHashCodeProvider = __cordl_object
            .invoke("get_HashCodeProvider", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Collections+CompatibleComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::CompatibleComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
