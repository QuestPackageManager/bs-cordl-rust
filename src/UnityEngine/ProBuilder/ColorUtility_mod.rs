#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::ColorUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ColorUtility";
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
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ColorUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ColorUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl crate::UnityEngine::ProBuilder::ColorUtility {
    pub fn CIELabFromRGB(
        R: f32,
        G: f32,
        B: f32,
        Scale: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::CIELabColor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CIELabFromRGB", (R, G, B, Scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeltaE(
        lhs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
        rhs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeltaE", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColor(
        vec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetColor", (vec))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColorName(
        InColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetColorName", (InColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn HSVtoRGB_HSVColor0(
        hsv: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::HSVColor>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HSVtoRGB", (hsv))?;
        Ok(__cordl_ret.into())
    }
    pub fn HSVtoRGB_f32_f32_f32_1(
        h: f32,
        s: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HSVtoRGB", (h, s, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn RGBToXYZ_Color0(
        col: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::XYZColor,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("RGBToXYZ", (col))?;
        Ok(__cordl_ret.into())
    }
    pub fn RGBToXYZ_f32_f32_f32_1(
        r: f32,
        g: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::XYZColor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RGBToXYZ", (r, g, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn RGBtoHSV(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::HSVColor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::HSVColor,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("RGBtoHSV", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn XYZToCIE_Lab(
        xyz: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::XYZColor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::CIELabColor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::CIELabColor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XYZToCIE_Lab", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn approx(lhs: f32, rhs: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("approx", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ColorUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ColorUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
