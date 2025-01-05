#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct CaseInsensitiveComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _compareInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CompareInfo,
    >,
}
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::CaseInsensitiveComparer =>
    "System.Collections"."CaseInsensitiveComparer"
);
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
impl std::ops::Deref for crate::System::Collections::CaseInsensitiveComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
impl std::ops::DerefMut for crate::System::Collections::CaseInsensitiveComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
impl crate::System::Collections::CaseInsensitiveComparer {
    pub fn Compare(
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_CultureInfo1(
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CultureInfo1(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::CaseInsensitiveComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
impl AsRef<crate::System::Collections::IComparer>
for crate::System::Collections::CaseInsensitiveComparer {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+CaseInsensitiveComparer")]
impl AsMut<crate::System::Collections::IComparer>
for crate::System::Collections::CaseInsensitiveComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
