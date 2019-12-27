use crate::graphics::BaseContext2d;
use crate::BaseSystem;
use mopa::*;

pub trait BaseResource: mopa::Any {
    fn upcast_object(self) -> Box<BaseResource> where Self: Sized {
        Box::new(self)
    }
    fn downcast_object_ref(obj: &BaseResource) -> Option<&Self> where Self: Sized {
        obj.downcast_ref()
    }
}

mopafy!(BaseResource);

#[macro_export]
macro_rules! declare_supertype {
    ($type:ty, $supertype:ty) => {
        impl BaseResource for $type {
            fn upcast_object(self) -> Box<BaseResource> where Self: Sized {
                Box::<$supertype>::upcast_object(Box::new(self))
            }
            fn downcast_object_ref(obj: &BaseResource) -> Option<&Self> where Self: Sized {
                Box::<$supertype>::downcast_object_ref(obj)?.downcast_ref()
            }
        }
    }
}

/// Represent a resource
pub trait Resource {
    type Context2d: BaseContext2d;

    /// Dispatched when the resource is loaded on memory
    fn parse<T>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseSystem<Context2d = Self::Context2d>;

    /// Should return true if the resource is ready to use it
    fn is_loaded(&self) -> bool;
}


impl<G: Resource + ?Sized> Resource for Box<G> where Box<G>: BaseResource {
    type Context2d = G::Context2d;

    fn parse<T>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String> where
        T: BaseSystem<Context2d=Self::Context2d> {
        unimplemented!()
    }

    fn is_loaded(&self) -> bool {
        unimplemented!()
    }
}

/// Represent a resource that can be created from the trait
pub trait ResourceConstructor {
    /// Create a new resource
    fn new(file: &str) -> Self;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba,
    Red,
    R8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

pub trait BaseTexture: Resource
where
    Self: Sized,
{
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn from_size<T: BaseSystem<Context2d = Self::Context2d>>(
        app: &mut T,
        width: i32,
        height: i32,
    ) -> Result<Self, String>;
    fn from<T: BaseSystem<Context2d = Self::Context2d>>(
        app: &mut T,
        width: i32,
        height: i32,
        internal_format: TextureFormat,
        format: TextureFormat,
        min_filter: TextureFilter,
        mag_filter: TextureFilter,
    ) -> Result<Self, String>;
    fn format(&self) -> TextureFormat;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

pub trait BaseFont: Resource {
    type Kind;

    fn text_size<T>(app: &mut T, font: &Self::Kind, text: &str, size: f32) -> (f32, f32)
    where
        T: BaseSystem<Context2d = Self::Context2d>;

    fn text_size_ext<T>(
        app: &mut T,
        font: &Self::Kind,
        text: &str,
        size: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        max_width: Option<f32>,
    ) -> (f32, f32)
    where
        T: BaseSystem<Context2d = Self::Context2d>;
}
