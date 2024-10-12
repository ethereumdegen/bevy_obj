use crate::{util::MeshConverter, ObjSettings};
use bevy_asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext};
use bevy_render::mesh::Mesh;
use bevy_utils::ConditionalSendFuture;

pub struct ObjLoader;

impl AssetLoader for ObjLoader {
    type Error = ObjError;
    type Settings = ObjSettings;
    type Asset = Mesh;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            load_obj_as_mesh(&bytes, settings)
        })
    }

    fn extensions(&self) -> &[&str] {
        crate::EXTENSIONS
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ObjError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid OBJ file: {0}")]
    InvalidFile(#[from] tobj::LoadError),
}

pub fn load_obj_as_mesh(mut bytes: &[u8], settings: &ObjSettings) -> Result<Mesh, ObjError> {
    let obj = tobj::load_obj_buf(&mut bytes, &tobj::GPU_LOAD_OPTIONS, |_| {
        Err(tobj::LoadError::GenericFailure)
    })?;

    Ok(MeshConverter::from(obj.0).convert(settings))
}