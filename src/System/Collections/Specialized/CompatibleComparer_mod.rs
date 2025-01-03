#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct CompatibleComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _comparer: *mut crate::System::Collections::IComparer,
    pub _hcp: *mut crate::System::Collections::IHashCodeProvider,
}
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Specialized::CompatibleComparer =>
    "System.Collections.Specialized"."CompatibleComparer"
);
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
impl std::ops::Deref for crate::System::Collections::Specialized::CompatibleComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
impl std::ops::DerefMut for crate::System::Collections::Specialized::CompatibleComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
impl crate::System::Collections::Specialized::CompatibleComparer {
    pub fn Equals(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
        hashCodeProvider: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IHashCodeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (comparer, hashCodeProvider))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
        hashCodeProvider: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IHashCodeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (comparer, hashCodeProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Comparer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IComparer,
        > = __cordl_object.invoke("get_Comparer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultComparer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IComparer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultComparer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultHashCodeProvider() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IHashCodeProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IHashCodeProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultHashCodeProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HashCodeProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IHashCodeProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IHashCodeProvider,
        > = __cordl_object.invoke("get_HashCodeProvider", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Specialized::CompatibleComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
impl AsRef<crate::System::Collections::IEqualityComparer>
for crate::System::Collections::Specialized::CompatibleComparer {
    fn as_ref(&self) -> &crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Specialized+CompatibleComparer")]
impl AsMut<crate::System::Collections::IEqualityComparer>
for crate::System::Collections::Specialized::CompatibleComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
