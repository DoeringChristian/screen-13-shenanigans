
use mipmap::{image_info_new_2d_mipmap, fill_mipmaps};
use screen_13::prelude_arc::*;
use screen_13_imgui::*;

mod presenter;
mod mipmap;

fn main() {
    //pretty_env_logger::init();
    let screen_13 = EventLoop::new()
        .window(|window_builder|{
            window_builder.with_transparent(true)
        })
        .debug(false)
        .build()
        .unwrap();
    let mut cache = HashPool::new(&screen_13.device);

    let mut imgui = ImGui::new(&screen_13.device);

    let presenter = presenter::Presenter::new(&screen_13.device);

    let rppl = screen_13.new_graphic_pipeline(
        GraphicPipelineInfo::new(),
        [
            Shader::new_vertex(inline_spirv::include_spirv!("src/shaders/quad_vert.glsl", vert, vulkan1_2).as_slice()),
            Shader::new_fragment(inline_spirv::include_spirv!("src/shaders/red.glsl", frag, vulkan1_2).as_slice()),
        ]
    );

    let mut graph = RenderGraph::new();

    let tmp_img = graph.bind_node(Image::create(&screen_13.device,
                image_info_new_2d_mipmap(vk::Format::R8G8B8A8_UNORM, 800, 600, vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::COLOR_ATTACHMENT)
    ).unwrap());


    graph.begin_pass("Red render pass")
        .bind_pipeline(&rppl)
        .clear_color(0)
        .store_color(0, tmp_img)
        .record_subpass(move |subpass|{
            subpass.draw(6, 1, 0, 0);
        });

    fill_mipmaps(&mut graph, tmp_img);

    let mut img = Some(graph.unbind_node(tmp_img));

    graph.resolve().submit(&mut cache).unwrap();

    screen_13.run(|mut frame|{

        let image = frame.render_graph.bind_node(img.take().unwrap());

        frame.render_graph.clear_color_image_value(frame.swapchain_image, [0., 0., 0., 0.]);

        //presenter.present(image, &mut frame);

        let gui_image = imgui.draw_frame(&mut frame, |ui|{
            ui.button("Test");
        });
        presenter.present(gui_image, &mut frame);

        img = Some(frame.render_graph.unbind_node(image));
    }).unwrap();
}
