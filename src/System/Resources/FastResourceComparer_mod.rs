#[cfg(feature = "System+Resources+FastResourceComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct FastResourceComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::FastResourceComparer =>
    "System.Resources"."FastResourceComparer"
);
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl std::ops::Deref for crate::System::Resources::FastResourceComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl std::ops::DerefMut for crate::System::Resources::FastResourceComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl crate::System::Resources::FastResourceComparer {
    pub fn CompareOrdinal_Il2CppArray_i32_Il2CppString1(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        aCharLength: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareOrdinal", (bytes, aCharLength, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareOrdinal_Il2CppObject_i32_Il2CppString2(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byteLen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareOrdinal", (a, byteLen, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareOrdinal_Il2CppString_Il2CppArray_i32_0(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bCharLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareOrdinal", (a, bytes, bCharLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppObject_Il2CppObject0(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppString_Il2CppString1(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject_Il2CppObject1(
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
    pub fn Equals_Il2CppString_Il2CppString0(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Il2CppObject0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Il2CppString1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashFunction(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashFunction", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::FastResourceComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::Resources::FastResourceComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::Resources::FastResourceComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::Resources::FastResourceComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::Resources::FastResourceComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsRef<crate::System::Collections::IComparer>
for crate::System::Resources::FastResourceComparer {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsMut<crate::System::Collections::IComparer>
for crate::System::Resources::FastResourceComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsRef<crate::System::Collections::IEqualityComparer>
for crate::System::Resources::FastResourceComparer {
    fn as_ref(&self) -> &crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FastResourceComparer")]
impl AsMut<crate::System::Collections::IEqualityComparer>
for crate::System::Resources::FastResourceComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
