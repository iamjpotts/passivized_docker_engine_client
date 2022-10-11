
/// See https://docs.docker.com/engine/api/v1.41/#tag/Image/operation/ImageCreate
#[derive(Clone, Default)]
pub struct CreateImageRequest {
    pub from_image: Option<String>,
    pub from_src: Option<String>,
    pub repo: Option<String>,
    pub tag: Option<String>,
    pub message: Option<String>,
    pub platform: Option<String>
}

impl CreateImageRequest {

    // "from_image" would violate Rust naming conventions.
    pub fn image<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.from_image = Some(v.into());
        self
    }

    // "from_src" would violate Rust naming conventions.
    pub fn src<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.from_src = Some(v.into());
        self
    }

    pub fn repo<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.repo = Some(v.into());
        self
    }

    pub fn tag<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.tag = Some(v.into());
        self
    }

    pub fn message<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.message = Some(v.into());
        self
    }

    pub fn platform<V>(mut self, v: V) -> Self
        where V: Into<String>
    {
        self.platform = Some(v.into());
        self
    }

}
