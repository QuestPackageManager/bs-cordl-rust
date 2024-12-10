#[cfg(feature = "System+StringComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct StringComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+StringComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::StringComparer => "System"
    ."StringComparer"
);
#[cfg(feature = "System+StringComparer")]
impl std::ops::Deref for crate::System::StringComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+StringComparer")]
impl std::ops::DerefMut for crate::System::StringComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+StringComparer")]
impl crate::System::StringComparer {
    pub fn Compare_Il2CppObject_Il2CppObject0(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppString_Il2CppString1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject_Il2CppObject0(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppString_Il2CppString1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Il2CppString1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
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
#[cfg(feature = "System+StringComparer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::StringComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::StringComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::StringComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::StringComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
> for crate::System::StringComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsRef<crate::System::Collections::IComparer> for crate::System::StringComparer {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsMut<crate::System::Collections::IComparer> for crate::System::StringComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsRef<crate::System::Collections::IEqualityComparer>
for crate::System::StringComparer {
    fn as_ref(&self) -> &crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+StringComparer")]
impl AsMut<crate::System::Collections::IEqualityComparer>
for crate::System::StringComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
