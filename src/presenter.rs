use archery::*;
use inline_spirv::*;
use screen_13::prelude_arc::*;

pub struct Presenter {
    rppl: SharedPointer<GraphicPipeline, ArcK>,
}
impl Presenter {
    pub fn new(device: &SharedPointer<Device, ArcK>) -> Self {
        let rppl = SharedPointer::new(
            GraphicPipeline::create(
                device,
                GraphicPipelineInfo::new(),
                [
                    Shader::new_vertex(
                        inline_spirv!(
                            r#"
                            #version 450

                            const vec2 UV[6] = {
                                vec2(0.0, 0.0),
                                vec2(0.0, 1.0),
                                vec2(1.0, 0.0),
                                vec2(1.0, 0.0),
                                vec2(0.0, 1.0),
                                vec2(1.0, 1.0),
                            };
                            const vec4 POS[6] = {
                                vec4(-1.0, -1.0, 0.0, 1.0),
                                vec4(-1.0, 1.0, 0.0, 1.0),
                                vec4(1.0, -1.0, 0.0, 1.0),
                                vec4(1.0, -1.0, 0.0, 1.0),
                                vec4(-1.0, 1.0, 0.0, 1.0),
                                vec4(1.0, 1.0, 0.0, 1.0),
                            };

                            layout(location = 0) out vec2 o_uv;

                            void main(){
                                o_uv = UV[gl_VertexIndex];
                                gl_Position = POS[gl_VertexIndex];
                            }
                            "#,
                            vert
                        )
                        .as_slice(),
                    ),
                    Shader::new_fragment(
                        inline_spirv!(
                            r#"
                            #version 450
                            layout(location = 0) out vec4 o_color;

                            layout(location = 0) in vec2 i_uv;

                            layout(set = 0, binding = 0) uniform sampler2D tex_s;

                            void main(){
                                o_color = textureLod(tex_s, i_uv, 0.);
                                //o_color = vec4(1., 0., 0., 1.);
                            }
                            "#,
                            frag
                        )
                        .as_slice(),
                    ),
                ],
            )
            .unwrap(),
        );

        Self { rppl }
    }

    pub fn present(
        &self,
        image: impl Into<AnyImageNode>,
        frame: &mut FrameContext,
    ) {
        let image = image.into();
        let width = frame.window.inner_size().width;
        let height = frame.window.inner_size().height;

        let mut render_graph = frame.render_graph
            .begin_pass("Present Pass")
            .bind_pipeline(&self.rppl)
            .read_descriptor((0, 0), image)
            /*
            .read_descriptor_as((0, 0), image, ImageViewInfo{
                array_layer_count: None,
                base_array_layer: 0,
                mip_level_count: Some(1),
                base_mip_level: 4,
                fmt: vk::Format::R8G8B8A8_UNORM,
                ty: ImageType::Texture2D,
                aspect_mask: vk::ImageAspectFlags::COLOR,
            })
            */
            //.clear_color(0)
            .store_color(0, frame.swapchain_image);
        render_graph.set_render_area(0, 0, width, height);
        render_graph.record_subpass(move |subpass| {
            //subpass.set_scissor(0, 0, width, height);
            subpass.set_viewport(0.0, 0.0, width as f32, height as f32, 0.0..1.);
            subpass.draw(6, 1, 0, 0);
        });
    }
}
