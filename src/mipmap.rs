

use screen_13::prelude_arc::*;

pub fn image_info_new_2d_mipmap(fmt: vk::Format, width: u32, height: u32, usage: vk::ImageUsageFlags) -> ImageInfo{
    ImageInfo::new_2d(fmt, width, height, usage)
        .mip_level_count((width as f32).log2().ceil().max((height as f32).log2().ceil()) as u32)
        .build()
}

pub fn fill_mipmaps(graph: &mut RenderGraph, image: impl Into<AnyImageNode>){
    let image = image.into();
    let info: ImageInfo = graph.node_info(image);

    let mut width = info.width as i32;
    let mut height = info.height as i32;
    let mut i = 0;

    while width > 1 && height > 1{
        println!("MipMap: {}, {}", width, height);
        graph.blit_image_region(image, image, 
            &vk::ImageBlit{
                src_subresource: vk::ImageSubresourceLayers{
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: i,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                src_offsets: [
                    vk::Offset3D{
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                    vk::Offset3D{
                        x: width,
                        y: height,
                        z: 1,
                    },
                ],
                dst_subresource: vk::ImageSubresourceLayers{
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: i+1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                dst_offsets: [
                    vk::Offset3D{
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                    vk::Offset3D{
                        x: if width > 1 {width / 2} else {1},
                        y: if height > 1 {height / 2} else {1},
                        z: 1,
                    },
                ],
            }, vk::Filter::LINEAR);
        if width > 1 {width /= 2};
        if height > 1 {height /= 2};
        i += 1;
    }
}
