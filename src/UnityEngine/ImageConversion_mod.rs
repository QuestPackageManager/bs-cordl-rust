#[cfg(feature = "UnityEngine+ImageConversion")]
#[repr(C)]
#[derive(Debug)]
pub struct ImageConversion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ImageConversion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ImageConversion => "UnityEngine"
    ."ImageConversion"
);
#[cfg(feature = "UnityEngine+ImageConversion")]
impl std::ops::Deref for crate::UnityEngine::ImageConversion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ImageConversion")]
impl std::ops::DerefMut for crate::UnityEngine::ImageConversion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ImageConversion")]
impl crate::UnityEngine::ImageConversion {
    pub fn EncodeToEXR_Texture2D1(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("EncodeToEXR", (tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeToEXR_Texture2D_EXRFlags0(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        flags: crate::UnityEngine::Texture2D_EXRFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodeToEXR", (tex, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeToJPG_Texture2D1(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("EncodeToJPG", (tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeToJPG_i32_0(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        quality: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodeToJPG", (tex, quality))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeToPNG(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("EncodeToPNG", (tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeToTGA(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("EncodeToTGA", (tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadImage_Texture2D_Il2CppArray1(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadImage", (tex, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadImage__cordl_bool0(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        markNonReadable: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadImage", (tex, data, markNonReadable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ImageConversion")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ImageConversion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
